use std::collections::HashMap;
use crate::{
    prelude::RiotApiClient,
    models::{
        ApiResult,
        routing::PlatformRouting,
        champion_mastery::ChampionMasteryDto,
    }
};

impl RiotApiClient {
    /// Get all champion mastery entries by puuid sorted by champion mastery points
    pub async fn champion_mastery_by_puuid(
        &self,
        platform_routing: Option<PlatformRouting>,
        puuid: String,
    ) -> ApiResult<Vec<ChampionMasteryDto>> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-puuid/{puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a champion mastery by puuid and champion ID
    pub async fn champion_mastery_by_puuid_champion_id(
        &self,
        platform_routing: Option<PlatformRouting>,
        puuid: String,
        champion_id: i32,
    ) -> ApiResult<ChampionMasteryDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-puuid/{puuid}/by-champion/{champion_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a specified number of top champion mastery entries by puuid sorted by champion mastery points
    pub async fn champion_mastery_by_puuid_top(
        &self,
        platform_routing: Option<PlatformRouting>,
        puuid: String,
        count: Option<u16>,
    ) -> ApiResult<Vec<ChampionMasteryDto>> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);
        let count = count.unwrap_or(3);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-puuid/{puuid}/top?count={count}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get all champion mastery entries by summoner id sorted by champion mastery points
    pub async fn champion_mastery_by_summoner_id(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
    ) -> ApiResult<Vec<ChampionMasteryDto>> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{summoner_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a champion mastery by summoner id and champion ID
    pub async fn champion_mastery_by_summoner_id_champion_id(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
        champion_id: i64,
    ) -> ApiResult<ChampionMasteryDto> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-puuid/{summoner_id}/by-champion/{champion_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a specified number of top champion mastery entries by summoner id sorted by champion mastery points
    pub async fn champion_mastery_by_summoner_id_top(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
        count: Option<u16>,
    ) -> ApiResult<Vec<ChampionMasteryDto>> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);
        let count = count.unwrap_or(3);

        self.make_request(
            format!("/lol/champion-mastery/v4/champion-masteries/by-puuid/{summoner_id}/top?count={count}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a player's total champion mastery score by puuid
    pub async fn champion_mastery_scores_by_puuid(
        &self,
        platform_routing: Option<PlatformRouting>,
        puuid: String,
    ) -> ApiResult<i32> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/scores/by-puuid/{puuid}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    /// Get a player's total champion mastery score by summoner id
    pub async fn champion_mastery_scores_by_summoner_id(
        &self,
        platform_routing: Option<PlatformRouting>,
        summoner_id: String,
    ) -> ApiResult<i32> {
        let routing = platform_routing.unwrap_or(self.configuration.default_platform);

        self.make_request(
            format!("/lol/champion-mastery/v4/scores/by-summoner/{summoner_id}"),
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
    #[tokio::test]
    async fn test_champion_mastery_by_puuid() {
        let (client, test_vars) = crate::tests::setup();

        let champions = client.champion_mastery_by_puuid(None, test_vars.test_puuid.clone())
            .await.unwrap();

        let champion = champions.first().unwrap();

        assert_eq!(champion.puuid, Some(test_vars.test_puuid));
        assert_eq!(champion.champion_id, test_vars.test_champion_id);
        assert!(champion.last_play_time.ge(&1701849021000));
        assert!(champion.champion_points.ge(&991726));
    }

    #[tokio::test]
    async fn test_champion_mastery_by_puuid_champion_id() {
        let (client, test_vars) = crate::tests::setup();

        let champion = client.champion_mastery_by_puuid_champion_id(
            None, test_vars.test_puuid.clone(), test_vars.test_champion_id
        ).await.unwrap();

        assert_eq!(champion.puuid, Some(test_vars.test_puuid));
        assert_eq!(champion.champion_id, test_vars.test_champion_id);
        assert!(champion.last_play_time.ge(&1701849021000));
        assert!(champion.champion_points.ge(&991726));
    }

    #[tokio::test]
    async fn test_champion_mastery_by_puuid_top() {
        let (client, test_vars) = crate::tests::setup();

        let champions = client.champion_mastery_by_puuid_top(
            None, test_vars.test_puuid.clone(), None
        ).await.unwrap();

        let champion = champions.first().unwrap();

        assert_eq!(champion.puuid, Some(test_vars.test_puuid));
        assert_eq!(champions.len(), 3);
        assert_eq!(champion.champion_id, test_vars.test_champion_id);
        assert!(champion.last_play_time.ge(&1701849021000));
        assert!(champion.champion_points.ge(&991726));
    }

    // Broken test - waiting on riot to fix
    // #[tokio::test]
    // async fn test_champion_mastery_by_summoner_id() {
    //     let client = crate::tests::setup();
    //
    //     let test_puuid = std::env::var("TEST_PUUID").unwrap();
    //     let test_summoner_id = std::env::var("TEST_SUMMONER_ID").unwrap();
    //     let test_champion = std::env::var("TEST_CHAMPION_ID")
    //         .unwrap().parse::<i64>().unwrap();
    //
    //     let champions = client.champion_mastery_by_summoner_id(None, test_summoner_id.clone())
    //         .await.unwrap();
    //
    //     let champion = champions.first().unwrap();
    //
    //     assert_eq!(champion.puuid, Some(test_puuid));
    //     assert_eq!(champion.champion_id, test_champion);
    //     assert!(champion.last_play_time.ge(&1701849021000));
    //     assert!(champion.champion_points.ge(&991726));
    // }

    // Broken test - waiting on riot to fix
    // #[tokio::test]
    // async fn test_champion_mastery_by_summoner_id_champion_id() {
    //     let client = crate::tests::setup();
    //
    //     let test_puuid = std::env::var("TEST_PUUID").unwrap();
    //     let test_summoner_id = std::env::var("TEST_SUMMONER_ID").unwrap();
    //     let test_champion = std::env::var("TEST_CHAMPION_ID")
    //         .unwrap().parse::<i64>().unwrap();
    //
    //     let champion = client.champion_mastery_by_summoner_id_champion_id(None, test_summoner_id.clone(), test_champion)
    //         .await.unwrap();
    //
    //     assert_eq!(champion.puuid, Some(test_puuid));
    //     assert_eq!(champion.champion_id, test_champion);
    //     assert!(champion.last_play_time.ge(&1701849021000));
    //     assert!(champion.champion_points.ge(&991726));
    // }

    #[tokio::test]
    async fn test_champion_mastery_by_summoner_id_top() {
        let (client, test_vars) = crate::tests::setup();

        let champions = client.champion_mastery_by_summoner_id_top(
            None, test_vars.test_puuid.clone(), None
        ).await.unwrap();

        let champion = champions.first().unwrap();

        assert_eq!(champion.puuid, Some(test_vars.test_puuid));
        assert_eq!(champions.len(), 3);
        assert_eq!(champion.champion_id, test_vars.test_champion_id);
        assert!(champion.last_play_time.ge(&1701849021000));
        assert!(champion.champion_points.ge(&991726));
    }

    #[tokio::test]
    async fn test_champion_mastery_score_by_puuid() {
        let (client, test_vars) = crate::tests::setup();

        let total = client.champion_mastery_scores_by_puuid(None, test_vars.test_puuid)
            .await.unwrap();

        assert!(total.ge(&550))
    }

    // Broken test - waiting on riot to fix
    // #[tokio::test]
    // async fn test_champion_mastery_score_by_summoner_id() {
    //     let client = crate::tests::setup();
    //
    //     let test_summoner_id = std::env::var("TEST_SUMMONER_ID").unwrap();
    //
    //     let total = client.champion_mastery_scores_by_summoner_id(None, test_summoner_id)
    //         .await.unwrap();
    //
    //     assert!(total.ge(&550))
    // }
}