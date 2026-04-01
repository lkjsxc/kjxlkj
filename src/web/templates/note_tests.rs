use super::{note::note_page, NoteAnalytics, NoteChrome};
use crate::web::db::Record;
use chrono::Utc;

fn sample_record() -> Record {
    Record {
        id: "abcdefghijklmnopqrstuvwx26".to_string(),
        alias: Some("demo-note".to_string()),
        title: "Demo".to_string(),
        summary: "Body".to_string(),
        body: "# Demo\n\nBody".to_string(),
        is_favorite: true,
        favorite_position: Some(1),
        is_private: false,
        view_count_total: 3,
        last_viewed_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn sample_chrome() -> NoteChrome {
    NoteChrome {
        id: "abcdefghijklmnopqrstuvwx26".to_string(),
        alias: Some("demo-note".to_string()),
        title: "Demo".to_string(),
        current_href: "/demo-note".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        is_favorite: true,
        visibility: "Public",
        previous: None,
        next: None,
        history_href: "/demo-note/history".to_string(),
    }
}

#[test]
fn guest_note_page_hides_editor() {
    let html = note_page(&sample_record(), &sample_chrome(), None, false);
    assert!(html.contains("shell-rail"));
    assert!(!html.contains("editor-root"));
}

#[test]
fn admin_note_page_renders_alias_and_favorite_controls() {
    let html = note_page(
        &sample_record(),
        &sample_chrome(),
        Some(&NoteAnalytics {
            total: 12,
            views_7d: 4,
            views_30d: 7,
            views_90d: 9,
            last_viewed_at: Some("2026-03-26 08:35 UTC".to_string()),
        }),
        true,
    );
    assert!(html.contains("favorite-toggle"));
    assert!(html.contains("alias-input"));
    assert!(html.contains("preview-toggle"));
    assert!(html.contains("editor-field-card"));
    assert!(html.contains("Views total"));
    assert!(html.contains("2026-03-26 08:35 UTC"));
    assert!(!html.contains("uicdn.toast.com"));
}
