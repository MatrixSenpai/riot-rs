use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountVerificationAuthenticateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountVerificationSendTokenRequest {
    pub device: String,
    pub locale: Option<String>,
    pub mediator: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountVerificationAuthenticateResponse {
    pub message: String,
    pub status: i32,
    pub sms_token_expiration_in_sec: Option<i32>,
    pub success: bool,
}