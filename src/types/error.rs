#[derive(Debug)]
pub struct CustomError {
  pub code: ErrorCode,
  pub msg: String
}

#[derive(Debug)]
pub enum ErrorCode {
  BufReaderError,
  NoRequestInfoFound,
  StreamWriteError,
  APIRequestError,
  SerializationError
}