use serde::{Deserialize, Serialize};
use super::info::InfoDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    pub metadata: MetadataDto,
    pub info: InfoDto,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDto {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}
