use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigStatus {
    pub readiness: ClientConfigReadyStatus,
    pub update_id: i64,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum ClientConfigReadyStatus {
    NotReady,
    Ready,
    Disabled,
}