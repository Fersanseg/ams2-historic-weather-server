mod requests;
mod types;

use std::{
  io::{prelude::*, Error}, net::{TcpListener, TcpStream}
};

use types::error::CustomError;
use crate::types::error::ErrorCode;

const CRLF: &str = "\r\n";
const IP_ADDR: &str = "127.0.0.1:7878";

fn main() -> Result<(), Error> {
  let listener = TcpListener::bind(IP_ADDR)?;
  println!("Server running on port 7878");

  for stream in listener.incoming() {
    let stream = stream?;
    match handle_connection(stream) {
      Ok(_) => (),
      Err(e) => eprintln!("Error handling connection: {:#?}", e)
    }
  }

  return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(), CustomError> {
  println!("Handling connection");
  let weatherData: Result<(), Box<dyn std::error::Error + Sync + Send>> = match requests::get_date_param(&stream) {
    Ok(date) => requests::request_weather_data(date),
    Err(e) => {
      eprintln!("Error getting date parameter: {:#?}", e);
      return Err(e);
    }
  };

  let status_line = "HTTP/1.1 200 OK";
  let date_length = "{date}".len();

  let response = format!("{status_line}{CRLF}Content-Length: {date_length}{CRLF}{CRLF}wqerwqrqwe");

  match stream.write_all(response.as_bytes()) {
    Ok(_) => (),
    Err(e) => {
      eprintln!("Error writing response to the stream: {}", e);
      return Err(CustomError { 
        code: ErrorCode::StreamWriteError, 
        msg: format!("Error writing response to the stream: {}", e)
      })
    }
  }

  return Ok(());
}
