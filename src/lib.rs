#![allow(unused, dead_code)]

pub mod endpoints;
pub mod models;

pub mod prelude {
    pub use crate::models::{
        RiotApiClient, RiotApiClientBuilder,
    };
}

#[cfg(test)]
mod tests {
    use crate::models::routing::{PlatformRouting, RegionRouting};
    use crate::prelude::{RiotApiClient, RiotApiClientBuilder};

    pub(crate) fn setup() -> RiotApiClient {
        dotenv::dotenv().ok();

        let client = reqwest::Client::new();

        let riot_client = RiotApiClientBuilder::new()
            .default_region(RegionRouting::AMERICAS)
            .default_platform(PlatformRouting::NA1)
            .riot_token(std::env::var("API_KEY").unwrap())
            .with_client(client)
            .build();

        riot_client.unwrap()
    }
}