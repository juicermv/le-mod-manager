use std::str::FromStr;
use anyhow::anyhow;
use ascii::AsciiString;
use crate::data::package::package_file_raw::PackageFileRaw;
use crate::data::package::PackageFileType;

pub struct PackageFile {
    pub name: String,
    pub file_type: PackageFileType,
    pub data: Vec<u8>,
}

impl PackageFile {
    pub(crate) fn to_raw(self) -> anyhow::Result<PackageFileRaw> {
        if self.name.len() > 20 {
            return Err(anyhow!("File name cannot exceed 20 ASCII characters!"));
        }

        let mut ascii_name = AsciiString::with_capacity(20);
        ascii_name += &AsciiString::from_str(&self.name)?;

        let mut byte_array: [u8; 20] = [0; 20];
        for (pos, byte) in ascii_name.as_bytes().iter().enumerate() {
            byte_array[pos] = *byte;
        }

        Ok(PackageFileRaw {
            f_type: self.file_type.into(),
            f_name: byte_array,
            f_length: (self.data.len() as u64).to_be_bytes(),
            f_content: self.data,
        })
    }
}
