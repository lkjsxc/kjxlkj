use super::card_frame::{card_body, card_meta, linked_card, meta_line, static_card};
use super::layout::html_escape;
use super::model::{NavLink, ResourceAnalytics, ResourceChrome};
use crate::web::db::{Resource, ResourceKind};

pub fn note_focus_strip(resource: &Resource, chrome: &ResourceChrome) -> String {
    if resource.kind != ResourceKind::Note {
        return String::new();
    }
    format!(
        r#"<section class="note-nav-strip">{}{}</section>
<section class="surface note-live-strip">
<p class="page-summary" data-live-summary>{}</p>
<div class="note-live-facts">
<span class="status-pill" data-live-visibility>{}</span>
{}
{}
<small><span>URL</span><a href="{}" data-current-resource-link>{}</a></small>
</div>
</section>"#,
        timeline_card(
            chrome.previous.as_ref(),
            "Prev",
            "No older accessible resource."
        ) + &linked_card(
            &chrome.history_href,
            " data-history-link",
            "summary-card note-nav-card",
            &card_body("History", "Open saved snapshots."),
            &card_meta("", &meta_line("Open", "Saved snapshots")),
        ),
        timeline_card(
            chrome.next.as_ref(),
            "Next",
            "No newer accessible resource."
        ),
        html_escape(&chrome.summary),
        chrome.visibility,
        meta_line("Created", &chrome.created_at),
        meta_line("Updated", &chrome.updated_at),
        html_escape(&chrome.current_href),
        html_escape(&chrome.current_href),
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
            "summary-card note-nav-card",
            &card_body(label, &link.title),
            &card_meta("", &meta_line("Created", &link.created_at)),
        ),
        None => static_card(
            r#" aria-disabled="true""#,
            "summary-card note-nav-card summary-card-muted",
            &card_body(label, empty),
            &card_meta("", ""),
        ),
    }
}
