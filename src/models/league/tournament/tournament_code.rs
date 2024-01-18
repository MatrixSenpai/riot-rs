use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentCodeV5Dto {
    pub code: String,
    pub spectators: Option<String>,
    pub lobby_name: String,
    pub meta_data: String,
    pub password: String,
    pub team_size: i32,
    pub provider_id: i32,
    pub pick_type: String,
    pub tournament_id: i32,
    pub id: i32,
    pub region: String,
    pub map: String,
    pub participants: Option<Vec<String>>,
}