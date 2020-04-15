use reqwest::{Error as ReqwestError};
use tungstenite::error::{Error as TungsteniteError};
use serde_json::error::{Error as SerdeError};
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    ReqwestError(ReqwestError),
    RestApiError(Value),
    TungsteniteError(TungsteniteError),
    SerdeError(SerdeError),
    Text(String)
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error {
        Error::ReqwestError(e)
    }
}

impl From<TungsteniteError> for Error {
    fn from(e: TungsteniteError) -> Error {
        Error::TungsteniteError(e)
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Error {
        Error::SerdeError(e)
    }
}