#![allow(unused, dead_code)]

//! A crate to interact with the full Riot API in rust
//!
//! This crate provides the ability to fully interact with the Riot API using Rust. Each part is split out, although by default League
//! is included with this crate. The breakout of features is intended to make the crate additive in a rust way.
//!
//! Explanation of features:
//!
//! ### no-default-features
//! Interact simply with the Account endpoint and Riot Single Sign-On (RSSO). This is the base form of the crate
//!
//! ### default-features
//! Interact with most features of League. Fetch matches, summoner information, champion information, queues, and more
//!
//! ### feature clash
//! Enables the ability to interact with the Clash endpoint, for fetching information about player participation and
//! performance in clash matches
//!
//! ### feature tournament
//! Enables the ability to interact with the Tournament endpoint, for creating and managing content creator tournaments
//! and generating tournament codes.
//!
//! ### feature tft (TODO)
//! Allows interaction with the TFT endpoint, which enables fetching information on TFT matches and compositions
//!
//! ### feature val (TODO)
//! Allows interaction with the Valorant endpoint, giving access to match information and performance
//!
//! ### Example
//! Get a summoner by name & tagline, retrieve match ids, and then retrieve each match
//!
//! ```no_run
//! use riot_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new API client
//!     let client = RiotApiClientBuilder::new()
//!         .api_key("YOUR_API_KEY".to_string())
//!         .default_platform(PlatformRouting::NA1)  // Default platform is not required, but by default will be set to NA1
//!         .default_region(RegionRouting::AMERICAS) // Default region is not required, but by default will be set to Americas
//!         .build().unwrap();
//!
//!     // Get the summoner by name#tag. Setting region to `None` will use the default region
//!     let summoner = client.account_by_riot_id(None, "MatrixSenpai".to_string(), "STDIN".to_string()).await.unwrap();
//!     // There are multiple optional params to include here, including count, start (for paging), start/end time, and more
//!     let match_ids = client.matches_by_puuid(None, summoner.puuid, None, None, None, None, None, None).await.unwrap();
//!
//!     let mut matches = Vec::new();
//!     for match_id in match_ids.into_iter() {
//!         let match_item = client.match_by_id(None, match_id).await.unwrap();
//!         matches.push(match_item);
//!     }
//! }

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
    use crate::{
        prelude::*,
        models::tournament::{
            ProviderRegistrationParamsV5,
            TournamentRegistrationParamsV5,
            TournamentCodeParamsV5,
            GamePickType,
            MapType,
            SpectatorType,
        }
    };

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

        pub(crate) test_tournament_provider_id: i32,
        pub(crate) test_tournament_region: String,
        pub(crate) test_tournament_provider_url: String,
        pub(crate) test_tournament_id: i32,
        pub(crate) test_tournament_name: String,
        pub(crate) test_tournament_lobby: String,
        pub(crate) test_tournament_code: String,
        pub(crate) test_tournament_provider_params: ProviderRegistrationParamsV5,
        pub(crate) test_tournament_registration_params: TournamentRegistrationParamsV5,
        pub(crate) test_tournament_code_params: TournamentCodeParamsV5,
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
                test_tournament_provider_id: var("TEST_TOURNAMENT_PROVIDER_ID").unwrap().parse().unwrap(),
                test_tournament_region: var("TEST_TOURNAMENT_REGION").unwrap(),
                test_tournament_provider_url: var("TEST_TOURNAMENT_PROVIDER_URL").unwrap(),
                test_tournament_id: var("TEST_TOURNAMENT_ID").unwrap().parse().unwrap(),
                test_tournament_name: var("TEST_TOURNAMENT_NAME").unwrap(),
                test_tournament_lobby: var("TEST_TOURNAMENT_LOBBY").unwrap(),
                test_tournament_code: var("TEST_TOURNAMENT_CODE").unwrap(),
                test_tournament_provider_params: ProviderRegistrationParamsV5 {
                    region: var("TEST_TOURNAMENT_REGION").unwrap(),
                    url: var("TEST_TOURNAMENT_PROVIDER_URL").unwrap(),
                },
                test_tournament_registration_params: TournamentRegistrationParamsV5 {
                    provider_id: var("TEST_TOURNAMENT_PROVIDER_ID").unwrap().parse().unwrap(),
                    name: Some(var("TEST_TOURNAMENT_NAME").unwrap())
                },
                test_tournament_code_params: TournamentCodeParamsV5 {
                    allowed_participants: None,
                    metadata: None,
                    team_size: 1,
                    pick_type: GamePickType::TournamentDraft,
                    map_type: MapType::SummonersRift,
                    spectator_type: SpectatorType::LobbyOnly,
                    enough_players: true,
                }
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
