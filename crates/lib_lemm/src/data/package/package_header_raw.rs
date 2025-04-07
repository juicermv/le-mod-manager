#[derive(Default, Clone, Debug, PartialEq)]
pub struct PackageHeaderRaw {
    // HEADER
    pub(crate) h_mod_name: [u8; 10],   // Mod name in 10 ASCII chars.
    pub(crate) h_mod_author: [u8; 10], // Mod author, also 10 ASCII chars.
    pub(crate) h_mod_version: [u8; 5], // Mod version, 5 ASCII chars. Should be enough for 01.15 or 1.156
}
