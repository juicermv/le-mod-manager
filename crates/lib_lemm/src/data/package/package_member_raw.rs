use crate::data::package::package_header_raw::PackageHeaderRaw;
use crate::data::package::package_member_header::PackageMemberHeader;
use crate::data::package::package_member_type::PackageMemberType;
use ascii::{AsciiChar, AsciiString};

#[derive(Clone, Debug, PartialEq)]
pub struct PackageMemberRaw {
    pub(crate) f_type: u8, // Single byte to represent file type. Either texture, config, ini, or pkg
    pub(crate) f_name: [u8; 64], // File name, 20 ASCII chars.
    pub(crate) f_length: [u8; 8], // File content length in bytes, 8 bytes = unsigned 64 bit integer
    pub(crate) f_content: Vec<u8>, // File content, length determined by f_length
}

impl Default for PackageMemberRaw {
    fn default() -> Self {
        Self {
            f_type: 0,
            f_name: [0; 64],
            f_length: [0; 8],
            f_content: Vec::new(),
        }
    }
}

impl From<(PackageMemberHeader, Vec<u8>)> for PackageMemberRaw {
    fn from((header, data): (PackageMemberHeader, Vec<u8>)) -> Self {
        Self {
            f_type: header.file_type.into(),
            f_name: header.file_name.map(|char| char.as_byte()),
            f_length: (data.len() as u64).to_be_bytes(),
            f_content: data,
        }
    }
}

impl Into<PackageMemberHeader> for PackageMemberRaw {
    fn into(self) -> PackageMemberHeader {
        PackageMemberHeader {
            file_type: PackageMemberType::from(self.f_type).unwrap(),
            file_name: self.f_name.map(|byte| AsciiChar::from_ascii(byte).unwrap()),
            content_length: u64::from_be_bytes(self.f_length),
        }
    }
}
