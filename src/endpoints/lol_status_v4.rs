use std::collections::HashMap;
use crate::models::{
    RiotApiClient, ApiResult,
    routing::RegionRouting,
    platform_data::PlatformDataDto
};

impl RiotApiClient {
    pub async fn platform_data(
        &self,
        region_routing: Option<RegionRouting>,
    ) -> ApiResult<PlatformDataDto> {
        let routing = region_routing.unwrap_or(self.configuration.default_region);

        self.make_request(
            "/lol/status/v4/platform-data".to_string(),
            routing,
            reqwest::Method::GET,
            HashMap::new(),
            HashMap::new(),
            None::<()>
        ).await
    }
}