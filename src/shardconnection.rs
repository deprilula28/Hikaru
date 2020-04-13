use flate2::read::ZlibDecoder;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::AutoStream;
use tungstenite::stream::Stream;
use url::Url;
use std::io::{Cursor, Read};
use serde_json::json;
use std::error::Error;

const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg";
const GATEWAY_VERSION: u8 = 6;

pub struct ShardConnection {
    socket: WebSocket<AutoStream>,
    token: String
}

impl ShardConnection {
    pub fn new(token: &str, shardId: u32, shardTotal: u32) {
        // let mut socket = connect(Url::parse(
        //     format!("{}/?v={}&encoding=json&compress=zlib-stream", DISCORD_GATEWAY, GATEWAY_VERSION).as_str())
        //     .unwrap()).expect("Failed to connect");
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