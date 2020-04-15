use crate::gateway::shardconnection::Shard;
use crate::gateway::shardconnection::GatewayState;
use crate::util::HikaruResult;
use crate::gateway::op_code;
use crate::gateway::op_code::GatewayPayload;
use crate::shard_log;

pub trait PayloadHandler {
    fn handle_payload(&self, shard: &mut Shard) -> HikaruResult<()>;
}

impl PayloadHandler for op_code::Hello {
    fn handle_payload(&self, shard: &mut Shard) -> HikaruResult<()> {
        debug!("{} Received connection hello, sending identify", shard_log!(shard));
        shard.heartbeat_interval = self.heartbeat_interval;
        shard.state = GatewayState::Connected;
        shard.send_payload(&GatewayPayload::Identify(op_code::Identify {
            token: shard.token.clone(),
            properties: op_code::IdentifyProperties {
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