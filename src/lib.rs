//! # openweathermap_lib
//!
//! A Rust library for interacting with the OpenWeatherMap API.
//! This library supports both native Rust applications and WebAssembly targets.
//!
//! ## Features
//!
//! - Current weather data retrieval
//! - Location-based weather lookups
//! - Fully typed API responses

pub mod forecast;
pub mod location;
pub mod weather;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[cfg(target_arch = "wasm32")]
pub(crate) use console_log;
