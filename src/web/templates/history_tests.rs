use super::{
    history::{history_page, HistoryPage},
    HistoryLink, NoteChrome,
};
use crate::web::db::Record;
use crate::web::site::SiteContext;
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
        summary: "Body".to_string(),
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

fn sample_site() -> SiteContext {
    SiteContext {
        site_name: "Launchpad".to_string(),
        site_description: "Search-friendly notes.".to_string(),
        public_base_url: Some("https://example.com".to_string()),
    }
}

#[test]
fn history_page_lists_live_note_and_saved_snapshots() {
    let html = history_page(
        &sample_record(),
        &sample_chrome(),
        HistoryPage {
            history: &[HistoryLink {
                href: "/zyxwvutsrqponmlkjihgfedcba".to_string(),
                label: "Latest saved snapshot".to_string(),
                summary: "Saved body".to_string(),
                created_at: "2026-03-26 08:00 UTC".to_string(),
                status: "Public",
            }],
            previous_cursor: Some("prev"),
            next_cursor: Some("next"),
            limit: 20,
        },
        false,
        &sample_site(),
    );
    assert!(html.contains("Live note"));
    assert!(html.contains("Latest saved snapshot"));
    assert!(html.contains("Saved body"));
    assert!(html.contains("/zyxwvutsrqponmlkjihgfedcba"));
    assert!(html.contains("2026-03-26 08:00 UTC"));
    assert!(html.contains(">Prev<"));
    assert!(html.contains(">Next<"));
    assert!(html.contains("<title>History: Demo | Launchpad</title>"));
    assert!(html.contains("content=\"noindex,nofollow\""));
}
