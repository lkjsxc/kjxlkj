use super::{history::history_page, HistoryLink, NoteChrome};
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
fn history_page_lists_current_note_and_revisions() {
    let html = history_page(
        &sample_record(),
        &sample_chrome(),
        &[HistoryLink {
            href: "/demo-note/history/2".to_string(),
            label: "Revision 2".to_string(),
            created_at: "2026-03-26 08:00 UTC".to_string(),
            status: "Public",
        }],
        Some("prev"),
        Some("next"),
        20,
        false,
    );
    assert!(html.contains("Current note"));
    assert!(html.contains("/demo-note/history/2"));
    assert!(html.contains("Saved 2026-03-26 08:00 UTC"));
    assert!(html.contains(">Previous<"));
    assert!(html.contains(">Next<"));
}
