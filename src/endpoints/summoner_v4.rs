use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::PlatformRouting,
    summoner::SummonerDto,
};

// TODO: needs tests written
impl RiotApiClient {
    pub async fn summoner_by_rso_puuid(
        &self,
        platform_routing: Option<PlatformRouting>,
        rso_puuid: String,
    ) -> ApiResult<SummonerDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/fulfillment/v1/summoners/by-puuid/{rso_puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn summoners_by_account(
        &self,
        platform_routing: Option<PlatformRouting>,
        account_id: String,
    ) -> ApiResult<SummonerDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/summoner/v4/summoners/by-account/{account_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    #[deprecated(since="0.1.0", note="Riot is deprecating this API because they are moving to RSO")]
    pub async fn summoners_by_name(
        &self,
        platform_routing: Option<PlatformRouting>,
        account_name: String,
    ) -> ApiResult<SummonerDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/summoner/v4/summoners/by-name/{account_name}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn summoners_by_puuid(
        &self,
        platform_routing: Option<PlatformRouting>,
        puuid: String,
    ) -> ApiResult<SummonerDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/summoner/v4/summoners/by-puuid/{puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn summoners_by_summoner_id(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
    ) -> ApiResult<SummonerDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/summoner/v4/summoners/{summoner_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}