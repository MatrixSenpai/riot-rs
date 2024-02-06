use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerFlag {
    pub earned_date_iso8601: String,
    pub item_id: i32,
    pub level: i64,
    pub season_id: i64,
    pub theme: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerFrame {
    pub level: i64,
}