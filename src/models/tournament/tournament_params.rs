use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentRegistrationParamsV5 {
    pub provider_id: i32,
    pub name: Option<String>,
}