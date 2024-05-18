use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};
use reqwest::Client;

use crate::requests_utils::{extract_date, build_api_url};
use crate::types::{
  api_response::ApiResponse, 
  error::{
    CustomError, 
    ErrorCode
  }
};


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




pub async fn request_weather_data(date: String) -> Result<Vec<ApiResponse>, Box<dyn std::error::Error + Send + Sync>> {
  let url = build_api_url(date)?;
  
  let client = Client::new();

  let response = client.get(url).send().await?;
  let status_code = response.status();
  if status_code.is_success() {
    match response.json::<Vec<ApiResponse>>().await {
      Ok(body) => return Ok(body),
      Err(err) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, err)))
    }
  }

  Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Weather API request failed")))
}




