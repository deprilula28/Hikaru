use reqwest::Client;
use reqwest::ClientBuilder;
use url::Url;

pub const DISCORD_BASE_API: &str = "https://discordapp.com/api";

pub struct RestSender {
    client: Client
}

impl RestSender {
    pub fn new() -> RestSender {
        RestSender {
            client: /*ClientBuilder::new().build()*/ Client::new()
        }
    }

    pub async fn get(&self, url: &str) {
        let body = self.client.get(url).send().await;
        println!("GET {}: {:?}", url, body);
    }
/*
    pub async fn post(&self, endpoint: &str, body: &str) {
        let url = format!("{}/{}", DISCORD_BASE_API, endpoint);
        let response = self.client.post(url).body(body).send().await;
        println!("POST {}: {:?}", url, response);
    }
    */
}