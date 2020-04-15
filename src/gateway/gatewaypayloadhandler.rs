use ansi_term::Color;

use crate::gateway::shardconnection::Shard;
use crate::gateway::shardconnection::GatewayState;
use crate::util::HikaruResult;
use crate::gateway::gatewayopcode;
use crate::gateway::gatewayopcode::GatewayPayload;

pub trait PayloadHandler {
    fn handle_payload(&self, shard: &mut Shard) -> HikaruResult<()>;
}

impl PayloadHandler for gatewayopcode::Hello {
    fn handle_payload(&self, shard: &mut Shard) -> HikaruResult<()> {
        println!("{} Received connection hello, sending identify", shard_log!(shard));
        shard.heartbeat_interval = self.heartbeat_interval;
        shard.state = GatewayState::Connected;
        shard.send_payload(&GatewayPayload::Identify(gatewayopcode::Identify {
            token: shard.token.clone(),
            properties: gatewayopcode::IdentifyProperties {
                os: String::from(std::env::consts::OS),
                browser: String::from("HikaruLib"),
                device: String::from("HikaruLib")
            },
            compress: false,
            shard: (0, 1),
            guild_subscriptions: false,
            intents: 0
        }))
    }
}