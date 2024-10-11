use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_timestamp_ms() {
        let timestamp = get_current_timestamp_ms();
        assert!(timestamp > 0);
    }
}