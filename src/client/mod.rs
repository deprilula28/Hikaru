use std::thread;
use std::sync::{Arc, RwLock, Mutex, MutexGuard};
use std::time::Duration;
use serde_json::{Value, Number};

use crate::rest::RestSender;
use crate::util::error::Error;
use crate::util::{format_time_period, HikaruResult};
use crate::gateway::op_code::GatewayPayload;
use crate::gateway::shardconnection::Shard;

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
        println!("Starting client");
        Client {
            token: token.to_string(),
            rest: RestSender::new(token),
            shards: None,
            shard_info: shards
        }
    }

    pub fn heartbeat_thread(&self) -> HikaruResult<()> {
        loop {
            debug!("Heartbeat waiting 45s");
            thread::sleep(Duration::from_millis(45 * 1000));
            debug!("Executing heartbeat...");
            if let Some(shards) = &self.shards {
                for shard in shards {
                    let mut lock = shard.lock().unwrap();
                    let seq = lock.seq;
                    lock.send_payload(&GatewayPayload::Heartbeat(seq));
                }
            }
            // let mut shards = self.shards.as_ref().unwrap().lock().unwrap();
            // for mut x in shards.iter() {
            //     x.send_payload(&GatewayPayload::Heartbeat(x.seq));
            // }
        }
    }

    pub fn init_shards(&mut self) -> HikaruResult<()> {
        println!("[Shard Manager] Starting {}-{} out of {} shards (ETA {})",
                 self.shard_info.0, self.shard_info.0 + self.shard_info.1, self.shard_info.2,
                 format_time_period(self.shard_info.1 as u64 * 5 * 1000));
        let shard_info = self.shard_info;
        let token_copy = self.token.clone();

        self.shards = Some((shard_info.0 .. shard_info.1).map(|num| {
            let mut shard = Arc::new(Mutex::new(Shard::new(&token_copy, (num, shard_info.2))?));
            let mut clone = shard.clone();
            thread::spawn(|| Shard::shard_loop(clone));
            thread::sleep(Duration::from_millis(5 * 1000));
            return Ok(shard);
        }).map(|res: Result<Arc<Mutex<Shard>>, Error>| res.unwrap()).collect());

        Ok(())
    }
}