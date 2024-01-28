use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerDto {
    pub summoner_id: String,
    pub team_id: Option<String>,
    pub position: PlayerPosition,
    pub role: PlayerRole,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all="UPPERCASE")]
pub enum PlayerPosition {
    Unselected,
    Fill,
    Top,
    Jungle,
    Middle,
    Bottom,
    Utility,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all="UPPERCASE")]
pub enum PlayerRole {
    Captain,
    Member
}