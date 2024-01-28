use serde::{Deserialize, Serialize};
use super::league_entry::MiniSeriesDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueListDto {
    pub league_id: String,
    pub entries: Vec<LeagueItemDto>,
    pub tier: String,
    pub name: String,
    pub queue: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueItemDto {
    pub fresh_blood: bool,
    pub wins: u32,
    pub summoner_name: String,
    pub mini_series: MiniSeriesDto,
    pub inactive: bool,
    pub veteran: bool,
    pub hot_streak: bool,
    pub rank: String,
    pub league_points: u32,
    pub losses: u32,
    pub summoner_id: String,
}