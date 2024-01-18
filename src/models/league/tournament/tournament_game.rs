use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentGamesV5 {
    pub winning_team: Vec<TournamentTeamV5>,
    pub losing_team: Vec<TournamentTeamV5>,
    pub short_code: String,
    pub meta_data: String,
    pub game_id: i64,
    pub game_name: String,
    pub game_type: String,
    pub game_map: i32,
    pub game_mode: String,
    pub region: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentTeamV5 {
    pub puuid: String,
}