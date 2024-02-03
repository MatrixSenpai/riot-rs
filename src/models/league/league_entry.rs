use std::fmt::{Display, Formatter};
use chrono::format::format;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueEntryDto {
    pub league_id: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: u32,
    pub wins: u32,
    pub losses: u32,
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
    pub mini_series: Vec<MiniSeriesDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeriesDto {
    pub losses: u32,
    pub progress: String,
    pub target: u32,
    pub wins: u32,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum LeagueQueueEntry {
    RankedSolo5v5,
    RankedTFT,
    RankedFlexSR,
    RankedFlexTT,
}
impl Display for LeagueQueueEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let result = match self {
           LeagueQueueEntry::RankedSolo5v5 => "RANKED_SOLO_5x5",
           LeagueQueueEntry::RankedTFT => "RANKED_TFT",
           LeagueQueueEntry::RankedFlexSR => "RANKED_FLEX_SR",
           LeagueQueueEntry::RankedFlexTT => "RANKED_FLEX_TT",
       };

        write!(f, "{result}")
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum LeagueTierEntry {
    Challenger,
    Grandmaster,
    Master,
    Diamond,
    Emerald,
    Platinum,
    Gold,
    Silver,
    Bronze,
    Iron
}
impl Display for LeagueTierEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let result = match self {
           LeagueTierEntry::Challenger => "CHALLENGER",
           LeagueTierEntry::Grandmaster => "GRANDMASTER",
           LeagueTierEntry::Master => "MASTER",
           LeagueTierEntry::Diamond => "DIAMOND",
           LeagueTierEntry::Emerald => "EMERALD",
           LeagueTierEntry::Platinum => "PLATINUM",
           LeagueTierEntry::Gold => "GOLD",
           LeagueTierEntry::Silver => "SILVER",
           LeagueTierEntry::Bronze => "BRONZE",
           LeagueTierEntry::Iron => "IRON",
       };

        write!(f, "{result}")
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum LeagueDivisionEntry {
    One,
    Two,
    Three,
    Four,
}
impl Display for LeagueDivisionEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let result = match self {
           LeagueDivisionEntry::One => "I",
           LeagueDivisionEntry::Two => "II",
           LeagueDivisionEntry::Three => "III",
           LeagueDivisionEntry::Four => "IV",
       };

        write!(f, "{result}")
    }
}