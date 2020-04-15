use crate::util::error::Error;

#[macro_use]
pub mod error;
#[macro_use]
pub mod logging;

pub type HikaruResult<T> = Result<T, Error>;