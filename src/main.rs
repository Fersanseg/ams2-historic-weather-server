mod requests;
mod requests_utils;
mod types;

use std::{
  error::Error, io::prelude::*, net::{TcpListener, TcpStream}
};


use lambda_http::{run, service_fn, tracing, Body, Error as LambdaError, Request, RequestExt, Response};
use serde_json::to_string;
use types::error::CustomError;
use crate::types::error::ErrorCode;

const CRLF: &str = "\r\n";
const IP_ADDR: &str = "127.0.0.1:7878";


async fn function_handler(event: Request) -> Result<Response<Body>, LambdaError> {
  if let Some(date_param) = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("date")) {

      return Ok(Response::builder()
      .status(200)
      .header("content-type", "text/html")
      .body(date_param.into())
      .map_err(Box::new)?);
    }

    return Err(LambdaError::from("No date parameter found"));



    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}



// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//   run_task().await?;
//   return Ok(());
// }

// async fn run_task() -> Result<(), Box<dyn Error>> {
//   let listener = TcpListener::bind(IP_ADDR)?;
//   println!("Server running on port 7878");

//   while let Some(stream) = listener.incoming().next() {
//     match stream {
//       Ok(stream) => {
//         tokio::task::spawn(handle_connection(stream));
//       },
//       Err(e) => eprintln!("Error handling connection: {:#?}", e)
//     }
//   }

//   Ok(())
// }

// async fn handle_connection(mut stream: TcpStream) -> Result<(), CustomError> {
//   let date = match requests::get_date_param(&stream) {
//     Ok(date) => date,
//     Err(e) => {
//       eprintln!("Error getting date parameter: {:#?}", e);
//       return Err(e);
//     },
//   };

//   let data = match requests::request_weather_data(date).await {
//     Ok(data) => data,
//     Err(e) => {
//       eprintln!("API request error: {}", e);
//       return Err(CustomError {
//         code: ErrorCode::APIRequestError,
//         msg: e.to_string(),
//     });
//     },
//   };
  
//   let status_line = "HTTP/1.1 200 OK";
//   let serialized = match to_string(&data) {
//     Ok(json) => json,
//     Err(e) => {
//       eprintln!("Serialization error: {}", e);
//       return Err(CustomError {
//         code: ErrorCode::SerializationError,
//         msg: String::from(format!("Serialization error: {}", e))
//       })
//     },
//   };

//   let json_length = serialized.len();
//   let response = format!("{status_line}{CRLF}Content-Length: {json_length}{CRLF}{CRLF}{serialized}");
  
//   match stream.write_all(response.as_bytes()) {
//     Ok(_) => (),
//     Err(e) => {
//       eprintln!("Error writing response to the stream: {}", e);
//       return Err(CustomError { 
//         code: ErrorCode::StreamWriteError, 
//         msg: format!("Error writing response to the stream: {}", e)
//       })
//     }
//   }

//   return Ok(());
// }
