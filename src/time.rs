use chrono::Local;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_current_time_hhmm() -> String {
    let now = Local::now();
    now.format("%H:%M").to_string()
}

pub fn get_time() -> (Duration, SystemTime) {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    (since_epoch, now)
}
