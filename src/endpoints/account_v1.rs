use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    account::AccountDto,
    active_shard::*,
    routing::{RegionRouting, PlatformRouting},
};

impl RiotApiClient {
    /// Get account by encrypted PUUID
    pub async fn account_by_puuid(
        &self,
        region_routing: Option<RegionRouting>,
        puuid: String,
    ) -> ApiResult<AccountDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/riot/account/v1/accounts/by-puuid/{puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get an account by Riot ID. NOTE: Tagline can be set to empty and the specified or default platform will be used
    pub async fn account_by_riot_id(
        &self,
        region_routing: Option<RegionRouting>,
        game_name: String,
        tag_line: String,
    ) -> ApiResult<AccountDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/riot/account/v1/accounts/by-riot-id/{game_name}/{tag_line}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn account_by_game(
        &self,
        region_routing: Option<RegionRouting>,
        game: ActiveShardGame,
        puuid: String,
    ) -> ApiResult<ActiveShardDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/riot/account/v1/active-shards/by-game/{}/by-puuid/{puuid}", Into::<String>::into(game)),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::routing::PlatformRouting;

    #[tokio::test]
    async fn test_account_by_puuid() {
        let (client, test_vars) = crate::tests::setup();

        let account = client.account_by_puuid(None, test_vars.test_puuid.clone()).await.unwrap();

        assert_eq!(account.puuid, test_vars.test_puuid);
        assert_eq!(account.game_name, Some(test_vars.test_game_name));
        assert_eq!(account.tag_line, Some(test_vars.test_tag_line));
    }

    #[tokio::test]
    async fn test_account_by_game_name_specified_tagline() {
        let (client, test_vars) = crate::tests::setup();

        let account = client.account_by_riot_id(
            None, test_vars.test_game_name.clone(), test_vars.test_tag_line.clone()
        ).await.unwrap();

        assert_eq!(account.puuid, test_vars.test_puuid);
        assert_eq!(account.game_name, Some(test_vars.test_game_name));
        assert_eq!(account.tag_line, Some(test_vars.test_tag_line));
    }

    // This test is broken until i have a key that can access one of the shards
    // #[tokio::test]
    // async fn test_account_by_active_shard() {
    //     let client = crate::tests::setup();
    //
    //     let test_game_name = std::env::var("TEST_GAME_NAME").unwrap();
    //     let test_puuid = std::env::var("TEST_PUUID").unwrap();
    //
    //     let account = client.account_by_game(None, ActiveShardGame::LoR, test_puuid.clone()).await.unwrap();
    //
    //     assert_eq!(account.puuid, test_puuid);
    //     assert_eq!(account.game, "lor".to_string());
    // }
}