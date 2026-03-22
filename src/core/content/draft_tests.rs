use chrono::{TimeZone, Utc};

use super::draft_title_and_slug;

#[test]
fn draft_placeholder_uses_expected_formats() {
    let now = Utc.with_ymd_and_hms(2026, 3, 22, 8, 36, 32).unwrap();
    let (title, slug) = draft_title_and_slug(now);
    assert_eq!(title, "Draft 2026-03-22 08:36 UTC");
    assert_eq!(slug, "draft-20260322083632");
}

#[test]
fn draft_placeholder_rolls_over_at_new_year_in_utc() {
    let now = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
    let (title, slug) = draft_title_and_slug(now);
    assert_eq!(title, "Draft 2027-01-01 00:00 UTC");
    assert_eq!(slug, "draft-20270101000000");
}
