use super::{
    popular_sections::{admin_popular_section, home_popular_section},
    IndexItem, IndexMetric,
};
use crate::web::db::PopularWindow;

fn sample_item() -> IndexItem {
    IndexItem {
        id: "q29udhjy3rsdw50aw1lmq26".to_string(),
        href: "/q29udhjy3rsdw50aw1lmq".to_string(),
        title: "Orbit Ledger".to_string(),
        summary: "Shared release.".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        kind_badge: "Note",
        image_href: None,
        is_favorite: true,
        visibility: Some("Public"),
        metrics: vec![IndexMetric {
            label: "30d views".to_string(),
            value: "9".to_string(),
        }],
    }
}

#[test]
fn home_popular_section_marks_surface_and_active_window() {
    let html = home_popular_section(&[sample_item()], PopularWindow::Days30);
    assert!(html.contains(">Popular<"));
    assert!(html.contains(r#"data-popular-surface="home""#));
    assert!(html.contains(r#"data-popular-window="30d" aria-pressed="true""#));
    assert!(html.contains("/search?sort=popular_desc&popular_window=30d"));
}

#[test]
fn admin_popular_section_uses_button_controls() {
    let html = admin_popular_section(&[sample_item()], PopularWindow::Days7);
    assert!(html.contains(r#"data-popular-surface="admin""#));
    assert!(html.contains(r#"type="button" class="btn btn-primary" data-popular-window="7d""#));
    assert!(html.contains(r#"data-popular-error hidden"#));
}
