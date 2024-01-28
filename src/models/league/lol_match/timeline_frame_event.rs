use serde::{Deserialize, Serialize};
use super::timeline_info::MatchTimelinePosition;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameEvent {
    pub real_timestamp: Option<i64>,
    pub timestamp: i32,
    #[serde(rename = "type")]
    pub event_type: String,
    pub item_id: Option<i32>,
    pub participant_id: Option<i32>,
    pub level_up_type: Option<String>,
    pub skill_slot: Option<i32>,
    pub creator_id: Option<i32>,
    pub ward_type: Option<String>,
    pub level: Option<i32>,
    pub assisting_participant_ids: Option<Vec<i32>>,
    pub bounty: Option<i32>,
    pub kill_streak_length: Option<i32>,
    pub killer_id: Option<i32>,
    pub position: Option<MatchTimelinePosition>,
    pub victim_damage_dealt: Option<Vec<MatchTimelineInfoFrameEventDamage>>,
    pub victim_damage_received: Option<Vec<MatchTimelineInfoFrameEventDamage>>,
    pub victim_id: Option<i32>,
    pub kill_type: Option<String>,
    pub lane_type: Option<String>,
    pub team_id: Option<i32>,
    pub multi_kill_length: Option<i32>,
    pub killer_team_id: Option<i32>,
    pub monster_type: Option<String>,
    pub monster_sub_type: Option<String>,
    pub building_type: Option<String>,
    pub tower_type: Option<String>,
    pub after_id: Option<i32>,
    pub before_id: Option<i32>,
    pub gold_gain: Option<i32>,
    pub game_id: Option<i64>,
    pub winning_team: Option<i32>,
    pub transform_type: Option<String>,
    pub name: Option<String>,
    pub shutdown_bounty: Option<i32>,
    pub actual_start_time: Option<i64>
}

// TODO: Revisit this, because there may be unknown values...
// #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum MatchEventType {
//     AscendedEvent,
//     BuildingKill,
//     CapturePoint,
//     ChampionKill,
//     ChampionSpecialKill,
//     ChampionTransform,
//     DragonSoulGiven,
//     EliteMonsterKill,
//     GameEnd,
//     ItemDestroyed,
//     ItemPurchased,
//     ItemSold,
//     ItemUndo,
//     LevelUp,
//     PauseEnd,
//     PauseStart,
//     SkillLevelUp,
//     TurretPlateDestroyed,
//     WardKill,
//     WardPlaced,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameEventDamage {
    pub basic: bool,
    pub magic_damage: i32,
    pub name: String,
    pub participant_id: i32,
    pub physical_damage: i32,
    pub spell_name: String,
    pub spell_slot: i32,
    pub true_damage: i32,
    #[serde(rename = "type")]
    pub damage_type: String
}
