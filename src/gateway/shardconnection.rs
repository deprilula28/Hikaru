use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::AutoStream;
use std::time::Instant;
use flate2::read::ZlibDecoder;
use serde_json::Value;
use tungstenite::protocol::frame::coding::CloseCode;
use std::convert::TryFrom;

use crate::gateway::op_code::GatewayPayload;
use crate::util::error::Error;
use crate::util::error::Error::GatewayError;
use crate::gateway::close_code::close_code;
use crate::util::HikaruResult;
use crate::gateway::gatewaypayloadhandler::PayloadHandler;

use crate::{ shard_log, shard_log_num };

const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg"; // Discord told us to cache the result from one of the requests, but it literally only returns this so... I guess it's cached in my code?
const GATEWAY_VERSION: u8 = 6;

pub enum GatewayState {
    Connecting,
    Connected,
    Disconnected
}

pub struct Shard {
    url: String,
    socket: WebSocket<AutoStream>,
    pub(crate) token: String,
    pub(crate) seq: Option<u64>,
    pub init: Instant,
    pub state: GatewayState,
    pub heartbeat_interval: u64,
    pub shard_info: (u32, u32)
}

impl Shard {
    pub fn new(token: &str, shards: (u32, u32)) -> HikaruResult<Shard> {
        info!("{} Shard is initializing", shard_log_num!(shards));
        let owned_token = token.to_string();
        let url = format!("{}/?v={}", DISCORD_GATEWAY, GATEWAY_VERSION);
        let (mut socket, _response) = connect(&url)?;

        Ok(Shard {
            url,
            socket,
            token: owned_token,
            seq: None,
            init: Instant::now(),
            state: GatewayState::Connecting,
            heartbeat_interval: 45000, // Default
            shard_info: shards
        })
    }

    pub fn send_payload(&mut self, payload: &GatewayPayload) -> HikaruResult<()> {
        let json = serde_json::to_string(&payload.serialize()?)?;
        Ok(self.socket.write_message(Message::Text(json))?)
    }

    pub fn shard_loop(&mut self) -> HikaruResult<()> {
        loop {
            let mut message_closure = || -> HikaruResult<()> {
                let decoded_msg_opt: Option<Value> = match self.socket.read_message()? {
                    Message::Binary(bytes) => {
                        debug!("{} Received binary payload {:?}", shard_log!(self), bytes);
                        Some(serde_json::from_reader(ZlibDecoder::new(&bytes[..]))?)
                    }
                    Message::Text(text) => {
                        debug!("{} Received text payload {:?}", shard_log!(self), text);
                        Some(serde_json::from_str(&text)?)
                    }
                    Message::Close(close) => {
                        self.state = GatewayState::Disconnected;
                        error!("{} Connection was closed {:?}", shard_log!(self), close);
                        return match close {
                            Some(frame) => match frame.code {
                                CloseCode::Library(code) => Err(GatewayError(close_code::try_from(code)?)),
                                CloseCode::Normal => Err(GatewayError(close_code::TimeOut)),
                                _ => Err(GatewayError(close_code::UnknownCloseCode))
                            },
                            _ => Err(GatewayError(close_code::UnknownCloseCode))
                        }
                    }
                    msg => {
                        info!("{} Received message: {:?}", shard_log!(self), msg);
                        None
                    }
                };
                if let Some(decoded_message) = decoded_msg_opt {
                    let payload = GatewayPayload::deserialize(decoded_message)?;
                    match payload {
                        GatewayPayload::Hello(hello) => hello.handle_payload(self)?,
                        GatewayPayload::Dispatch() => {},
                        GatewayPayload::Heartbeat(seq) => {},
                        GatewayPayload::Reconnect() => return Err(GatewayError(close_code::Reconnect)),
                        GatewayPayload::InvalidSession() => {},
                        GatewayPayload::HeartbeatACK() => {},
                        _ => return Err(Error::Text(format!("Invalid gateway payload received {:?}", payload)))
                    }
                }
                Ok(())
            };

            if let Err(err) = message_closure() { return Err(err) }
        }
    }
}