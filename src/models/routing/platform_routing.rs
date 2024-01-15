use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::models::client::{
    ConversionError, configuration::Routable
};

#[derive(Debug, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PlatformRouting {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    NA1,
    OC1,
    TR1,
    RU,
    PH2,
    SG2,
    TH2,
    TW2,
    VN2,
}

impl Routable for PlatformRouting {
    fn base_url(&self) -> String {
        format!("https://{}.api.riotgames.com", self)
    }
}

impl TryFrom<&str> for PlatformRouting {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "br1"  => Ok(Self::BR1),
            "eun1" => Ok(Self::EUN1),
            "euw1" => Ok(Self::EUW1),
            "jp1"  => Ok(Self::JP1),
            "kr"   => Ok(Self::KR),
            "la1"  => Ok(Self::LA1),
            "la2"  => Ok(Self::LA2),
            "na1"  => Ok(Self::NA1),
            "oc1"  => Ok(Self::OC1),
            "tr1"  => Ok(Self::TR1),
            "ru"   => Ok(Self::RU),
            "ph2"  => Ok(Self::PH2),
            "sg2"  => Ok(Self::SG2),
            "th2"  => Ok(Self::TH2),
            "tw2"  => Ok(Self::TW2),
            "vn2"  => Ok(Self::VN2),

            _ => Err(Self::Error::InvalidStringError),
        }
    }
}
impl TryFrom<String> for PlatformRouting {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl<'l> Into<&'l str> for PlatformRouting {
    fn into(self) -> &'l str {
        match self {
            Self::BR1  => "BR1",
            Self::EUN1 => "EUN1",
            Self::EUW1 => "EUW1",
            Self::JP1  => "JP1",
            Self::KR   => "KR",
            Self::LA1  => "LA1",
            Self::LA2  => "LA2",
            Self::NA1  => "NA1",
            Self::OC1  => "OC1",
            Self::TR1  => "TR1",
            Self::RU   => "RU",
            Self::PH2  => "PH2",
            Self::SG2  => "SG2",
            Self::TH2  => "TH2",
            Self::TW2  => "TW2",
            Self::VN2  => "VN2",
        }
    }
}
impl Into<String> for PlatformRouting {
    fn into(self) -> String {
        match self {
            Self::BR1  => "BR1",
            Self::EUN1 => "EUN1",
            Self::EUW1 => "EUW1",
            Self::JP1  => "JP1",
            Self::KR   => "KR",
            Self::LA1  => "LA1",
            Self::LA2  => "LA2",
            Self::NA1  => "NA1",
            Self::OC1  => "OC1",
            Self::TR1  => "TR1",
            Self::RU   => "RU",
            Self::PH2  => "PH2",
            Self::SG2  => "SG2",
            Self::TH2  => "TH2",
            Self::TW2  => "TW2",
            Self::VN2  => "VN2",
        }.to_string()
    }
}

impl Display for PlatformRouting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}