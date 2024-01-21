use std::collections::HashMap;
use crate::{
    prelude::RiotApiClient,
    models::{
        ApiResult,
        league::champion_info::ChampionInfoDto,
        routing::PlatformRouting,
    }
};

impl RiotApiClient {
    pub async fn champion_rotations(
        &self,
        platform: Option<PlatformRouting>,
    ) -> ApiResult<ChampionInfoDto> {
        let routing = platform.unwrap_or(self.configuration.default_platform);

        self.make_request(
            "/lol/platform/v3/champion-rotations".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}