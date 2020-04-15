#[macro_export]
macro_rules! shard_log_num {
    ($s:expr) => (ansi_term::Color::Yellow.bold().paint(format!("[Shard ID {}]", $s.0)));
}

#[macro_export]
macro_rules! shard_log {
    ($s:expr) => (match $s.state {
        crate::gateway::shardconnection::GatewayState::Connecting => ansi_term::Color::Yellow,
        crate::gateway::shardconnection::GatewayState::Connected => ansi_term::Color::Green,
        crate::gateway::shardconnection::GatewayState::Disconnected => ansi_term::Color::Red
    }.bold().paint(format!("[Shard ID {}]", $s.shard_info.0)));
}