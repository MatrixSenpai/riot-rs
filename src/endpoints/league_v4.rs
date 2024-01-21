use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    league::{
        league_list::LeagueListDto,
        league_entry::{
            LeagueEntryDto,
            LeagueQueueEntry,
            LeagueTierEntry,
            LeagueDivisionEntry
        },
    },
};

// TODO: Needs tests written
// TODO: Needs documentation written
impl RiotApiClient {
    pub async fn challenger_leagues_by_queue(
        &self,
        region_routing: Option<RegionRouting>,
        league_queue_entry: LeagueQueueEntry,
    ) -> ApiResult<LeagueListDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/league/v4/challengerleagues/by-queue/{league_queue_entry}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn entries_by_summoner(
        &self,
        region_routing: Option<RegionRouting>,
        summoner_id: String,
    ) -> ApiResult<LeagueEntryDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/league/v4/entries/by-summoner/{summoner_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn league_entries(
        &self,
        region_routing: Option<RegionRouting>,
        league_queue_entry: LeagueQueueEntry,
        league_tier_entry: LeagueTierEntry,
        league_division_entry: LeagueDivisionEntry,
        page: Option<usize>,
    ) -> ApiResult<Vec<LeagueEntryDto>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        let options = if let Some(page) = page {
            let mut temp = HashMap::new();
            temp.insert("page".to_string(), page.to_string());
            temp
        } else { HashMap::new() };

        self.make_request(
            format!(
                "/lol/league/entries/{league_queue_entry}/{league_tier_entry}/{league_division_entry}"
            ),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            options,
            None::<()>
        ).await
    }

    pub async fn grandmaster_leagues_by_queue(
        &self,
        region_routing: Option<RegionRouting>,
        league_queue_entry: LeagueQueueEntry,
    ) -> ApiResult<LeagueListDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/league/v4/grandmasterleagues/by-queue/{league_queue_entry}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn leagues(
        &self,
        region_routing: Option<RegionRouting>,
        league_id: String,
    ) -> ApiResult<LeagueListDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/league/v4/leagues/{league_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn master_leagues_by_queue(
        &self,
        region_routing: Option<RegionRouting>,
        league_queue_entry: LeagueQueueEntry,
    ) -> ApiResult<LeagueListDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/league/v4/masterleagues/by-queue/{league_queue_entry}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }
}