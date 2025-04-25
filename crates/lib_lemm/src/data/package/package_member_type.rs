use anyhow::anyhow;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash, EnumIter, Ord, PartialOrd)]
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
    
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "Engine Texture" => Some(PackageMemberType::ETexture),
            "Texture" => Some(PackageMemberType::TEXTURE),
            "ini" => Some(PackageMemberType::INI),
            "Config" => Some(PackageMemberType::CONFIG),
            "cfgpbr" => Some(PackageMemberType::CFGPBR),
            "Package" => Some(PackageMemberType::PKG),
            _ => None
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

impl Into<String> for PackageMemberType {
    fn into(self) -> String {
        match self {
            PackageMemberType::ETexture => String::from("Engine Texture"),
            PackageMemberType::TEXTURE => String::from("Texture"),
            PackageMemberType::CONFIG => String::from("Config"),
            PackageMemberType::CFGPBR => String::from("cfgpbr"),
            PackageMemberType::INI => String::from("ini"),
            PackageMemberType::PKG => String::from("Package"),
        }
    }
}