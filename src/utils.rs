use chrono::{Utc, TimeZone};

/// Generates a unique ID (UUID v4)
pub fn generate_unique_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Returns the current UNIX timestamp
pub fn current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

/// Converts a UNIX timestamp to a readable date string
pub fn format_timestamp(timestamp: u64) -> String {
    Utc.timestamp_opt(timestamp as i64, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string()
}
