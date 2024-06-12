use reqwest::Client;

use crate::{
  requests_utils::{
    build_api_url,
    map_coords
  }, 
  types::weather_data::WeatherData
};
use crate::types::api_response::ApiResponse;

pub async fn request_weather_data(date: String) -> Result<Vec<WeatherData>, Box<dyn std::error::Error + Send + Sync>> {
  let url = build_api_url(date)?;
  
  let client = Client::new();

  let response = client.get(url).send().await?;
  let status_code = response.status();
  if status_code.is_success() {
    match response.json::<Vec<ApiResponse>>().await {
      Ok(body) => return Ok(map_coords(body)),
      Err(err) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, err)))
    }
  }

  Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Weather API request failed")))
}




