use super::{search::search_page, IndexItem};

fn sample_item() -> IndexItem {
    IndexItem {
        href: "/q29udhjy3rsdw50aw1lmq".to_string(),
        title: "Orbit Ledger".to_string(),
        summary: "Shared release.".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        is_favorite: true,
        visibility: Some("Public"),
    }
}

#[test]
fn search_page_browses_without_query() {
    let html = search_page(
        &[sample_item()],
        None,
        Some("cursor"),
        None,
        20,
        "updated_desc",
        false,
    );
    assert!(html.contains(">All notes<"));
    assert!(html.contains(">Query<"));
    assert!(html.contains("name=\"sort\""));
    assert!(html.contains("value=\"updated_desc\" selected"));
    assert!(html.contains(">Previous<"));
    assert!(html.contains(">Next<"));
}

#[test]
fn search_page_keeps_query_and_sort_in_form() {
    let html = search_page(
        &[sample_item()],
        Some("prev"),
        Some("cursor"),
        Some("orbit"),
        20,
        "relevance",
        true,
    );
    assert!(html.contains("name=\"q\" value=\"orbit\""));
    assert!(html.contains("value=\"relevance\" selected"));
    assert!(html.contains("name=\"direction\" value=\"prev\""));
    assert!(html.contains("New note"));
}
