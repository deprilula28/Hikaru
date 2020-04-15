use std::thread;
use std::env;
use crate::error::Error;
use std::time::Duration;

pub mod rest;
pub mod error;
pub mod gateway;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut shard = gateway::shardconnection::Shard::new(&format!("Bot {}", env::var("token").expect("Token not provided")), 0, 1)?;
    thread::spawn(move || gateway::shardconnection::shard_loop(&mut shard));
    thread::sleep(Duration::from_millis(1000));
    Ok(())
}
