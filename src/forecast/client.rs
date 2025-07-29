use std::num::NonZeroUsize;

use super::types::ForecastResponse;
use crate::location::Location;

const FORECAST_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/forecast";

/// A client for interacting with the 5-day forecast API.
pub struct ForecastClient {
    client: reqwest::Client,
    location: Location,
    units: String,
    api_key: String,
    cnt: Option<NonZeroUsize>,
}

impl ForecastClient {
    pub fn new(
        location: Location,
        units: String,
        api_key: String,
        cnt: Option<NonZeroUsize>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            location,
            units: units.clone(),
            api_key,
            cnt,
        }
    }

    pub async fn get_current_forecast(
        &self,
    ) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(FORECAST_API_BASE_URL)
            .query(&[
                ("lat", Some(self.location.lat.to_string())),
                ("lon", Some(self.location.lon.to_string())),
                ("units", Some(self.units.to_string())),
                ("appid", Some(self.api_key.clone())),
                ("cnt", self.cnt.map(|cnt| format!("{}", cnt))),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }
}
