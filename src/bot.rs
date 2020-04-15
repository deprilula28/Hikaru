use crate::gateway::shardconnection::Shard;
use crate::rest::RestSender;
use ansi_term::Color;
use crate::util::{format_time_period, HikaruResult};
use std::thread;
use std::sync::{Arc, RwLock, Mutex};
use std::time::Duration;
use crate::gateway::gatewayopcode::GatewayPayload;
use serde_json::{Value, Number};
use crate::util::error::Error;

pub struct Bot {
    token: String,
    shard_info: (u32, u32, u32),
    pub rest: RestSender,
    pub shards: Option<Arc<Mutex<Vec<Shard>>>>
}

impl Bot {
    // fn new_flexible(token: &String) -> Bot {
    //     let rest = RestSender::new(token);
    //     Bot {
    //         token: token.clone(),
    //         rest,
    //         shards: init_shards()
    //     }
    // }

    pub fn new(token: &String, shards: (u32, u32, u32)) -> Bot {
        Bot {
            token: token.clone(),
            rest: RestSender::new(token),
            shards: None,
            shard_info: shards
        }
    }

    pub fn heartbeat_thread(&self) -> HikaruResult<()> {
        loop {
            thread::sleep(Duration::from_millis(45 * 1000));
            let mut shards = self.shards.as_ref().unwrap().lock().unwrap();
            // for mut x in shards.iter() {
            //     x.send_payload(&GatewayPayload::Heartbeat(x.seq));
            // }
        }
        Ok(()) // Should never be reached but compiler is annoying
    }

    pub fn init_shards(&mut self) -> HikaruResult<()> {
        println!("{} Starting {}-{} out of {} shards (ETA {})",
                 Color::Cyan.bold().paint("[Shard Manager]"),
                 self.shard_info.0, self.shard_info.0 + self.shard_info.1, self.shard_info.2,
                 format_time_period(self.shard_info.1 as u64 * 5 * 1000));
        let vector: Arc<Mutex<Vec<Shard>>> = Arc::new(Mutex::new(Vec::with_capacity(self.shard_info.1 as usize)));
        let shard_info_copy = self.shard_info.clone();
        let vector_copy = vector.clone();
        let token_copy = self.token.clone();

        thread::spawn(move || {
            let mut starter_closure = || -> HikaruResult<()> {
                for i in 0..shard_info_copy.1 {
                    let shard = Shard::new(&token_copy, (i + shard_info_copy.0, shard_info_copy.2))?;
                    { vector_copy.lock().unwrap()[i as usize] = shard; }
                    thread::sleep(Duration::from_millis(5 * 1000));
                }
                Ok(())
            };
            match starter_closure() {
                Err(e) => println!("{} {} {:?}",
                       Color::Cyan.bold().paint("[Shard Manager]"),
                       Color::Red.bold().paint("An error occured when starting shards"), e),
                Ok(_) => println!("{} Finished starting shards",
                      Color::Cyan.bold().paint("[Shard Manager]"))
            };
        });
        self.shards = Some(vector);

        Ok(())
    }
}