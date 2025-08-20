use chrono::{Datelike, Utc};
use serenity::all::Timestamp;

pub fn format_duration_since(timestamp: Timestamp) -> String {
    let joined = timestamp.naive_utc().date();
    let now = Utc::now().naive_utc().date();

    let mut years = now.year() - joined.year();
    let mut months = now.month() as i32 - joined.month() as i32;

    if months < 0 {
        years -= 1;
        months += 12;
    }

    format!(
        "{} year{}, {} month",
        years,
        if years > 1 { "s" } else { "" },
        months
    )
}
