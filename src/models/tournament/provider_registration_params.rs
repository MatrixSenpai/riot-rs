use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRegistrationParamsV5 {
    pub region: String,
    pub url: String,
}