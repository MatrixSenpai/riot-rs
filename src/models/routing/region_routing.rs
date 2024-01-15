use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::models::client::{
    ConversionError, configuration::Routable
};

#[derive(Debug, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum RegionRouting {
    AMERICAS,
    ASIA,
    EUROPE,
    SEA,
}

impl Routable for RegionRouting {
    fn base_url(&self) -> String {
        format!("https://{}.api.riotgames.com", self)
    }
}

impl TryFrom<&str> for RegionRouting {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "americas" => Ok(Self::AMERICAS),
            "asia"     => Ok(Self::ASIA),
            "europe"   => Ok(Self::EUROPE),
            "sea"      => Ok(Self::SEA),

            _ => Err(Self::Error::InvalidStringError),
        }
    }
}
impl TryFrom<String> for RegionRouting {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl<'l> Into<&'l str> for RegionRouting {
    fn into(self) -> &'l str {
        match self {
            Self::AMERICAS => "AMERICAS",
            Self::ASIA     => "ASIA",
            Self::EUROPE   => "EUROPE",
            Self::SEA      => "SEA",
        }
    }
}
impl Into<String> for RegionRouting {
    fn into(self) -> String {
        match self {
            Self::AMERICAS => "AMERICAS",
            Self::ASIA     => "ASIA",
            Self::EUROPE   => "EUROPE",
            Self::SEA      => "SEA",
        }.to_string()
    }
}

impl Display for RegionRouting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}