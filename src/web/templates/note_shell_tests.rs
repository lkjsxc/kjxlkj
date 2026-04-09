use super::model::{NavLink, NoteChrome};
use super::note_shell::note_rail;

fn chrome(previous: Option<NavLink>, next: Option<NavLink>) -> NoteChrome {
    NoteChrome {
        id: "demo".to_string(),
        kind: crate::web::db::RecordKind::Note,
        alias: Some("orbit-ledger".to_string()),
        title: "Orbit Ledger".to_string(),
        summary: "Shared release note for the orbit ledger.".to_string(),
        current_href: "/demo".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        is_favorite: true,
        visibility: "Public",
        previous,
        next,
        history_href: "/demo/history".to_string(),
    }
}

#[test]
fn note_rail_keeps_single_history_card() {
    let html = note_rail(&chrome(None, None), true, "/demo");
    let history = html.find("History").unwrap();
    let github = html.find("Open GitHub").unwrap();
    let delete = html.find("Delete note").unwrap();
    let logout = html.find("Logout").unwrap();
    assert!(html.contains("History"));
    assert!(html.contains("Open GitHub"));
    assert!(html.contains("<span>Created</span>"));
    assert!(html.contains("<span>Updated</span>"));
    assert!(history < github && github < delete && delete < logout);
}

#[test]
fn note_rail_renders_disabled_timeline_cards() {
    let html = note_rail(&chrome(None, None), false, "/demo");
    assert!(html.contains("No older accessible resource."));
    assert!(html.contains("No newer accessible resource."));
    assert!(!html.contains("CREATE"));
    assert!(html.contains("timeline-grid"));
}
