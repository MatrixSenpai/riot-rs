use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyEventV5DtoWrapper {
    pub event_list: Vec<LobbyEventV5Dto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyEventV5Dto {
    pub timestamp: String,
    pub event_type: String,
    pub puuid: Option<String>,
}