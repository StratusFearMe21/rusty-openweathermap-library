use crate::weather::{Clouds, Main, Weather, Wind};
use serde::{Deserialize, Serialize};

// region Sys

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    /// Part of the day (n - night, d - day)
    pub pod: char,
}

impl Sys {
    pub fn new(pod: char) -> Result<Self, String> {
        if pod != 'd' && pod != 'n' {
            return Err("Part of day must be either 'd' (day) or 'n' (night)".to_string());
        }

        Ok(Sys { pod })
    }
}

// endregion

// region Rain
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rain {
    #[serde(rename = "3h")]
    /// Rain volume for last 3 hours, mm. Please note that only mm as units of measurement are available for this parameter
    three_hour: f64,
}

impl Rain {
    pub fn new(three_hour: f64) -> Self {
        Self { three_hour }
    }
}

// endregion

// region Snow
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snow {
    #[serde(rename = "3h")]
    /// Snow volume for last 3 hours. Please note that only mm as units of measurement are available for this parameter
    three_hour: f64,
}

impl Snow {
    pub fn new(three_hour: f64) -> Self {
        Self { three_hour }
    }
}

// endregion

// region ForecastItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastItem {
    /// Time of data forecasted, unix, UTC
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    /// Average visibility, metres. The maximum value of the visibility is 10km
    pub visibility: i64,
    /// Probability of precipitation. The values of the parameter vary between 0 and 1, where 0 is equal to 0%, 1 is equal to 100%
    pub pop: f64,
    pub rain: Rain,
    pub snow: Snow,
    pub sys: Sys,
    /// Time of data forecasted, ISO, UTC
    pub dt_txt: String,
}

// endregion

// region ForecastResponse
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastResponse {
    /// Internal parameter
    pub cod: String,
    /// Internal parameter
    pub message: i64,
    /// A number of timestamps returned in the API response
    pub cnt: usize,
    pub list: Vec<ForecastItem>,
}

impl ForecastResponse {
    pub fn new(cod: String, message: i64, list: Vec<ForecastItem>) -> Self {
        ForecastResponse {
            cod,
            message,
            cnt: list.len(),
            list,
        }
    }
}

// endregion
