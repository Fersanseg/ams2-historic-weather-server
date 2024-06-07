use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResponse {
  pub latitude: f64,
  pub longitude: f64,
  generationtime_ms: f64,
  utc_offset_seconds: f32,
  timezone: String,
  timezone_abbreviation: String,
  elevation: f64,
  location_id: Option<i16>,
  hourly_units: HourlyUnits,
  hourly: Hourly
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HourlyUnits {
  time: String,
  temperature_2m: String,
  rain: String,
  cloud_cover: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Hourly {
  time: Vec<String>,
  temperature_2m: Vec<Option<f32>>,
  rain: Vec<Option<f32>>,
  cloud_cover: Vec<Option<u8>>
}