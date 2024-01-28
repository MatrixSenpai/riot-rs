use serde::Serialize;
use super::tournament_code_params::{
    GamePickType, MapType, SpectatorType,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentCodeUpdateParamsV5 {
    pub allowed_participants: Option<Vec<String>>,
    pub pick_type: GamePickType,
    pub map_type: MapType,
    pub spectator_type: SpectatorType,
}