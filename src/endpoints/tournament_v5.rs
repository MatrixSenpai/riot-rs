use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    tournament::{
        TournamentCodeParamsV5,
        TournamentCodeV5Dto,
        LobbyEventV5DtoWrapper,
        ProviderRegistrationParamsV5,
        TournamentRegistrationParamsV5,
        TournamentCodeUpdateParamsV5,
        TournamentGamesV5,
    }
};

impl RiotApiClient {
    pub async fn tournament_codes(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_id: i64,
        count: Option<usize>,
        params: TournamentCodeParamsV5,
    ) -> ApiResult<Vec<String>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        let mut options = HashMap::new();

        options.insert("tournamentId".to_string(), tournament_id.to_string());
        if let Some(count) = count {
            options.insert("count".to_string(), count.to_string());
        }

        self.make_request(
            "/lol/tournament/v5/codes".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            options,
            Some(params),
        ).await
    }

    pub async fn tournament_codes_by_tournament_code(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
    ) -> ApiResult<TournamentCodeV5Dto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament/v5/codes/{tournament_code}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn tournament_codes_update_by_tournament_code(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
        update_params: TournamentCodeUpdateParamsV5,
    ) -> ApiResult<()> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament/v5/codes/{tournament_code}"),
            routing,
            reqwest::Method::PUT,
            HashMap::new(),
            HashMap::new(),
            Some(update_params),
        ).await
    }

    pub async fn tournament_game_by_code(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
    ) -> ApiResult<Vec<TournamentGamesV5>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament/v5/games/by-code/{tournament_code}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn tournament_get_lobby_events(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
    ) -> ApiResult<LobbyEventV5DtoWrapper> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament/v5/lobby-events/by-code/{tournament_code}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn tournament_register_provider(
        &self,
        region_routing: Option<RegionRouting>,
        registration_params: ProviderRegistrationParamsV5,
    ) -> ApiResult<usize> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/tournament/v5/providers".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            HashMap::new(),
            Some(registration_params),
        ).await
    }

    pub async fn tournament_register_tournament(
        &self,
        region_routing: Option<RegionRouting>,
        registration_params: TournamentRegistrationParamsV5,
    ) -> ApiResult<usize> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/tournament/v5/tournaments".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            HashMap::new(),
            Some(registration_params),
        ).await
    }
}

