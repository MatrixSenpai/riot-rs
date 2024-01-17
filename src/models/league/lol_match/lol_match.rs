use serde::Deserialize;
use super::info::InfoDto;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    pub metadata: MetadataDto,
    pub info: InfoDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDto {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}
