use serde::Deserialize;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A single object containing champion mastery information for a player/champion combination
pub struct ChampionMasteryDto {
    /// Number of points needed to achieve next level. Zero if player has reached maximum champion level for this champion
    pub champion_points_until_next_level: i64,
    /// Is chest granted for this champion in the current season
    pub chest_granted: bool,
    /// Champion ID for this entry
    pub champion_id: i32,
    /// Last time this champion was played by this player. Timestamp in unix milliseconds.
    /// Use one of the following to get a timestamp:
    /// - [`Self::last_play_time_naive()`]
    /// - [`Self::last_play_time_utc()`]
    /// - [`Self::last_play_time_local()`]
    pub last_play_time: i64,
    /// Champion level for a player/champion combination
    pub champion_level: i32,
    /// Encrypted summoner id for this entry
    pub summoner_id: String,
    /// Total number of champion points for this player/champion combination. Used to determine champion level
    pub champion_points: i32,
    /// Number of points since the current level has been achieved
    pub champion_points_since_last_level: i64,
    /// The token earned for this champion at the current level. When the level is advanced the count resets to 0
    pub tokens_earned: i32,
    pub puuid: Option<String>,
}

impl ChampionMasteryDto {
    /// A naive datetime object of when the player last played this champion
    pub fn last_play_time_naive(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp_millis(self.last_play_time).unwrap()
    }
    /// A Utc-aligned datetime object of when the player last played this champion
    pub fn last_play_time_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.last_play_time).unwrap()
    }
    /// A datetime object aligned to a given timezone of when the player last played this champion
    pub fn last_play_time_local<T: TimeZone>(&self, time_zone: T) -> DateTime<T> {
        time_zone.timestamp_millis_opt(self.last_play_time).unwrap()
    }
}

impl Eq for ChampionMasteryDto {}
impl PartialEq for ChampionMasteryDto {
    fn eq(&self, other: &Self) -> bool {
       self.champion_id == other.champion_id
       && self.summoner_id == other.summoner_id
    }
}