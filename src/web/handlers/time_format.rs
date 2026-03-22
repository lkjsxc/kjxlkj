use chrono::{DateTime, Utc};

pub fn format_utc_timestamp(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%d %H:%M UTC").to_string()
}
