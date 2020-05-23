use futures_util::lock::Mutex;
use std::thread;
use std::time::Duration;
use std::sync::Arc;

use crate::rest::RestSender;
use crate::util::{format_time_period, HikaruResult};
use crate::gateway::op_code::GatewayPayload;
use crate::gateway::shardconnection::Shard;
use crate::util::error::Error::GatewayError;
use crate::gateway::close_code::GatewayErrorResolve;

pub struct Client {
    token: String,
    shard_info: (u32, u32, u32),
    pub rest: RestSender,
    pub shards: Option<Vec<Arc<Mutex<Shard>>>>
}

impl Client {
    // fn new_flexible(token: &String) -> Client {
    //     let rest = RestSender::new(token);
    //     Client {
    //         token: token.clone(),
    //         rest,
    //         shards: init_shards()
    //     }
    // }

    pub fn new(token: &str, shards: (u32, u32, u32)) -> Client {
        debug!("Starting client");
        Client {
            token: token.to_string(),
            rest: RestSender::new(token),
            shards: None,
            shard_info: shards
        }
    }

    pub async fn heartbeat_thread(&self) -> HikaruResult<()> {
        loop {
            debug!("Heartbeat waiting 5s");
            thread::sleep(Duration::from_millis(5 * 1000));
            debug!("Executing heartbeat...");
            
            match &self.shards {
                Some(shards) => {
                    for shard_lock in shards {
                        let mut guard = shard_lock.lock().await;
                        let seq = guard.seq.clone();
                        guard.send_payload(&GatewayPayload::Heartbeat(seq)).await?;
                    }
                }
                None => {
                    warn!("Got no shards!");
                }
            }
            // let mut shards = self.shards.as_ref().unwrap().lock().unwrap();
            // for mut x in shards.iter() {
            //     x.send_payload(&GatewayPayload::Heartbeat(x.seq));
            // }
        }
    }

    pub async fn init_shards(&mut self) -> HikaruResult<()> {
        debug!("[Shard Manager] Starting {}-{} out of {} shards (ETA {})",
                 self.shard_info.0, self.shard_info.0 + self.shard_info.1, self.shard_info.2,
                 format_time_period(self.shard_info.1 as u64 * 5 * 1000));
        let shard_info = self.shard_info;
        let token_copy = self.token.clone();
        let mut shards: Vec<Arc<Mutex<Shard>>> = Vec::new();

        for num in shard_info.0 .. shard_info.1 {
            let shard_instance = Shard::new(&token_copy, (num, shard_info.2)).await?;
            let shard = Arc::new(Mutex::new(shard_instance));
            let clone = shard.clone();
            
            tokio::spawn(async {
                let moved_clone = clone;
                loop {
                    if let Err(err) = Shard::shard_loop(&moved_clone).await {
                        match err {
                            GatewayError(close_code) => {
                                info!("Gateway close code {:?}", close_code);
                                match close_code.resolve() {
                                    GatewayErrorResolve::Reconnect => { info!("Reconnecting"); }
                                    GatewayErrorResolve::NewSession => { info!("Unable to start new session just yet, but received status for it"); }
                                    GatewayErrorResolve::Panic => { panic!(format!("Gateway error: {:?}", close_code)); }
                                };
                            }
                            _ => warn!("Received error: {:?}", err)
                        }
                    }
                }
            });
            thread::sleep(Duration::from_millis(5 * 1000));
            shards.push(shard);
        }
        self.shards = Some(shards);

        Ok(())
    }
}

pub mod user;
pub mod guild;