use serde::{Deserialize, Serialize};

use super::api_response::ApiResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
  pub name: String,
  pub data: ApiResponse
}