use serde::Deserialize;
use super::{
    participant::ParticipantDto,
    team::TeamDto,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoDto {
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: Option<i64>,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub participants: Vec<ParticipantDto>,
    pub platform_id: String,
    pub queue_id: i32,
    pub teams: Vec<TeamDto>,
    pub tournament_code: Option<String>,
}

