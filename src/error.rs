use serde_json;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    UnknownOperator(String),
    NoOperands,
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}
