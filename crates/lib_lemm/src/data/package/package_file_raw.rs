use ascii::AsciiString;

use crate::data::package::package_file::PackageFile;
use crate::data::package::package_file_type::PackageFileType;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct PackageFileRaw {
    pub(crate) f_type: u8, // Single byte to represent file type. Either texture, config, ini, or pkg
    pub(crate) f_name: [u8; 20], // File name, 20 ASCII chars.
    pub(crate) f_length: [u8; 8], // File content length in bytes, 8 bytes = unsigned 64 bit integer
    pub(crate) f_content: Vec<u8>, // File content, length determined by f_length
}

impl PackageFileRaw {
    pub fn into_normal(self) -> anyhow::Result<PackageFile> {
        let file_type = PackageFileType::from(self.f_type)?;
        let name = AsciiString::from_ascii(self.f_name)?.to_string();
        Ok(PackageFile {
            name,
            file_type,
            data: self.f_content,
        })
    }
}