use reqwest::Client;
use serde_json::Value;
use crate::error::Error;

pub const DISCORD_BASE_API: &str = "https://discordapp.com/api";

pub struct RestSender {
    client: Client,
    token: String
}

impl RestSender {
    pub fn new(token: String) -> RestSender {
        RestSender {
            client: Client::new(),
            token
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Value, Error> {
        let response = self.client.get(&format!("{}{}", DISCORD_BASE_API, endpoint))
            .header(reqwest::header::USER_AGENT, "Hikaru-Lib REST API")
            .header(reqwest::header::AUTHORIZATION, &self.token)
            .send().await?;
        let json = response.json().await?;
        println!("GET {}: {:?}", endpoint, json);
        Ok(json)
    }

    pub async fn post(&self, endpoint: &str, body: &str) {
        let response = self.client.post(&format!("{}{}", DISCORD_BASE_API, endpoint)).body(body.to_owned()).send().await;
        println!("POST {}: {:?}", endpoint, response);
    }
}