use reqwest::{Error as ReqwestError};
use tungstenite::error::{Error as TungsteniteError};
use serde_json::error::{Error as SerdeError};
use std::io::{Error as StdError};
use num_enum::TryFromPrimitiveError;
use std::sync::{RwLockReadGuard, PoisonError};
use serde_json::Value;

use crate::gateway::close_code::GatewayCloseCode;
use crate::gateway::shardconnection::Shard;

#[derive(Debug)]
pub enum Error {
    // Mirrors
    ReqwestError(ReqwestError),
    RestApiError(Value),
    TungsteniteError(TungsteniteError),
    SerdeError(SerdeError),
    StdError(StdError),
    AnsiTermError(u32),

    Invalidclose_code(TryFromPrimitiveError<GatewayCloseCode>),
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
        Error::Invalidclose_code(e)
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