use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub discriminator: &'a str,
    pub bot: bool,
    pub avatar: Option<&'a str>,
    pub flags: u32
}