use flate2::read::ZlibDecoder;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::AutoStream;
use serde_json::Value;
use crate::error::Error;
use crate::gateway::gatewayopcode;
use std::time::Instant;
use crate::gateway::gatewayopcode::GatewayOpcode;

const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg"; // Discord told us to cache the result from one of the requests, but it literally only returns this so... I guess it's cached in my code?
const GATEWAY_VERSION: u8 = 6;

pub struct Shard {
    url: String,
    socket: WebSocket<AutoStream>,
    token: String,
    init: Instant
}

impl Shard {
    pub fn new(token: &str, shard_id: u32, shard_total: u32) -> Result<Shard, Error> {
        println!("Shard is initializing ({}/{})", shard_id, shard_total);
        let owned_token = token.to_owned();
        let url = format!("{}/?v={}", DISCORD_GATEWAY, GATEWAY_VERSION);
        let (mut socket, _response) = connect(&url)?;

        Ok(Shard {
            url,
            socket,
            token: owned_token,
            init: Instant::now()
        })
    }
}

pub fn shard_loop(shard: &mut Shard) {
    loop {
        let mut message_closure = || -> Result<(), Error> {
            let decoded_msg_opt: Option<Value> = match shard.socket.read_message()? {
                Message::Binary(bytes) => {
                    Some(serde_json::from_reader(ZlibDecoder::new(&bytes[..]))?)
                }
                Message::Text(text) => {
                    Some(serde_json::from_str(&text)?)
                }
                msg => {
                    println!("Received message: {:?}", msg);
                    None
                }
            };
            if let Some(decoded_message) = decoded_msg_opt {
                println!("{:?}", GatewayOpcode::decode(&decoded_message)?);
            }
            Ok(())
        };
        match message_closure() {
            Err(e) => {
                println!("Error decoding message: {:?}", e);
                break;
            },
            Ok(_) => {}
        };
    }
}

/*
pub struct GatewayMessage {
    opcode: u32,
    d: Value, 
}

impl GatewayMessage {
    fn send(&self, shard: &Shard) {

    }
}

fn decode(shard: &Shard, message: &str) -> GatewayMessage {

}
pub struct Shard {
    connection: Sender,
    zlibEncoder: ZlibEncoder<
}

impl Shard {
    pub fn new(token: &str) -> Shard {
        Shard {
            connection:
            zlibEncoder: ZlibEncoder::new(Vec::new(), Compression::default())
        }
    }
}
*/

/*
pub fn random_function() {    /*
    connect(url, |out: Sender|
        move |msg: Message| {
            println!("got message: {:?}", msg);
        })?;
        */
    let (mut socket, response) = connect(Url::parse(
        format!("{}/?v={}&encoding=json&compress=zlib-stream", DISCORD_GATEWAY, GATEWAY_VERSION).as_str())
            .unwrap()).expect("Failed to connect");

    socket.set_config(|c| {
        c.max_frame_size = None;
        c.max_message_size = None;
    });
    /*
    let stream_impl = match socket.get_mut() {
        Stream::Plain(stream) => stream,
        Stream::Tls(stream) => stream.get_mut(),
    };
    stream_impl.set_read_timeout(Some(StdDuration::from_millis(600)))?;
    stream_impl.set_write_timeout(Some(StdDuration::from_millis(50)))?;*/

    println!("Connected successfully, response: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, value) in response.headers() {
        println!("- {}: {:?}", header, value);
    }

    // Send identify
    serde_json::to_string(&json!({
        "op": 2,
        "d": {
            "compress": true,
            "token": "Bot token",
            "properties": {
                "$browser": "hikaru",
                "$device": "hikaru",
                "$os": "windows"
            }
        }
    })).map(Message::Text).and_then(|msg: Message| Ok(socket.write_message(msg).map_err(Error::from)));
    println!("Sent identify");

    loop {
        let msg = socket.read_message().expect("Failed to read message");
        match msg {
            Message::Binary(vec) => {
                /*
                let mut decoder = ZlibDecoder::new(Cursor::new(vec));
                let mut string_buffer = String::new();
                decoder.read_to_string(&mut string_buffer).unwrap();
                println!("Received: {}", string_buffer);
                */
                println("Received binary: {}", vec);
            },
            Message::Text(text) => {
                println!("Received text: {}", text)
            },
            _ => {}
        }
    }
}
*/