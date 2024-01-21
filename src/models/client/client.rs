use std::collections::HashMap;
use std::sync::Arc;
use serde::{
    Deserialize, Serialize,
    de::DeserializeOwned,
};
use reqwest::{
    Client, Method, Request, RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use reqwest_middleware::{
    ClientWithMiddleware, ClientBuilder,
};
use reqwest_retry::{
    RetryTransientMiddleware, policies::ExponentialBackoff
};
use super::{
    ApiResult, RiotApiErrorStatus, ApiError,
    configuration::{ApiConfiguration, Routable, AuthConfiguration}
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Successful(T),
    RiotApiError(RiotApiErrorStatus),
    Unknown(String),
}

pub struct RiotApiClient {
    pub(crate) configuration: Arc<ApiConfiguration>,
    client: ClientWithMiddleware,
}
impl RiotApiClient {
    pub fn new(api_configuration: ApiConfiguration, client: Client) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(
            api_configuration.retry_count
        );

        let client = ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

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
            .await.map_err(super::map_reqwest_middleware_error)?;

        let status = response.status();

        let response_decoded: ApiResponse<T> = response.json()
            .await.map_err(super::map_reqwest_error)?;

        match response_decoded {
            ApiResponse::Successful(val) => Ok(val),
            ApiResponse::RiotApiError(e) if status.eq(&429) => Err(ApiError::RateLimitError(e.status)),
            ApiResponse::RiotApiError(e) => Err(ApiError::RiotApiError(e.status, status)),
            ApiResponse::Unknown(e) => Err(ApiError::UnknownTypeError(e)),
        }
    }
}

unsafe impl Send for RiotApiClient {}
unsafe impl Sync for RiotApiClient {}