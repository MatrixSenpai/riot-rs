#![allow(unused, dead_code)]

pub mod endpoints;
pub mod models;

pub mod prelude {
    pub use crate::models::{
        RiotApiClient, RiotApiClientBuilder,
        routing::*
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    pub(crate) struct TestEnvVars {
        pub(crate) default_platform: PlatformRouting,
        pub(crate) default_region: RegionRouting,
        pub(crate) api_key: String,
        pub(crate) test_game_name: String,
        pub(crate) test_tag_line: String,
        pub(crate) test_puuid: String,
        pub(crate) test_summoner_id: String,
        pub(crate) test_champion_id: i32,
        pub(crate) test_match_id: String,
    }
    impl Default for TestEnvVars {
        fn default() -> Self {
            use std::env::var;

            Self {
                default_platform: PlatformRouting::NA1,
                default_region: RegionRouting::AMERICAS,
                api_key: var("API_KEY").unwrap(),
                test_game_name: var("TEST_GAME_NAME").unwrap(),
                test_tag_line: var("TEST_TAG_LINE").unwrap(),
                test_puuid: var("TEST_PUUID").unwrap(),
                test_summoner_id: var("TEST_SUMMONER_ID").unwrap(),
                test_champion_id: var("TEST_CHAMPION_ID").unwrap().parse().unwrap(),
                test_match_id: var("TEST_MATCH_ID").unwrap(),
            }
        }
    }

    pub(crate) fn setup() -> (RiotApiClient, TestEnvVars) {
        dotenv::dotenv().ok();

        let client = reqwest::Client::new();
        let test_vars = TestEnvVars::default();

        let riot_client = RiotApiClientBuilder::new()
            .default_region(test_vars.default_region.clone())
            .default_platform(test_vars.default_platform.clone())
            .riot_token(test_vars.api_key.clone())
            .with_client(client)
            .build();

        (riot_client.unwrap(), test_vars)
    }
}