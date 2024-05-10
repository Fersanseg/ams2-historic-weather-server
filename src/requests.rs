use std::{
  io::{BufRead, BufReader},
  net::TcpStream
};

use crate::types::error::{
  CustomError, 
  ErrorCode
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