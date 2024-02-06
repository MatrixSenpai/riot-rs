use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AntiAddictionState {
    pub anti_addiction_token: String,
    pub localization_key: String,
    pub policy_type: AntiAddictionStatePolicy,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AntiAddictionStatePolicy {
    AntiAddictionWarning,
    AntiAddictionShutdown,
    AntiAddictionHeartbeat,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AntiAddictionToken {
    pub anti_addiction_token: String,
}