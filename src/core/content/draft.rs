use chrono::{DateTime, Datelike, Timelike, Utc};

pub fn draft_title_and_slug(now: DateTime<Utc>) -> (String, String) {
    let title = format!(
        "Draft {:04}-{:02}-{:02} {:02}:{:02} UTC",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );
    let slug = format!(
        "draft-{:04}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    (title, slug)
}
