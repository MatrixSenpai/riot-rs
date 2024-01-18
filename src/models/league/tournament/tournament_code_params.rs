use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentCodeParamsV5 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_participants: Option<Vec<String>>,
    pub metadata: Option<String>,
    pub team_size: i32,
    pub pick_type: GamePickType,
    pub map_type: MapType,
    pub spectator_type: SpectatorType,
    pub enough_players: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GamePickType {
    BlindPick,
    DraftMode,
    AllRandom,
    TournamentDraft,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MapType {
    SummonersRift,
    HowlingAbyss
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SpectatorType {
    None,
    LobbyOnly,
    All,
}