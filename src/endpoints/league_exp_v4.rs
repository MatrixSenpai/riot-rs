use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    league_entry::{LeagueEntryDto, LeagueQueueEntry, LeagueTierEntry, LeagueDivisionEntry}
};

impl RiotApiClient {
    pub async fn league_entries_by(
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
                "/lol/league-exp/entries/{league_queue_entry}/{league_tier_entry}/{league_division_entry}"
            ),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            options,
            None::<()>
        ).await
    }
}