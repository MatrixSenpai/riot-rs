use serde::Deserialize;
use super::player::PlayerDto;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct TeamDto {
    pub id: String,
    pub tournament_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub tier: u32,
    pub captain: String,
    pub abbreviation: String,
    pub players: Vec<PlayerDto>,
}