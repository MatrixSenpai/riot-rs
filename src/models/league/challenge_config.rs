use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeConfigInfoDto {
    pub id: u64,
    pub localized_names: HashMap<String, HashMap<String, String>>,
    pub state: ChallengeState,
    pub tracking: ChallengeTracking,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub leaderboard: bool,
    pub thresholds: HashMap<String, f64>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChallengeState {
    Disabled,
    Hidden,
    Enabled,
    Archived,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChallengeTracking {
    Lifetime,
    Season
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApexPlayerInfoDto {
    pub puuid: String,
    pub value: f64,
    pub position: u32,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChallengeLevel {
    None,
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
    Challenger,
    HighestNotLeaderboardOnly,
    Highest,
    Lowest
}
impl Display for ChallengeLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let result = match self {
           ChallengeLevel::None => "NONE",
           ChallengeLevel::Iron => "IRON",
           ChallengeLevel::Bronze => "BRONZE",
           ChallengeLevel::Silver => "SILVER",
           ChallengeLevel::Gold => "GOLD",
           ChallengeLevel::Platinum => "PLATINUM",
           ChallengeLevel::Diamond => "DIAMOND",
           ChallengeLevel::Master => "MASTER",
           ChallengeLevel::Grandmaster => "GRANDMASTER",
           ChallengeLevel::Challenger => "CHALLENGER",
           ChallengeLevel::HighestNotLeaderboardOnly => "HIGHEST_NOT_LEADERBOARD_ONLY",
           ChallengeLevel::Highest => "HIGHEST",
           ChallengeLevel::Lowest => "LOWEST",
       };

        write!(f, "{result}")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoDto {
    pub challenges: Vec<ChallengeInfo>,
    pub preferences: PlayerClientPreferences,
    pub total_points: ChallengePoints,
    pub category_points: HashMap<String, ChallengeInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeInfo {
    pub challenge_id: u64,
    pub percentile: f64,
    pub level: ChallengeLevel,
    pub value: f64,
    pub achieved_time: i64,
    pub position: i64,
    pub players_in_level: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClientPreferences {
    pub banner_accent: String,
    pub title: String,
    pub challenge_ids: Vec<u64>,
    pub crest_border: String,
    pub prestige_crest_border_level: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengePoints {
    pub level: ChallengeLevel,
    pub current: i64,
    pub max: i64,
    pub percentile: f64,
}