use serde::{Deserialize, Serialize};
use super::{
    MetadataDto,
    timeline_info::MatchTimelineInfoDto,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineDto {
    pub metadata: MetadataDto,
    pub info: MatchTimelineInfoDto
}