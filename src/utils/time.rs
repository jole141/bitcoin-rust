use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}