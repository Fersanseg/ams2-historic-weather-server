use std::{
  io::{BufRead, BufReader},
  net::TcpStream, str::FromStr
};
use chrono::{Duration, NaiveDate};

use crate::types::error::{
  CustomError, 
  ErrorCode
};

const API_BASE_URL: &str = "https://archive-api.open-meteo.com/v1/archive?";

pub fn get_date_param(stream: &TcpStream) -> Result<String, CustomError> {
  let buf_reader = BufReader::new(stream);

  for line in buf_reader.lines() {
    match line {
      Ok(ok_line) => {
        if ok_line.trim().is_empty() {
          return Err(CustomError {
            code: ErrorCode::NoRequestInfoFound,
            msg: String::from("No date was provided with the request")
          })
        }
        if let Some(date) = extract_date(ok_line) {
          return Ok(date);
        }
      },
      Err(err) => return Err(CustomError {
        code: ErrorCode::BufReaderError, 
        msg: format!("Error reading the browser request: '{:#?}'", err) 
      }),
    };
  }
  
  Err(CustomError {
    code: ErrorCode::NoRequestInfoFound,
    msg: String::from("No date was provided with the request")
  })
}


fn extract_date(req_line: String) -> Option<String> {
  if !req_line.starts_with("GET") {
    return None;
  }

  req_line.split_once("GET /?date=")
    // Discard the first part of the split ("GET /?date=") and keep the part that retains the date
    .and_then(|(_, date_part)| date_part.split_once(' '))
    // Discard the second part of the split and keep the first one (the actual date) 
    .map(|(date, _)| String::from(date))
}

pub fn request_weather_data(date: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let url = String::from(API_BASE_URL);

  match NaiveDate::from_str(&date) {
    Ok(date_chrono) => {
      let date_next_day = date_chrono + Duration::days(1);

      let latitude_param = format!("{}{}", "latitude=", ListCircuitCoordinates::latitudes());
      let longitude_param = format!("{}{}", "&longitude=", ListCircuitCoordinates::longitudes());
      let start_date_param = format!("{}{}", "&start_date=", date);
      let end_date_param = format!("{}{}", "&end_date=", date_next_day);

      let full_url = format!(
        "{}{}{}{}{}{}",
        url,
        latitude_param,
        longitude_param,
        start_date_param,
        end_date_param,
        String::from(ListCircuitCoordinates::DESIRED_DATA_PARAMS)
      );
          
      println!("{:#?}", full_url);
    },
    Err(date_parse_error) => return Err(Box::new(date_parse_error)),
  }
  
  Ok(())
}


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