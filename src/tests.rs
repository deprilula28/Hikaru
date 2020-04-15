use crate::client::Client;

fn init() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

#[test]
fn test_babysteps() -> Result<(), crate::util::error::Error> {
    init();

    let env = env_logger::Env::default();

    let client = Client::new(&std::env::var("token").expect("No `token` env variable set"), (0, 1, 1));

    client.heartbeat_thread();
    Ok(())
}