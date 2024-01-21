use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::PlatformRouting,
    league::{
        current_game_info::CurrentGameInfoDto,
        featured_game_info::FeaturedGamesDto,
    },
};

impl RiotApiClient {
    pub async fn active_game_by_summoner(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
    ) -> ApiResult<CurrentGameInfoDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/spectator/v4/active-games/by-summoner/{summoner_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn featured_games(
        &self,
        platform_routing: Option<PlatformRouting>,
    ) -> ApiResult<FeaturedGamesDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            "/lol/spectator/v4/featured-games".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}