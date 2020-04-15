use std::env;
use ansi_term::Color;

use crate::util::error::Error;
use crate::bot::Bot;

#[macro_use]
pub mod rest;
#[macro_use]
pub mod util;
#[macro_use]
pub mod gateway;

pub mod bot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let mut shard = Shard::new(&format!("Bot {}", env::var("token").expect("Token not provided")), 0, 1)?;

    // if let Err(e) = shard.shard_loop() {
    //     println!("{} Disconnected {:?}", shard_log!(shard), e);
    // };

    let bot = Bot::new(&env::var("token").expect("Token not provided"), (0, 1, 1));
    bot.heartbeat_thread();
    Ok(())
}
