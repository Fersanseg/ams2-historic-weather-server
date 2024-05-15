use std::str::FromStr;
use chrono::{Duration, NaiveDate, ParseError};


pub fn extract_date(req_line: String) -> Option<String> {
  if !req_line.starts_with("GET") {
    return None;
  }

  req_line.split_once("GET /?date=")
    // Discard the first part of the split ("GET /?date=") and keep the part that retains the date
    .and_then(|(_, date_part)| date_part.split_once(' '))
    // Discard the second part of the split and keep the first one (the actual date) 
    .map(|(date, _)| String::from(date))
}


pub fn build_api_url(date: String) -> Result<String, ParseError> {
  let date_chrono = NaiveDate::from_str(&date)?;
  let date_next_day = date_chrono + Duration::days(1);

  let latitude_param = format!("{}{}", "latitude=", ListCircuitCoordinates::latitudes());
  let longitude_param = format!("{}{}", "&longitude=", ListCircuitCoordinates::longitudes());
  let start_date_param = format!("{}{}", "&start_date=", date);
  let end_date_param = format!("{}{}", "&end_date=", date_next_day);

  let full_url = format!(
    "{}{}{}{}{}{}",
    String::from(API_BASE_URL),
    latitude_param,
    longitude_param,
    start_date_param,
    end_date_param,
    String::from(ListCircuitCoordinates::DESIRED_DATA_PARAMS)
  );
      
  println!("{:#?}", full_url);

  return Ok(full_url);
}

const API_BASE_URL: &str = "https://archive-api.open-meteo.com/v1/archive?";

#[derive(Debug)]
struct CircuitCoordinates<'a> {
  name: &'a str,
  latitude: &'a str,
  longitude: &'a str
}

struct ListCircuitCoordinates {}

impl ListCircuitCoordinates {
  const CIRCUITS_COORDS: [CircuitCoordinates<'static>; 3] = [
    CircuitCoordinates{ name: "Le Mans", latitude: "47.97891", longitude: "0.14950167"},
    CircuitCoordinates{ name: "Suzuka", latitude: "34.8833", longitude: "136.5833"},
    CircuitCoordinates{ name: "Interlagos", latitude: "23.7048", longitude: "46.6992"}
  ];
  const DESIRED_DATA_PARAMS: &'static str = "&hourly=temperature_2m,rain,cloud_cover";

  fn latitudes() -> String {
    return Self::CIRCUITS_COORDS
        .map(|c| c.latitude)
        .join(",");
  }

  fn longitudes() -> String {
    return Self::CIRCUITS_COORDS
        .map(|c| c.longitude)
        .join(",");
  }
}