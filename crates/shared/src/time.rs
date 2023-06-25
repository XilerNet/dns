use std::time::SystemTime;

pub fn system_time_from_epoch_seconds(seconds: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds)
}
