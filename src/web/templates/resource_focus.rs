use super::card_frame::{card_body, card_meta, linked_card, static_card};
use super::model::{NavLink, ResourceAnalytics, ResourceChrome};

pub fn live_resource_nav_strip(chrome: &ResourceChrome, is_admin: bool) -> String {
    format!(
        r#"<section class="resource-nav-strip">{}{}{}</section>"#,
        timeline_card(chrome.previous.as_ref(), "Prev", "No older accessible resource."),
        history_card(chrome, is_admin),
        timeline_card(chrome.next.as_ref(), "Next", "No newer accessible resource."),
    )
}

pub fn analytics_block(analytics: Option<&ResourceAnalytics>) -> String {
    let Some(analytics) = analytics else {
        return String::new();
    };
    format!(
        r#"<section class="surface resource-analytics-grid">
<article><small>Views total</small><strong>{}</strong></article>
<article><small>Views 1d</small><strong>{}</strong></article>
<article><small>Views 7d</small><strong>{}</strong></article>
<article><small>Views 30d</small><strong>{}</strong></article>
<article><small>Views 90d</small><strong>{}</strong></article>
<article><small>Last viewed</small><strong>{}</strong></article>
</section>"#,
        analytics.total,
        analytics.views_1d,
        analytics.views_7d,
        analytics.views_30d,
        analytics.views_90d,
        analytics
            .last_viewed_at
            .clone()
            .unwrap_or_else(|| "Never".to_string()),
    )
}

fn timeline_card(link: Option<&NavLink>, label: &str, empty: &str) -> String {
    match link {
        Some(link) => linked_card(
            &link.href,
            "",
            "summary-card resource-nav-card",
            &card_body(label, &link.title),
            &card_meta("", ""),
        ),
        None => static_card(
            r#" aria-disabled="true""#,
            "summary-card resource-nav-card summary-card-muted",
            &card_body(label, empty),
            &card_meta("", ""),
        ),
    }
}

fn history_card(chrome: &ResourceChrome, is_admin: bool) -> String {
    if !is_admin {
        return String::new();
    }
    linked_card(
        &chrome.history_href,
        " data-history-link",
        "summary-card resource-nav-card",
        &card_body("History", ""),
        &card_meta("", ""),
    )
}
