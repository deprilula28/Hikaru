#[macro_export]
macro_rules! shard_log_num {
    ($s:expr) => (format!("[Shard ID {}]", $s.0));
}

#[macro_export]
macro_rules! shard_log {
    ($s:expr) => (format!("[Shard ID {}]", $s.shard_info.0));
}