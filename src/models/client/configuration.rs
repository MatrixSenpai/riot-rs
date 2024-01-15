use crate::models::routing::*;

pub(crate) trait Routable {
    fn base_url(&self) -> String;
}

pub struct ApiConfiguration {
    pub(crate) auth_configuration: AuthConfiguration,

    pub(crate) default_region: RegionRouting,
    pub(crate) default_platform: PlatformRouting,
}
impl ApiConfiguration {
    pub fn new(auth_configuration: AuthConfiguration, default_region: Option<RegionRouting>, default_platform: Option<PlatformRouting>) -> Self {
        Self {
            auth_configuration,
            default_region: default_region.unwrap_or(RegionRouting::AMERICAS),
            default_platform: default_platform.unwrap_or(PlatformRouting::NA1),
        }
    }
}

pub enum AuthConfiguration {
    ApiKey(String),
    BearerToken(String),
}