// use std::fmt;
// use std::option::NoneError;
// use std::error::Error as StdError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum Error {
    SerdeJsonError,
    ParseError(String)
}

impl From<SerdeJsonError> for Error {
  fn from(_error: SerdeJsonError) -> Self {
    Error::ParseError(String::from("Can't parse JSON"))
  }
}
