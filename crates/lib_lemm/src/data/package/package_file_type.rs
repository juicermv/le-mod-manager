use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
pub enum PackageFileType {
    TEXTURE,
    CONFIG,
    INI,
    PKG,
}

impl PackageFileType {
    pub fn from(byte: u8) -> anyhow::Result<Self> {
        match byte {
            100u8 => Ok(Self::TEXTURE),
            200u8 => Ok(Self::CONFIG),
            101u8 => Ok(Self::INI),
            202u8 => Ok(Self::PKG),
            _ => Err(anyhow!("Invalid byte!")),
        }
    }
}

impl Into<u8> for PackageFileType {
    fn into(self) -> u8 {
        match self {
            PackageFileType::TEXTURE => 100u8,
            PackageFileType::CONFIG => 200u8,
            PackageFileType::INI => 101u8,
            PackageFileType::PKG => 202u8,
        }
    }
}