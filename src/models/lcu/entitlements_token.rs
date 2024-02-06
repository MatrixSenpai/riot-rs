use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsToken {
    pub access_token: String,
    pub entitlements: Vec<String>,
    pub issuer: String,
    pub subject: String,
    pub token: String,
}