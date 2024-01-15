use reqwest::Client;
use crate::models::{
    routing::*,
    client::{
        ConversionError,
        configuration::{AuthConfiguration, ApiConfiguration},
    },
};
use crate::models::RiotApiClient;

#[derive(Default)]
pub struct RiotApiClientBuilder {
    client: Option<Client>,
    api_key: Option<String>,
    riot_token: Option<String>,
    default_region: Option<RegionRouting>,
    default_platform: Option<PlatformRouting>,
}
impl RiotApiClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, key: String) -> Self {
        if self.riot_token.is_some() { log::warn!("API key being set but Riot token is already set. Token will be preferred over key") }
        self.api_key = Some(key);
        self
    }
    pub fn riot_token(mut self, token: String) -> Self {
        if self.api_key.is_some() { log::warn!("Riot token is being set but API key is already set. Token will be preferred over key") }
        self.riot_token = Some(token);
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn default_region(mut self, default: RegionRouting) -> Self {
        self.default_region = Some(default);
        self
    }
    pub fn default_platform(mut self, default: PlatformRouting) -> Self {
        self.default_platform = Some(default);
        self
    }

    pub fn build(self) -> Result<RiotApiClient, ConversionError> {
        let auth_config = match (self.riot_token, self.api_key) {
            (Some(token), _) => AuthConfiguration::BearerToken(token),
            (_, Some(key)) => AuthConfiguration::ApiKey(key),
            _ => return Err(ConversionError::MissingDataError)
        };
        let config = ApiConfiguration::new(auth_config, self.default_region, self.default_platform);

        let client = self.client.ok_or(ConversionError::MissingDataError)?;

        Ok(RiotApiClient::new(config, client))
    }
}