use crate::data::package::package_header::PackageHeader;
use crate::data::package::package_header_raw::PackageHeaderRaw;
use crate::data::package::package_member_header::PackageMemberHeader;
use crate::data::package::package_member_raw::PackageMemberRaw;
use crate::data::package::package_reader::PackageReader;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};

pub struct PackageWriter {
    path: PathBuf,
}

impl PackageWriter {
    pub fn write(
        &self,
        header: &PackageHeader,
        members: &Vec<(PackageMemberHeader, Vec<u8>)>,
    ) -> Result<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(&[200u8, 20u8, 200u8])?;

        let header_raw: PackageHeaderRaw = (*header).clone().into();

        file.write_all(&header_raw.h_mod_name)?;
        file.write_all(&header_raw.h_mod_author)?;
        file.write_all(&header_raw.h_mod_version)?;

        for header_data in members {
            let member_raw = PackageMemberRaw::from((*header_data).clone());

            file.write_all(&[member_raw.f_type])?;
            file.write_all(&member_raw.f_name)?;
            file.write_all(&member_raw.f_length)?;
            file.write_all(&member_raw.f_content)?;
        }

        drop(file);

        Ok(())
    }

    pub fn write_header(&self, header: &PackageHeader) -> Result<()> {
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&self.path)?;
        file.write_all(&[200u8, 20u8, 200u8])?;

        let header_raw: PackageHeaderRaw = (*header).clone().into();

        file.write_all(&header_raw.h_mod_name)?;
        file.write_all(&header_raw.h_mod_author)?;
        file.write_all(&header_raw.h_mod_version)?;

        drop(file);

        Ok(())
    }

    // Appends members to package file and returns positions at which they were appended
    pub fn append(
        &self,
        members: &Vec<(PackageMemberHeader, Vec<u8>)>,
    ) -> Result<Vec<(PackageMemberHeader, u64)>> {
        let mut reader = PackageReader::new(self.path.clone());
        match reader.read_header() {
            // Check that we are appending to a valid package file
            Err(e) => Err(e),
            Ok(_) => {
                let mut positions: Vec<(PackageMemberHeader, u64)> =
                    Vec::with_capacity(members.len());
                let mut file = File::options().append(true).open(&self.path)?;
                for header_data in members {
                    let member_raw = PackageMemberRaw::from((*header_data).clone());
                    positions.push((header_data.0.clone(), file.stream_position()?));

                    file.write_all(&[member_raw.f_type])?;
                    file.write_all(&member_raw.f_name)?;
                    file.write_all(&member_raw.f_length)?;
                    file.write_all(&member_raw.f_content)?;
                }

                Ok(positions)
            }
        }
    }

    pub fn new(path: PathBuf) -> Self {
        PackageWriter { path }
    }
}
