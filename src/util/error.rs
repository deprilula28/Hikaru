use async_tungstenite::tungstenite::error::{Error as TungsteniteError};
use reqwest::{Error as ReqwestError};
use serde_json::error::{Error as SerdeError};
use std::io::{Error as StdError};
use num_enum::TryFromPrimitiveError;
use serde_json::Value;

use crate::gateway::close_code::GatewayCloseCode;

#[derive(Debug)]
pub enum Error {
    // Mirrors
    ReqwestError(ReqwestError),
    RestApiError(Value),
    TungsteniteError(TungsteniteError),
    SerdeError(SerdeError),
    StdError(StdError),
    AnsiTermError(u32),

    InvalidCloseCode(TryFromPrimitiveError<GatewayCloseCode>),
    GatewayError(GatewayCloseCode),
    Text(String)
}

impl From<u32> for Error {
    fn from(e: u32) -> Error {
        Error::AnsiTermError(e)
    }
}

impl From<TryFromPrimitiveError<GatewayCloseCode>> for Error {
    fn from(e: TryFromPrimitiveError<GatewayCloseCode>) -> Error {
        Error::InvalidCloseCode(e)
    }
}

impl From<StdError> for Error { // an std error, like u lmao
    fn from(e: StdError) -> Error {
        Error::StdError(e)
    }
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