use serde::Deserialize;
use super::{
    timeline_frame_event::MatchTimelineInfoFrameEvent,
    timeline_participant::MatchTimelineInfoFrameParticipantFrames,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoDto {
    pub frame_interval: i32,
    pub frames: Vec<MatchTimelineInfoFrame>,
    pub game_id: i64,
    pub participants: Vec<MatchTimelineInfoParticipant>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrame {
    pub events: Vec<MatchTimelineInfoFrameEvent>,
    pub participant_frames: MatchTimelineInfoFrameParticipantFrames,
    pub timestamp: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelinePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoParticipant {
    pub participant_id: i32,
    pub puuid: String,
}