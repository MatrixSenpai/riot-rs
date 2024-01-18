use serde::Deserialize;
use super::current_game_info::{
    Observer, BannedChampion,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedGamesDto {
    pub game_list: Vec<FeaturedGameInfoDto>,
    pub client_refresh_interval: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedGameInfoDto {
    pub game_mode: String,
    pub game_length: i64,
    pub map_id: i64,
    pub game_type: String,
    pub banned_champions: Vec<BannedChampion>,
    pub game_id: i64,
    pub observers: Observer,
    pub game_queue_config_id: i64,
    pub game_start_time: Option<i64>,
    pub participants: Vec<Participant>,
    pub platform_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub bot: bool,
    pub spell2_id: i64,
    pub profile_icon_id: i64,
    pub summoner_name: String,
    pub champion_id: i64,
    pub team_id: i64,
    pub spell1_id: i64,
    pub puuid: Option<String>,
    pub summoner_id: Option<String>,
}