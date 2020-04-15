use serde::Deserialize;
use serde_json::Value;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Hello {
    heartbeat_interval: u64
}

#[derive(Debug)]
pub enum GatewayOpcode {
    Dispatch(),              // RECEIVE An event was dispatched.
    Heartbeat(),             // SEND/RECEIVE Fired periodically by the client to keep the connection alive.
    Identify(),              // SEND Starts a new session during the initial handshake.
    PresenceUpdate(),        // SEND Update the client's presence.
    VoiceStateUpdate(),      // SEND Used to join/leave or move between voice channels.
    Resume(),                // SEND Resume a previous session that was disconnected.
    Reconnect(),             // RECEIVE You must reconnect with a new session immediately.
    RequestGuildMembers(),   // SEND Request information about offline guild members in a large guild.
    InvalidSession(),        // RECEIVE The session has been invalidated. You should reconnect and identify/resume accordingly.
    Hello(Hello),            // RECEIVE Sent immediately after connecting, contains the heartbeat_interval to use.
    HeartbeatACK()           // RECEIVE Sent in response to receiving a heartbeat to acknowledge that it has been received.
}

impl GatewayOpcode {
    pub fn decode(json: &Value) -> Result<GatewayOpcode, Error> {
        if let Value::Number(code) = &json["op"] {
            let d = &json["d"];
            return match code.as_u64() {
                Some(0) => Ok(GatewayOpcode::Dispatch()),
                Some(1) => Ok(GatewayOpcode::Heartbeat()),
                Some(2) => Ok(GatewayOpcode::Identify()),
                Some(3) => Ok(GatewayOpcode::PresenceUpdate()),
                Some(4) => Ok(GatewayOpcode::VoiceStateUpdate()),
                Some(6) => Ok(GatewayOpcode::Resume()),
                Some(7) => Ok(GatewayOpcode::Reconnect()),
                Some(8) => Ok(GatewayOpcode::RequestGuildMembers()),
                Some(9) => Ok(GatewayOpcode::InvalidSession()),
                Some(10) => Ok(GatewayOpcode::Hello(Hello::deserialize(d)?)),
                Some(11) => Ok(GatewayOpcode::HeartbeatACK()),
                _ => Err(Error::Text(String::from("Invalid gateway request, opcode not a number")))
            }
        } else { Err(Error::Text(String::from("Invalid gateway request, opcode not a number"))) }
    }
}