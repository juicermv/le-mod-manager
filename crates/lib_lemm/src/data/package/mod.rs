mod package_file;
mod package_file_raw;
mod package_file_type;
mod package_header_raw;
mod package_raw;

pub use package_file::PackageFile;
pub use package_file_type::PackageFileType;

use anyhow::anyhow;
use anyhow::Result;
use ascii::AsciiStr;
use ascii::AsciiString;
use ascii::IntoAsciiString;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::u64;

use crate::data::package::package_file_raw::PackageFileRaw;
use crate::data::package::package_header_raw::PackageHeaderRaw;
use crate::data::package::package_raw::PackageRaw;

#[derive(PartialEq, Debug, Clone)]
pub struct Package {
    data: PackageRaw,
    path: Option<PathBuf>,
}

impl Package {
    fn read_header(file_path: impl AsRef<Path>) -> Result<PackageHeaderRaw> {
        // Setup buffers
        let mut check_buff: [u8; 3] = [0; 3];
        let mut h_mod_name: [u8; 10] = [0; 10];
        let mut h_mod_author: [u8; 10] = [0; 10];
        let mut h_mod_version: [u8; 5] = [0; 5];

        let mut file = File::open(file_path)?;

        file.seek(SeekFrom::Start(0))?;

        // Check some bytes for validity. First three bytes of package should be 200, 20, 200.
        file.read_exact(&mut check_buff)?;
        if check_buff != [200u8, 20u8, 200u8] {
            return Err(anyhow!("Invalid package format!"));
        }
        // Read headers
        file.read_exact(&mut h_mod_name)?;
        file.read_exact(&mut h_mod_author)?;
        file.read_exact(&mut h_mod_version)?;

        Ok(PackageHeaderRaw {
            h_mod_name,
            h_mod_author,
            h_mod_version,
        })
    }

    fn read_file_metadata(file_path: impl AsRef<Path>) -> Result<Vec<PackageFileRaw>> {
        let mut files: Vec<PackageFileRaw> = vec![];
        let mut file = File::open(file_path)?;
        let end = file.metadata()?.len();

        file.seek(SeekFrom::Start(28))?;
        while file.stream_position()? < end {
            let mut f_type: [u8; 1] = [0; 1];
            let mut f_name: [u8; 20] = [0; 20];
            let mut f_length: [u8; 8] = [0; 8];

            file.read_exact(&mut f_type)?;
            file.read_exact(&mut f_name)?;
            file.read_exact(&mut f_length)?;

            let offset: u64 = u64::from_be_bytes(f_length);
            file.seek(SeekFrom::Current(offset as i64))?; // We will not read file contents right now

            files.push(PackageFileRaw {
                f_type: f_type[0],
                f_name,
                f_length,
                f_content: vec![],
            });
        }

        Ok(files)
    }

    fn read_file_from_disk(&mut self, target: PackageFile) -> Result<PackageFile> {
        let mut raw_target = target.to_raw()?;

        match &self.path {
            None => Err(anyhow!("PKG does not exist on disk!")),
            Some(path) => {
                let mut file = File::open(path)?;
                let end = file.metadata()?.len();

                file.seek(SeekFrom::Start(28))?;
                while file.stream_position()? < end {
                    let mut f_type: [u8; 1] = [0; 1];
                    let mut f_name: [u8; 20] = [0; 20];
                    let mut f_length: [u8; 8] = [0; 8];

                    file.read_exact(&mut f_type)?;
                    file.read_exact(&mut f_name)?;
                    file.read_exact(&mut f_length)?;

                    let offset: u64 = u64::from_be_bytes(f_length);
                    if raw_target.f_name == f_name && raw_target.f_type == f_type[0] {
                        raw_target.f_content = vec![0; offset as usize];
                        (&file).take(offset).read_exact(&mut raw_target.f_content)?;
                    } else {
                        file.seek(SeekFrom::Current(offset as i64))?;
                    }
                }

                raw_target.into_normal()
            }
        }
    }

    pub fn load_files(&mut self) -> Result<()> {
        match &self.path {
            None => Err(anyhow!("PKG does not exist on disk!")),
            Some(path) => {
                let mut files: Vec<PackageFileRaw> = vec![];
                let mut file = File::open(path)?;
                let end = file.metadata()?.len();

                let mut pos: u64 = 0;
                file.seek(SeekFrom::Start(28))?;
                while file.stream_position()? < end {
                    let mut f_type: [u8; 1] = [0; 1];
                    let mut f_name: [u8; 20] = [0; 20];
                    let mut f_length: [u8; 8] = [0; 8];

                    file.read_exact(&mut f_type)?;
                    file.read_exact(&mut f_name)?;
                    file.read_exact(&mut f_length)?;

                    let offset: u64 = u64::from_be_bytes(f_length);
                    let mut f_content: Vec<u8> = vec![0; offset as usize];
                    (&file).take(offset).read_exact(&mut f_content)?;

                    files.push(PackageFileRaw {
                        f_type: f_type[0],
                        f_name,
                        f_length,
                        f_content,
                    });
                }

                self.data.p_files = files;
                Ok(())
            }
        }
    }

    pub fn load_files_of_type(&mut self, file_type: &PackageFileType) -> Result<()> {
        match &self.path {
            None => Err(anyhow!("PKG does not exist on disk!")),
            Some(path) => {
                let mut files: Vec<PackageFileRaw> = vec![];
                let mut file = File::open(path)?;
                let end = file.metadata()?.len();

                file.seek(SeekFrom::Start(28))?;
                while file.stream_position()? < end {
                    let mut f_type: [u8; 1] = [0; 1];
                    let mut f_name: [u8; 20] = [0; 20];
                    let mut f_length: [u8; 8] = [0; 8];

                    file.read_exact(&mut f_type)?;
                    file.seek(SeekFrom::Current(1))?;
                    file.read_exact(&mut f_name)?;
                    file.seek(SeekFrom::Current(20))?;
                    file.read_exact(&mut f_length)?;
                    file.seek(SeekFrom::Current(8))?;

                    let offset: u64 = u64::from_be_bytes(f_length);
                    let mut f_content: Vec<u8> = vec![0; offset as usize];
                    let file_type_byte: u8 = file_type.clone().into();
                    if f_type[0] == file_type_byte {
                        (&file).take(offset).read_exact(&mut f_content)?;
                    }

                    file.seek(SeekFrom::Current(offset as i64))?;

                    files.push(PackageFileRaw {
                        f_type: f_type[0],
                        f_name,
                        f_length,
                        f_content,
                    });
                }

                self.data.p_files = files;
                Ok(())
            }
        }
    }

    pub fn write(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut file = File::create(&path)?;
        file.seek(SeekFrom::Start(0))?;
        file.write(&[200u8, 20u8, 200u8])?; // Write 3 first validation bytes

        // Write headers
        file.write(&self.data.p_header.h_mod_name)?;
        file.write(&self.data.p_header.h_mod_author)?;
        file.write(&self.data.p_header.h_mod_version)?;

        for pkg_file in &self.data.p_files {
            file.write(&[pkg_file.f_type])?;
            file.write(&pkg_file.f_name)?;
            file.write(&pkg_file.f_length)?;
            file.write(pkg_file.f_content.as_slice())?;
        }

        self.path = Some(path.as_ref().to_owned());

        Ok(())
    }

    pub fn from_disk(path: impl AsRef<Path>) -> Result<Self> {
        let headers = Self::read_header(&path)?;
        let files = Self::read_file_metadata(&path)?;

        Ok(Self {
            data: PackageRaw {
                p_header: headers,
                p_files: files,
            },
            path: Some(path.as_ref().to_owned()),
        })
    }

    pub fn new() -> Self {
        Self {
            data: PackageRaw::default(),
            path: None,
        }
    }

    pub fn set_mod_name(&mut self, name: &str) -> Result<()> {
        if name.len() > 10 {
            return Err(anyhow!("Mod name can only be 10 ASCII characters!"));
        };

        let mut ascii_string = AsciiString::with_capacity(10);
        ascii_string += &AsciiString::from_str(name)?;

        let mut byte_array: [u8; 10] = [0; 10];
        for (pos, byte) in ascii_string.as_bytes().iter().enumerate() {
            byte_array[pos] = *byte;
        }

        self.data.p_header.h_mod_name = byte_array;
        Ok(())
    }

    pub fn get_mod_name(&mut self) -> Result<String> {
        let ascii_string = AsciiString::from_ascii(self.data.p_header.h_mod_name)?;
        Ok(ascii_string.into())
    }

    pub fn set_mod_author(&mut self, author: &str) -> Result<()> {
        if author.len() > 10 {
            return Err(anyhow!("Mod author can only be 10 ASCII characters!"));
        };

        let mut ascii_string = AsciiString::with_capacity(10);
        ascii_string += &AsciiString::from_str(author)?;

        let mut byte_array: [u8; 10] = [0; 10];
        for (pos, byte) in ascii_string.as_bytes().iter().enumerate() {
            byte_array[pos] = *byte;
        }

        self.data.p_header.h_mod_author = byte_array;
        Ok(())
    }

    pub fn get_mod_author(&mut self) -> Result<String> {
        let ascii_string = AsciiString::from_ascii(self.data.p_header.h_mod_author)?;
        Ok(ascii_string.into())
    }

    pub fn set_mod_version(&mut self, version: &str) -> Result<()> {
        if version.len() > 4 {
            return Err(anyhow!("Mod version can only be 5 ASCII characters!"));
        };

        let mut ascii_string = AsciiString::with_capacity(10);
        ascii_string += &AsciiString::from_str(version)?;

        let mut byte_array: [u8; 5] = [0; 5];
        for (pos, byte) in ascii_string.as_bytes().iter().enumerate() {
            byte_array[pos] = *byte;
        }

        self.data.p_header.h_mod_version = byte_array;
        Ok(())
    }

    pub fn get_mod_version(&mut self) -> Result<String> {
        let ascii_string = AsciiString::from_ascii(self.data.p_header.h_mod_version)?;
        Ok(ascii_string.into())
    }

    pub fn add_file(&mut self, file: PackageFile) -> Result<()> {
        self.data.p_files.push(file.to_raw()?);
        Ok(())
    }

    pub fn clear_files(&mut self) {
        self.data.p_files.clear();
    }

    pub fn get_files(&self) -> Result<Vec<PackageFile>> {
        self.data
            .p_files
            .iter()
            .map(|val| val.clone().into_normal())
            .collect()
    }

    pub fn get_files_of_type(&self, file_type: PackageFileType) -> Result<Vec<PackageFile>> {
        let file_type_byte: u8 = file_type.clone().into();
        self.data
            .p_files
            .iter()
            .filter(|val| val.f_type == file_type_byte)
            .map(|val| val.clone().into_normal())
            .collect()
    }
}
