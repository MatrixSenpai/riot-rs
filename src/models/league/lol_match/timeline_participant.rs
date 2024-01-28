use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::timeline_info::MatchTimelinePosition;

pub type MatchTimelineInfoFrameParticipantFrames = HashMap<String, MatchTimelineInfoFrameParticipantFrame>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameParticipantFrame {
    pub champion_stats: ChampionStats,
    pub current_gold: i32,
    pub damage_stats: DamageStats,
    pub gold_per_second: i32,
    pub jungle_minions_killed: i32,
    pub level: i32,
    pub minions_killed: i32,
    pub participant_id: i32,
    pub position: MatchTimelinePosition,
    pub time_enemy_spent_controlled: i32,
    pub total_gold: i32,
    pub xp: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_haste: Option<i32>,
    pub ability_power: i32,
    pub armor: i32,
    pub armor_pen: i32,
    pub armor_pen_percent: i32,
    pub attack_damage: i32,
    pub attack_speed: i32,
    pub bonus_armor_pen_percent: i32,
    pub bonus_magic_pen_percent: i32,
    pub cc_reduction: i32,
    pub cooldown_reduction: i32,
    pub health: i32,
    pub health_max: i32,
    pub health_regen: i32,
    pub lifesteal: i32,
    pub magic_pen: i32,
    pub magic_pen_percent: i32,
    pub magic_resist: i32,
    pub movement_speed: i32,
    pub onmivamp: Option<i32>,
    pub physical_vamp: Option<i32>,
    pub power: i32,
    pub power_max: i32,
    pub power_regen: i32,
    pub spell_vamp: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    pub magic_damage_done: i32,
    pub magic_damage_done_to_champions: i32,
    pub magic_damage_taken: i32,
    pub physical_damage_done: i32,
    pub physical_damage_done_to_champions: i32,
    pub physical_damage_taken: i32,
    pub total_damage_done: i32,
    pub total_damage_done_to_champions: i32,
    pub total_damage_taken: i32,
    pub true_damage_done: i32,
    pub true_damage_done_to_champions: i32,
    pub true_damage_taken: i32,
}