use tokio;
use hikaru::client::Client;
use hikaru::util::error::Error;

fn init() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {    
    init();

    env_logger::Env::default();
    let mut client = Client::new(&std::env::var("token").expect("No `token` env variable set"), (0, 1, 1));
    client.init_shards().await?;
    client.heartbeat_thread().await?;
    Ok(())
}