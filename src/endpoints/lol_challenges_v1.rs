use std::collections::HashMap;
use std::fmt::format;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    challenge_config::*
};

// TODO: Needs tests written
// TODO: Needs documentation written
impl RiotApiClient {
    pub async fn challenges_config(
        &self,
        region_routing: Option<RegionRouting>,
    ) -> ApiResult<Vec<ChallengeConfigInfoDto>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/challenges/v1/challenges/config".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn challenges_percentiles(
        &self,
        region_routing: Option<RegionRouting>,
    ) -> ApiResult<HashMap<String, HashMap<ChallengeLevel, f64>>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/challenges/v1/challenges/percentiles".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn challenges_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        challenge_id: u64,
    ) -> ApiResult<ChallengeConfigInfoDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/challenges/v1/challenges/{challenge_id}/config"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn challenge_leaderboards(
        &self,
        region_routing: Option<RegionRouting>,
        challenge_id: u64,
        level: ChallengeLevel,
        limit: Option<u32>,
    ) -> ApiResult<Vec<ApexPlayerInfoDto>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        let options = if let Some(limit) = limit {
            let mut temp = HashMap::new();
            temp.insert("limit".to_string(), limit.to_string());
            temp
        } else { HashMap::new() };

        self.make_request(
            format!("/lol/challenges/v1/challenges/{challenge_id}/leaderboards/by-level/{level}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            options,
            None::<()>,
        ).await
    }

    pub async fn challenges_percentile_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        challenge_id: u64,
    ) -> ApiResult<HashMap<ChallengeLevel, f64>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/challenges/v1/challenges/{challenge_id}/percentiles"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn challenges_player_data(
        &self,
        region_routing: Option<RegionRouting>,
        puuid: String,
    ) -> ApiResult<PlayerInfoDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/challenges/v1/player-data/{puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}