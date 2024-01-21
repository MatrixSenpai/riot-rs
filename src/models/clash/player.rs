use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct PlayerDto {
    pub summoner_id: String,
    pub team_id: Option<String>,
    pub position: PlayerPosition,
    pub role: PlayerRole,
}

#[derive(Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all="UPPERCASE")]
pub enum PlayerRole {
    Captain,
    Member
}