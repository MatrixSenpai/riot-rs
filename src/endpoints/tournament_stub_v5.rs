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
    }
};

impl RiotApiClient {
    pub async fn stub_codes(
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
            "/lol/tournament-stub/v5/codes".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            options,
            Some(params),
        ).await
    }

    pub async fn stub_codes_by_tournament_code(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
    ) -> ApiResult<TournamentCodeV5Dto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament-stub/v5/codes/{tournament_code}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn stub_get_lobby_events(
        &self,
        region_routing: Option<RegionRouting>,
        tournament_code: String,
    ) -> ApiResult<LobbyEventV5DtoWrapper> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/tournament-stub/v5/lobby-events/by-code/{tournament_code}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn stub_register_provider(
        &self,
        region_routing: Option<RegionRouting>,
        registration_params: ProviderRegistrationParamsV5,
    ) -> ApiResult<usize> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/tournament-stub/v5/providers".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            HashMap::new(),
            Some(registration_params),
        ).await
    }

    pub async fn stub_register_tournament(
        &self,
        region_routing: Option<RegionRouting>,
        registration_params: TournamentRegistrationParamsV5,
    ) -> ApiResult<usize> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/tournament-stub/v5/tournaments".to_string(),
            routing,
            reqwest::Method::POST,
            HashMap::new(),
            HashMap::new(),
            Some(registration_params),
        ).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_create_codes() {
        let (client, test_vars) = crate::tests::setup();

        let codes = client.stub_codes(
            None, test_vars.test_tournament_id.into(), Some(5), test_vars.test_tournament_code_params
        ).await.unwrap();

       assert_eq!(codes.len(), 5);
    }

    #[tokio::test]
    async fn test_get_tournament_code() {
        let (client, test_vars) = crate::tests::setup();

        let details = client.stub_codes_by_tournament_code(None, test_vars.test_tournament_code).await.unwrap();

        assert_eq!(details.id, 0);
        assert_eq!(details.provider_id, test_vars.test_tournament_provider_id);
        assert_eq!(details.tournament_id, test_vars.test_tournament_id);
        assert_eq!(details.lobby_name, test_vars.test_tournament_lobby);
    }

    #[tokio::test]
    async fn test_get_lobby_events() {
        let (client, test_vars) = crate::tests::setup();

        let events = client.stub_get_lobby_events(None, test_vars.test_tournament_code).await.unwrap();
        let test_event = events.event_list.first().unwrap();

        assert_eq!(test_event.event_type, "PracticeGameCreatedEvent".to_string());
    }

    #[tokio::test]
    async fn test_create_provider() {
        let (client, test_vars) = crate::tests::setup();

        let provider = client.stub_register_provider(None, test_vars.test_tournament_provider_params).await.unwrap();

        assert_eq!(provider, test_vars.test_tournament_provider_id as usize);
    }

    #[tokio::test]
    async fn test_register_tournament() {
        let (client, test_vars) = crate::tests::setup();

        let tournament = client.stub_register_tournament(None, test_vars.test_tournament_registration_params).await.unwrap();

        assert_eq!(tournament, test_vars.test_tournament_id as usize);
    }
}