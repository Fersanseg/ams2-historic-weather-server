use std::{
  io::{BufRead, BufReader},
  net::TcpStream, os::linux::raw::stat
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




pub async fn request_weather_data(date: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let url = build_api_url(date)?;

  let client = Client::new();

  let response = client.get(url).send().await?;
  let status_code = response.status();
  if status_code.is_success() {
    let body = response.json::<ApiResponse>().await?;
    // The call does happen, need to fill out the ApiResponse struct with the response JSON structure
    // TODO: Change start_date and end_date (use input date as end_date, and 1 day before as start_date)
    //  This is because the api returns an error if the end_date is higher than the current date
    // println!("RESPONSE API: {:#?}", body);
  }

  Ok(())
}




