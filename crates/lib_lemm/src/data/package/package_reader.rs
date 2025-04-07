use crate::data::package::package_header::PackageHeader;
use crate::data::package::package_header_raw::PackageHeaderRaw;
use crate::data::package::package_member_header::PackageMemberHeader;
use crate::data::package::package_member_raw::PackageMemberRaw;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct PackageReader {
    path: PathBuf,
}

impl PackageReader {
    // Check that first three bytes are valid
    fn validate(&self) -> Result<()> {
        let mut file = File::open(&self.path)?;
        let mut three_bytes: [u8; 3] = [0; 3];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut three_bytes)?;
        drop(file);

        if three_bytes == [200u8, 20u8, 200u8] {
            Ok(())
        } else {
            Err(anyhow!("Invalid package file!"))
        }
    }

    pub fn read_header(&self) -> Result<PackageHeader> {
        match self.validate() {
            Err(e) => Err(e),

            Ok(()) => {
                let mut header_raw = PackageHeaderRaw::default();

                let mut file = File::open(&self.path)?;
                file.seek(SeekFrom::Start(3))?;
                file.read_exact(&mut header_raw.h_mod_name)?;
                file.read_exact(&mut header_raw.h_mod_author)?;
                file.read_exact(&mut header_raw.h_mod_version)?;
                drop(file);

                Ok(header_raw.into())
            }
        }
    }

    /**
        Reads member headers from package file and returns a list
        of the headers with their positions in the file.
    */
    pub fn read_member_headers(&self) -> Result<Vec<(PackageMemberHeader, u64)>> {
        match self.validate() {
            Err(e) => Err(e),
            Ok(()) => {
                let mut return_val: Vec<(PackageMemberHeader, u64)> = Vec::new();
                let mut file = File::open(&self.path)?;
                file.seek(SeekFrom::Start(28))?;

                let end = file.metadata()?.len();
                while file.stream_position()? < end {
                    let current_pos = file.stream_position()?;

                    let mut member = PackageMemberRaw::default();

                    let mut type_buff: [u8; 1] = [0; 1];
                    file.read_exact(&mut type_buff)?;
                    member.f_type = type_buff[0];

                    file.read_exact(&mut member.f_name)?;
                    file.read_exact(&mut member.f_length)?;
                    let content_size = u64::from_be_bytes(member.f_length);

                    // Skip over content
                    file.seek(SeekFrom::Current(content_size as i64))?;

                    return_val.push((member.into(), current_pos));
                }

                drop(file);

                Ok(return_val)
            }
        }
    }

    pub fn read_member_contents(
        &self,
        input: Vec<(PackageMemberHeader, u64)>,
    ) -> Result<Vec<(PackageMemberHeader, Vec<u8>)>> {
        match self.validate() {
            Err(e) => Err(e),
            Ok(()) => {
                let mut return_val: Vec<(PackageMemberHeader, Vec<u8>)> = Vec::new();
                let mut file = File::open(&self.path)?;

                for (header, pos) in input {
                    file.seek(SeekFrom::Start(pos + 73u64))?;

                    let mut content = vec![0; header.content_length as usize];
                    (&file)
                        .take(header.content_length)
                        .read_exact(&mut content)?;

                    return_val.push((header, content));
                }

                drop(file);

                Ok(return_val)
            }
        }
    }

    pub fn new(path: PathBuf) -> PackageReader {
        PackageReader { path }
    }
}
