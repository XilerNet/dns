use std::time::SystemTime;

/// Converts a u64 integer representing seconds since the Unix epoch to a `SystemTime` object.
///
/// # Arguments
///
/// * `seconds` - The number of seconds since the Unix epoch.
///
/// # Returns
///
/// A `SystemTime` object representing the timestamp.
pub fn system_time_from_epoch_seconds(seconds: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds)
}
