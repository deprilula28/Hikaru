use serde::{Deserialize, Serialize};
use crate::client::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ready<'a> {
    pub v: u32,
    #[serde(borrow)] pub user: User<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Event<'a> {
    #[serde(borrow)] Ready(Ready<'a>),
    Resumed()
}