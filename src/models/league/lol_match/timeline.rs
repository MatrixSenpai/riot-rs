use serde::Deserialize;
use super::{
    MetadataDto,
    timeline_info::MatchTimelineInfoDto,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineDto {
    pub metadata: MetadataDto,
    pub info: MatchTimelineInfoDto
}