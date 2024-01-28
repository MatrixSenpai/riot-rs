use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Information on champion rotations, including for low-level players
pub struct ChampionInfoDto {
    /// The new player level limit
    pub max_new_player_level: i32,
    /// Champions available to new players
    pub free_champion_ids_for_new_players: Vec<i32>,
    /// Champions available to players over the new player limit
    pub free_champion_ids: Vec<i32>,
}