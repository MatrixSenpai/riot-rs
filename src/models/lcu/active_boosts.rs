use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveBoosts {
    pub first_win_of_the_day_start_time: String,
    pub ip_boost_end_date: String,
    pub ip_boost_per_win_count: i32,
    pub ip_loyalty_boost: i32,
    pub summoner_id: i64,
    pub xp_boost_end_date: String,
    pub xp_boost_per_win_count: i32,
    pub xp_loyalty_boost: i32,
}