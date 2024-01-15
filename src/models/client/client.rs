use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use reqwest::{
    Client, Method, Request, RequestBuilder,
    header::HeaderMap,
};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use super::{
    ApiResult, RiotApiErrorStatus, ApiError,
    configuration::{ApiConfiguration, Routable, AuthConfiguration}
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Successful(T),
    RiotApiError(RiotApiErrorStatus),
}

pub struct RiotApiClient {
    pub(crate) configuration: Arc<ApiConfiguration>,
    client: Client,
}
impl RiotApiClient {
    pub fn new(api_configuration: ApiConfiguration, client: Client) -> Self {
        Self {
            configuration: Arc::new(api_configuration),
            client,
        }
    }

    pub(crate) async fn make_request<R, B, T>(
        &self,
        endpoint: String,
        routing: R,
        method: Method,
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
        body: Option<B>,
    ) -> ApiResult<T>
    where
        R: Routable + Sized,
        B: Serialize + Sized,
        T: DeserializeOwned {
        let buildable_url = format!("{}{}", routing.base_url(), endpoint);
        let buildable_headers = HeaderMap::from_iter(
            headers.into_iter().filter_map(|(k, v)| {
                let header_key = k.as_str().try_into().ok()?;
                let header_val = k.as_str().try_into().ok()?;

                Some((header_key, header_val))
            })
        );

        let mut request = self.client.request(method, buildable_url)
            .headers(buildable_headers)
            .query(&query_params);

        if let Some(body) = body {
            request = request.json(&body);
        }

        match &self.configuration.auth_configuration {
            AuthConfiguration::ApiKey(key_value) => {
                request = request.query(
                    &[("api_key", key_value)]
                )
            }
            AuthConfiguration::BearerToken(token) => {
                request = request.header("X-Riot-Token", token);
            }
        }

        let request = request.build()
            .map_err(super::map_reqwest_error)?;

        let response = self.client.execute(request)
            .await.map_err(super::map_reqwest_error)?;

        let status = response.status();

        let response_decoded: ApiResponse<T> = response.json()
            .await.map_err(super::map_reqwest_error)?;

        match response_decoded {
            ApiResponse::Successful(val) => Ok(val),
            ApiResponse::RiotApiError(e) => Err(ApiError::RiotApiError(e.status, status)),
        }
    }
}

unsafe impl Send for RiotApiClient {}
unsafe impl Sync for RiotApiClient {}