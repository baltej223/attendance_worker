use chrono::{Datelike, Local, Weekday};
use std::{
    alloc::System,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn get_current_time_hhmm() -> String {
    let now = Local::now();
    now.format("%H:%M").to_string()
}

pub fn get_time() -> (Duration, SystemTime) {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    (since_epoch, now)
}

pub fn compare_day(day: String) -> bool {
    let given_day = match day.to_lowercase().as_str() {
        "mon" | "monday" => Weekday::Mon,
        "tue" | "tuesday" => Weekday::Tue,
        "wed" | "wednesday" => Weekday::Wed,
        "thu" | "thursday" => Weekday::Thu,
        "fri" | "friday" => Weekday::Fri,
        "sat" | "saturday" => Weekday::Sat,
        "sun" | "sunday" => Weekday::Sun,
        _ => return false, // invalid input
    };

    let today = Local::now().weekday();
    given_day == today
}
