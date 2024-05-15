mod requests;
mod requests_utils;
mod types;

use std::{
  error::Error, io::prelude::*, net::{TcpListener, TcpStream}
};

use types::error::CustomError;
use crate::types::error::ErrorCode;

const CRLF: &str = "\r\n";
const IP_ADDR: &str = "127.0.0.1:7878";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  run_task().await?;
  return Ok(());
}

async fn run_task() -> Result<(), Box<dyn Error>> {
  let listener = TcpListener::bind(IP_ADDR)?;
  println!("Server running on port 7878");

  while let Some(stream) = listener.incoming().next() {
    match stream {
      Ok(stream) => {
        tokio::task::spawn(handle_connection(stream));
      },
      Err(e) => eprintln!("Error handling connection: {:#?}", e)
    }
  }

  Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), CustomError> {
  println!("Handling connection");
  let weather_data: Result<(), Box<dyn std::error::Error + Sync + Send>> = match requests::get_date_param(&stream) {
    Ok(date) => requests::request_weather_data(date).await,
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
