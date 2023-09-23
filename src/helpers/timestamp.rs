use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    timestamp
}
