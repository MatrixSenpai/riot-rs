use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveShardDto {
    pub puuid: String,
    pub game: String,
    pub active_shard: String,
}

impl Eq for ActiveShardDto {}
impl PartialEq for ActiveShardDto {
    fn eq(&self, other: &Self) -> bool {
        self.puuid == other.puuid
    }
}

impl Into<String> for ActiveShardDto {
    fn into(self) -> String {
        self.puuid
    }
}

impl Display for ActiveShardDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} ({}) - {}",
            &self.puuid, &self.game, &self.active_shard
        )
    }
}

pub enum ActiveShardGame {
    LoR,
    Valorant,
}
impl Into<String> for ActiveShardGame {
    fn into(self) -> String {
        match self {
            Self::LoR => "lor",
            Self::Valorant => "val",
        }.to_string()
    }
}