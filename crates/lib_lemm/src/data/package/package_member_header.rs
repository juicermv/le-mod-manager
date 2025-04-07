use ascii::AsciiChar;
use crate::data::PackageMemberType;


#[derive(Debug, Clone, PartialEq)]
pub struct PackageMemberHeader {
    pub file_type: PackageMemberType,
    pub file_name: [AsciiChar; 64],
    pub content_length: u64
}