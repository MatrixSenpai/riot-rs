use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDto {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: i64,
}