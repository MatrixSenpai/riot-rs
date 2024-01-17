use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    lol_match::{MatchDto, MatchTimelineDto},
};

impl RiotApiClient {
    pub async fn matches_by_puuid(
        &self,
        region_routing: Option<RegionRouting>,
        puuid: String,
        start_time: Option<i64>,
        end_time: Option<i64>,
        queue: Option<i32>,
        match_type: Option<String>,
        start: Option<usize>,
        count: Option<usize>,
    ) -> ApiResult<Vec<String>> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        let mut options = HashMap::new();

        if let Some(time) = start_time {
            options.insert("startTime".to_string(), time.to_string());
        }
        if let Some(time) = end_time {
            options.insert("endTime".to_string(), time.to_string());
        }
        if let Some(queue) = queue {
            options.insert("queue".to_string(), queue.to_string());
        }
        if let Some(match_type) = match_type {
            options.insert("type".to_string(), match_type);
        }
        if let Some(start) = start {
            options.insert("start".to_string(), start.to_string());
        }
        if let Some(count) = count {
            options.insert("count".to_string(), count.to_string());
        }

        self.make_request(
            format!("/lol/match/v5/matches/by-puuid/{puuid}/ids"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            options,
            None::<()>
        ).await
    }

    pub async fn match_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        match_id: String,
    ) -> ApiResult<MatchDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            format!("/lol/match/v5/matches/{match_id}"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await
    }

    pub async fn match_timeline_by_id(
        &self,
        region_routing: Option<RegionRouting>,
        match_id: String
    ) -> ApiResult<MatchTimelineDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        let response = self.make_request(
            format!("/lol/match/v5/matches/{match_id}/timeline"),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>,
        ).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_match_list() {
        let (client, test_vars) = crate::tests::setup();

        let match_list = client.matches_by_puuid(
            None, test_vars.test_puuid, None, None, None, None, None, None,
        ).await.unwrap();

        assert_eq!(match_list.len(), 20);
    }

    #[tokio::test]
    async fn test_get_match() {
        let (client, test_vars) = crate::tests::setup();

        let match_item = client.match_by_id(None, test_vars.test_match_id.clone()).await.unwrap();
        let binding = match_item.info.participants.iter()
            .filter(|m| m.puuid == test_vars.test_puuid)
            .collect::<Vec<&crate::models::ParticipantDto>>();
        let participant = binding.first().unwrap();

        assert!(match_item.metadata.participants.contains(&test_vars.test_puuid));
        assert_eq!(participant.champion_id, test_vars.test_champion_id);
        assert_eq!(participant.lane, "MIDDLE".to_string());
        assert_eq!(participant.summoner_id, test_vars.test_summoner_id);
        assert_eq!(participant.riot_id_game_name, Some(test_vars.test_game_name));
        assert_eq!(participant.riot_id_tagline, Some(test_vars.test_tag_line));
        assert_eq!(participant.win, true);
    }

    #[tokio::test]
    async fn test_get_match_timeline() {
        let (client, test_vars) = crate::tests::setup();

        let timeline = client.match_timeline_by_id(None, test_vars.test_match_id.clone()).await.unwrap();
        let first_frame = timeline.info.frames.first().unwrap();
        let binding = first_frame.participant_frames.iter()
            .filter(|(p, _)| p == &&"3".to_string())
            .map(|(_, p)| p)
            .collect::<Vec<&crate::models::MatchTimelineInfoFrameParticipantFrame>>();
        let participant = binding.first().unwrap();

        assert!(timeline.metadata.participants.contains(&test_vars.test_puuid));
        assert_eq!(participant.level, 1);
        assert_eq!(participant.minions_killed, 0);
    }
}