use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PackageMemberType {
    #[default]
    TEXTURE,
    ETexture,
    CFGPBR,
    CONFIG,
    INI,
    PKG,
}

impl PackageMemberType {
    pub fn from(byte: u8) -> anyhow::Result<Self> {
        match byte {
            10u8 => Ok(Self::TEXTURE),
            15u8 => Ok(Self::ETexture),
            20u8 => Ok(Self::CONFIG),
            30u8 => Ok(Self::INI),
            40u8 => Ok(Self::PKG),
            50u8 => Ok(Self::CFGPBR),
            _ => Err(anyhow!("Invalid file type byte!")),
        }
    }
}

impl Into<u8> for PackageMemberType {
    fn into(self) -> u8 {
        match self {
            PackageMemberType::TEXTURE => 10u8,
            PackageMemberType::ETexture => 15u8,
            PackageMemberType::CONFIG => 20u8,
            PackageMemberType::INI => 30u8,
            PackageMemberType::PKG => 40u8,
            PackageMemberType::CFGPBR => 50u8,
        }
    }
}