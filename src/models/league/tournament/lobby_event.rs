use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyEventV5DtoWrapper {
    pub event_list: Vec<LobbyEventV5Dto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyEventV5Dto {
    pub timestamp: String,
    pub event_type: String,
    pub puuid: Option<String>,
}