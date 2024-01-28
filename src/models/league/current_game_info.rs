use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfoDto {
    pub game_id: i64,
    pub game_type: String,
    pub game_start_time: i64,
    pub map_id: i64,
    pub game_length: i64,
    pub platform_id: String,
    pub game_mode: String,
    pub banned_champions: Vec<BannedChampion>,
    pub game_queue_config_id: Option<i64>,
    pub observers: Observer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    pub pick_turn: i32,
    pub champion_id: i64,
    pub team_id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Observer {
    pub encryption_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameParticipant {
    pub champion_id: i64,
    pub perks: Perks,
    pub profile_icon_id: i64,
    pub bot: bool,
    pub team_id: i64,
    pub summoner_name: String,
    pub summoner_id: String,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub game_customization_objects: Vec<GameCustomizationObject>,
    pub puuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub perk_ids: Vec<i64>,
    pub perks_type: i64,
    pub perk_sub_style: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCustomizationObject {
    pub category: String,
    pub content: String,
}