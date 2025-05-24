use crate::data::package::package_header::PackageHeader;
use ascii::AsciiChar;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct PackageHeaderRaw {
    // HEADER
    pub(crate) h_mod_name: [u8; 32],   // Mod name in 10 ASCII chars.
    pub(crate) h_mod_author: [u8; 16], // Mod author, also 10 ASCII chars.
    pub(crate) h_mod_version: [u8; 5], // Mod version, 5 ASCII chars. Should be enough for 01.15 or 1.156
}

impl Into<PackageHeader> for PackageHeaderRaw {
    fn into(self) -> PackageHeader {
        PackageHeader {
            mod_name: self
                .h_mod_name
                .map(|byte| AsciiChar::from_ascii(byte).unwrap()),
            mod_author: self
                .h_mod_author
                .map(|byte| AsciiChar::from_ascii(byte).unwrap()),
            mod_version: self
                .h_mod_version
                .map(|byte| AsciiChar::from_ascii(byte).unwrap()),
        }
    }
}
