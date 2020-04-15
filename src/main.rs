use std::env;
use ansi_term::Color;

use crate::util::error::Error;
use crate::gateway::shardconnection::Shard;

mod rest;
#[macro_use] mod util;
mod gateway;

#[tokio::main]
async fn main() -> Result<(), Error> {
    ansi_term::enable_an ()?;
    let mut shard = Shard::new(&format!("Bot {}", env::var("token").expect("Token not provided")), 0, 1)?;

    if let Err(e) = shard.shard_loop() {
        println!("{} Disconnected {:?}", shard_log!(shard), e);
    };
    Ok(())
}
