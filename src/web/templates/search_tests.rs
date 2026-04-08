use super::{
    search::{search_page, SearchView},
    IndexItem,
};
use crate::web::site::SiteContext;

fn sample_item() -> IndexItem {
    IndexItem {
        id: "q29udhjy3rsdw50aw1lmq26".to_string(),
        href: "/q29udhjy3rsdw50aw1lmq".to_string(),
        title: "Orbit Ledger".to_string(),
        summary: "Shared release.".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        is_favorite: true,
        visibility: Some("Public"),
        metrics: Vec::new(),
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
fn search_page_browses_without_query() {
    let html = search_page(SearchView {
        notes: &[sample_item()],
        previous_cursor: None,
        next_cursor: Some("cursor"),
        query: None,
        limit: 20,
        scope: "all",
        sort: "updated_desc",
        popular_window: "30d",
        is_admin: false,
        site: &sample_site(),
    });
    assert!(html.contains(">Notes<"));
    assert!(!html.contains(">Query<"));
    assert!(html.contains("name=\"sort\""));
    assert!(html.contains("aria-label=\"Sort\""));
    assert!(!html.contains("<span>Sort</span>"));
    assert!(html.contains("value=\"updated_desc\" selected"));
    assert!(html.contains(">Prev<"));
    assert!(html.contains(">Next<"));
    assert!(html.contains("<title>Search | Launchpad</title>"));
    assert!(html.contains("content=\"noindex,nofollow\""));
}

#[test]
fn search_page_keeps_query_and_sort_in_form() {
    let html = search_page(SearchView {
        notes: &[sample_item()],
        previous_cursor: Some("prev"),
        next_cursor: Some("cursor"),
        query: Some("orbit"),
        limit: 20,
        scope: "all",
        sort: "relevance",
        popular_window: "30d",
        is_admin: true,
        site: &sample_site(),
    });
    assert!(html.contains("name=\"q\" value=\"orbit\""));
    assert!(html.contains(">Query<"));
    assert!(html.contains("value=\"relevance\" selected"));
    assert!(html.contains("name=\"direction\" value=\"prev\""));
    assert!(html.contains("New note"));
}
