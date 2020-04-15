use std::thread;
use std::sync::{Arc, RwLock, Mutex};
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
    pub shards: Option<Arc<Mutex<Vec<Shard>>>>
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
        Client {
            token: token.to_string(),
            rest: RestSender::new(token),
            shards: None,
            shard_info: shards
        }
    }

    pub fn heartbeat_thread(&self) -> ! {
        loop {
            thread::sleep(Duration::from_millis(45 * 1000));
            let mut shards = self.shards.as_ref().unwrap().lock().unwrap();
            // for mut x in shards.iter() {
            //     x.send_payload(&GatewayPayload::Heartbeat(x.seq));
            // }
        }
    }

    pub fn init_shards(&mut self) -> HikaruResult<()> {
        info!("[Shard Manager] Starting {}-{} out of {} shards (ETA {})",
                 self.shard_info.0, self.shard_info.0 + self.shard_info.1, self.shard_info.2,
                 format_time_period(self.shard_info.1 as u64 * 5 * 1000));
        let vector: Arc<Mutex<Vec<Shard>>> = Arc::new(Mutex::new(Vec::with_capacity(self.shard_info.1 as usize)));
        let shard_info = self.shard_info;
        let vector_copy = vector.clone();
        let token_copy = self.token.clone();

        thread::spawn(move || {
            let mut starter_closure = || -> HikaruResult<()> {
                for i in 0..shard_info.1 {
                    let shard = Shard::new(&token_copy, (i + shard_info.0, shard_info.2))?;
                    { vector_copy.lock().unwrap()[i as usize] = shard; }
                    thread::sleep(Duration::from_millis(5 * 1000));
                }
                Ok(())
            };
            match starter_closure() {
                Err(e) => info!("[Shard Manager] An error occured when starting shards {:?}", e),
                Ok(_) => info!("[Shard Manager] Finished starting shards")
            };
        });
        self.shards = Some(vector);

        Ok(())
    }
}