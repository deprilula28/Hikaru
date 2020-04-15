use serde::{Deserialize, Serialize};
use serde_json::{Value,json};
use crate::util::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hello {
    pub heartbeat_interval: u64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyProperties,
    pub compress: bool,
    pub shard: (u32, u32),
    pub guild_subscriptions: bool,
    pub intents: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String
}

// https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Debug)]
pub enum GatewayPayload {
    Dispatch(),              // RECEIVE An event was dispatched.
    Heartbeat(u64),          // SEND/RECEIVE Fired periodically by the client to keep the connection alive.
    Identify(Identify),      // SEND Starts a new session during the initial handshake.
    PresenceUpdate(),        // SEND Update the client's presence.
    VoiceStateUpdate(),      // SEND Used to join/leave or move between voice channels.
    Resume(),                // SEND Resume a previous session that was disconnected.
    Reconnect(),             // RECEIVE You must reconnect with a new session immediately.
    RequestGuildMembers(),   // SEND Request information about offline guild members in a large guild.
    InvalidSession(),        // RECEIVE The session has been invalidated. You should reconnect and identify/resume accordingly.
    Hello(Hello),            // RECEIVE Sent immediately after connecting, contains the heartbeat_interval to use.
    HeartbeatACK()           // RECEIVE Sent in response to receiving a heartbeat to acknowledge that it has been received.
}

impl GatewayPayload {
    pub fn serialize(&self) -> Result<Value, Error> {
        match self {
            GatewayPayload::Heartbeat(d) => Ok(json!({ "op": 1, "d": d })),
            GatewayPayload::Identify(d) => Ok(json!({ "op": 2, "d": d })),
            GatewayPayload::PresenceUpdate() => Ok(json!({ "op": 3 })),
            GatewayPayload::VoiceStateUpdate() => Ok(json!({ "op": 4 })),
            GatewayPayload::Resume() => Ok(json!({ "op": 6 })),
            GatewayPayload::RequestGuildMembers() => Ok(json!({ "op": 7 })),
            _ => Err(Error::Text(String::from("Invalid gateway request, sending receive-only opcode")))
        }
    }

    pub fn deserialize(value: Value) -> Result<Self, Error> {
        if let Value::Number(code) = &value["op"] {
            let d = &value["d"];
            match code.as_u64() {
                Some(0) => Ok(GatewayPayload::Dispatch()),
                Some(1) => Ok(GatewayPayload::Heartbeat(if let Some(v) = d.as_u64() { v } else { return Err(Error::Text(String::from("Invalid gateway request, d not a number for op 1 heartbeat"))) })), // TODO make this less shit
                Some(7) => Ok(GatewayPayload::Reconnect()),
                Some(9) => Ok(GatewayPayload::InvalidSession()),
                Some(10) => Ok(GatewayPayload::Hello(Hello::deserialize(d)?)),
                Some(11) => Ok(GatewayPayload::HeartbeatACK()),
                _ => Err(Error::Text(String::from("Invalid gateway request, received send-only opcode")))
            }
        } else { Err(Error::Text(String::from("Invalid gateway request, opcode not a number"))) }
    }
}