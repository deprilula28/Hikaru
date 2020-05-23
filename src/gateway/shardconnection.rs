use async_tungstenite::{tokio::TokioAdapter, WebSocketStream, stream::Stream, tokio::connect_async};
use async_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use async_tungstenite::tungstenite::Message;
use flate2::read::ZlibDecoder;
use serde_json::Value;

use std::convert::TryFrom;
use std::sync::{Arc};
use std::time::{Duration, Instant};

use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use futures_util::{StreamExt, SinkExt};
use futures_util::lock::{Mutex};

use crate::gateway::op_code::GatewayPayload;
use crate::util::error::Error;
use crate::util::error::Error::GatewayError;
use crate::gateway::close_code::GatewayCloseCode;
use crate::util::HikaruResult;
use crate::gateway::gatewaypayloadhandler::PayloadHandler;
use crate::{ shard_log_num };

const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg"; // Discord told us to cache the result from one of the requests, but it literally only returns this so... I guess it's cached in my code?
const GATEWAY_VERSION: u8 = 6;

#[derive(PartialEq)]
pub enum GatewayState {
    Connecting,
    Connected,
    Reconnecting,
    Resuming,
    Disconnected
}

pub struct Shard {
    url: String,
    pub(crate) socket: WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TokioAdapter<TokioAdapter<TcpStream>>>>>>,
    session_id: Option<String>,
    last_heartbeat: Option<Instant>,
    pub(crate) token: String,
    pub(crate) seq: Option<u64>,
    pub ping: Duration,
    pub init: Instant,
    pub state: GatewayState,
    pub heartbeat_interval: u64,
    pub shard_info: (u32, u32)
}

impl Shard {
    pub async fn new(token: &str, shards: (u32, u32)) -> HikaruResult<Shard> {
        info!("{} Shard is initializing", shard_log_num!(shards));
        let owned_token = token.to_string();
        let url = format!("{}/?v={}", DISCORD_GATEWAY, GATEWAY_VERSION);

        let (socket, _response) = connect_async(&url).await?;
        Ok(Shard {
            url,
            socket,
            token: owned_token,
            init: Instant::now(),
            state: GatewayState::Connecting,

            ping: Duration::new(0, 0),
            heartbeat_interval: 45000, // Default
            shard_info: shards,
            session_id: None,
            seq: None,
            last_heartbeat: None
        })
    }

    pub async fn reconnect(&mut self, resume: bool) -> HikaruResult<()> {
        let (socket, _response) = connect_async(&self.url).await?;
        self.socket = socket;
        if resume {
            let payload = &GatewayPayload::Resume(&self.token, &self.session_id.as_ref().unwrap(), self.seq.unwrap());
            let json = serde_json::to_string(&payload.serialize()?)?;
            self.socket.send(Message::Text(json)).await?
        }
        Ok(())
    }

    pub async fn send_payload(&mut self, payload: &GatewayPayload<'_>) -> HikaruResult<()> {
        debug!("Sending payload {:?}", payload);
        let json = serde_json::to_string(&payload.serialize()?)?;
        Ok(self.socket.send(Message::Text(json)).await?)
        // Ok(())
    }

    pub async fn shard_loop(shard_lock: &Arc<Mutex<Shard>>) -> HikaruResult<()> {
        loop {
            let shard_info;
            let next;
            {
                let mut shard = shard_lock.lock().await;
                shard_info = shard.shard_info;
                next = shard.socket.next().await.unwrap();
            }
            let decoded_msg_opt: Option<Value> = match next {
                Ok(msg) => match msg {
                    Message::Binary(bytes) => Some(serde_json::from_reader(ZlibDecoder::new(&bytes[..]))?),
                    Message::Text(text) => Some(serde_json::from_str(&text)?),
                    Message::Close(close) => {
                        {
                            let mut shard = shard_lock.lock().await;
                            shard.state = GatewayState::Disconnected;
                        }
                        debug!("{} Connection was closed {:?}", shard_log_num!(shard_info), close);
                        return match close {
                            Some(frame) => match frame.code {
                                CloseCode::Library(code) => Err(GatewayError(GatewayCloseCode::try_from(code)?)),
                                CloseCode::Normal => Err(GatewayError(GatewayCloseCode::TimeOut)),
                                _ => Err(GatewayError(GatewayCloseCode::UnknownCloseCode))
                            },
                            _ => Err(GatewayError(GatewayCloseCode::UnknownCloseCode))
                        }
                    }
                    unknown_type => {
                        warn!("{} Received message of unknown type: {:?}", shard_log_num!(shard_info), unknown_type);
                        None
                    }
                },
                Err(err) => {
                    warn!("{} Received error when reading {:?}", shard_log_num!(shard_info), err);
                    None
                }
            };
            if let Some(decoded_message) = decoded_msg_opt {
                println!("Decoded packet is {:?}", decoded_message);
                let payload = GatewayPayload::deserialize(decoded_message)?;
                match payload {
                    GatewayPayload::Hello(hello) => {
                        let mut shard = shard_lock.lock().await;
                        hello.handle_payload(&mut shard).await?;
                    },
                    GatewayPayload::Heartbeat(seq) => shard_lock.lock().await.seq = seq,
                    
                    GatewayPayload::Dispatch(seq, event) => {
                        
                    },
                    
                    GatewayPayload::Reconnect() => {
                        let mut shard = shard_lock.lock().await;
                        (&mut shard).state = GatewayState::Resuming;
                        return Err(GatewayError(GatewayCloseCode::Reconnect))
                    },
                    GatewayPayload::InvalidSession() => {
                        let mut shard = shard_lock.lock().await;
                        (&mut shard).state = GatewayState::Reconnecting;
                        return Err(GatewayError(GatewayCloseCode::Reconnect))
                    },
                    GatewayPayload::HeartbeatACK() => {
                        debug!("Heartbeat acknowledged.");
                        let mut shard = shard_lock.lock().await;
                        if let Some(last_heartbeat) = shard.last_heartbeat {
                            shard.ping = last_heartbeat.elapsed();
                        }
                    },
                    _ => return Err(Error::Text(format!("Invalid gateway payload received {:?}", payload)))
                }
            }
        }
    }
}