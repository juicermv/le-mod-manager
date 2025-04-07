use ascii::AsciiChar;
use crate::data::package::package_header_raw::PackageHeaderRaw;
use crate::data::package::package_member_header::PackageMemberHeader;
use crate::data::package::package_member_raw::PackageMemberRaw;


#[derive(Debug, Clone, PartialEq)]
pub struct PackageHeader {
    pub mod_name: [AsciiChar; 10],
    pub mod_author: [AsciiChar; 10],
    pub mod_version: [AsciiChar; 5],
}

impl Into<PackageHeaderRaw> for PackageHeader {
    fn into(self) -> PackageHeaderRaw {
        PackageHeaderRaw {
            h_mod_name: self.mod_name.map(|char| char.as_byte()),
            h_mod_author: self.mod_author.map(|char| char.as_byte()),
            h_mod_version: self.mod_version.map(|char| char.as_byte()),
        }
    }
}

