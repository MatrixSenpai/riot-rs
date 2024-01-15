use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    clash::{
        player::PlayerDto,
        team::TeamDto,
        tournament::TournamentDto
    },
};

impl RiotApiClient {
    /// Get a player in a clash tournament by summoner id
    pub async fn players_by_summoner(
        &self,
        region_routing: Option<RegionRouting>,
        summoner_id: String,
    ) -> ApiResult<PlayerDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/clash/v1/players/by-summoner/{summoner_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }

    /// Get a team in a clash tournament by the team id
    pub async fn team_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        team_id: String,
    ) -> ApiResult<TeamDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/clash/v1/teams/{team_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }

    /// Get a list of all upcoming tournaments
    pub async fn tournaments(
        &self,
        region_routing: Option<RegionRouting>,
    ) -> ApiResult<Vec<TournamentDto>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/clash/v1/tournaments".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }

    /// Get a tournament by a team id
    pub async fn tournaments_by_team(
        &self,
        region_routing: Option<RegionRouting>,
        team_id: String,
    ) -> ApiResult<TournamentDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/clash/v1/tournaments/by-team/{team_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }

    /// Get a tournament by id
    pub async fn tournament_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_id: String,
    ) -> ApiResult<TournamentDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/clash/v1/tournaments/{tournament_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }
}
