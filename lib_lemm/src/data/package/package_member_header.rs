use crate::data::PackageMemberType;
use ascii::AsciiChar;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageMemberHeader {
    pub file_type: PackageMemberType,
    pub file_name: [AsciiChar; 64],
    pub content_length: u64,
}

impl Default for PackageMemberHeader {
    fn default() -> Self {
        Self {
            file_type: PackageMemberType::TEXTURE,
            file_name: [AsciiChar::default(); 64],
            content_length: 0,
        }
    }
}
