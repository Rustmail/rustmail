use chrono::{Datelike, Utc};
use serenity::all::Timestamp;

pub fn format_account_age(timestamp: Timestamp) -> String {
    let created = timestamp.naive_utc().date();
    let now = Utc::now().naive_utc().date();

    let mut years = now.year() - created.year();
    let mut months = now.month() as i32 - created.month() as i32;

    if months < 0 {
        years -= 1;
        months += 12;
    }

    format!(
        "{} year{}, {} months",
        years,
        if years > 1 { "s" } else { "" },
        months
    )
}
