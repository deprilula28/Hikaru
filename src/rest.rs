use reqwest::Client;
use serde_json::Value;
use crate::util::error::Error;
use crate::util::HikaruResult;

pub const DISCORD_BASE_API: &str = "https://discordapp.com/api";

pub struct RestSender {
    client: Client,
    token: String
}

impl RestSender {
    pub fn new(token: &str) -> RestSender {
        RestSender {
            client: Client::new(),
            token: token.to_string()
        }
    }

    pub async fn get(&self, endpoint: &str) -> HikaruResult<Value> {
        let response = self.client.get(&format!("{}{}", DISCORD_BASE_API, endpoint))
            .header(reqwest::header::USER_AGENT, "Hikaru-Lib REST API")
            .header(reqwest::header::AUTHORIZATION, &self.token)
            .send().await?;
        let json = response.json().await?;
        debug!("GET {}: {:?}", endpoint, json);
        Ok(json)
    }

    pub async fn post(&self, endpoint: &str, body: &str) {
        let response = self.client.post(&format!("{}{}", DISCORD_BASE_API, endpoint)).body(body.to_owned()).send().await;
        debug!("POST {}: {:?}", endpoint, response);
    }
}