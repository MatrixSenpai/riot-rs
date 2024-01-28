use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TournamentDto {
    pub id: u32,
    pub theme_id: u32,
    pub name_key: String,
    pub name_key_secondary: String,
    pub schedule: Vec<TournamentPhaseDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TournamentPhaseDto {
    pub id: u32,
    pub registration_time: u32,
    pub start_time: u32,
    pub cancelled: bool,
}