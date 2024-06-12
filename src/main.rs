mod requests;
mod requests_utils;
mod types;

use lambda_http::{run, service_fn, tracing::{self}, Body, Error as LambdaError, Request, RequestExt, Response};
use serde_json::to_string;


async fn function_handler(event: Request) -> Result<Response<Body>, LambdaError> {
  if let Some(date_param) = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("date")) {

    let data = match requests::request_weather_data(date_param.to_string()).await {
        Ok(data) => data,
        Err(er) => {
          let errmsg = format!("API request error: {}", er);
          eprintln!("{}", errmsg);
          return Err(LambdaError::from(errmsg));
        },
    };

    let serialized = match to_string(&data) {
      Ok(json) => json,
      Err(er) => {
        let errmsg = format!("Serialization error: {}", er);
        eprintln!("{}", errmsg);
        return Err(LambdaError::from(errmsg));
      },
    };

    return Ok(Response::builder()
      .status(200)
      .header("content-type", "text/html")
      .body(serialized.into())
      .map_err(Box::new)?);
    }

    return Err(LambdaError::from("No date parameter found"));
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}