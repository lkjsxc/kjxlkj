//! History page templates

use super::card_frame::{
    card_body, card_meta, created_updated_lines, linked_card, meta_line, status_pill,
};
use super::index::pager;
use super::layout::{base, html_escape, shell_page};
use super::model::{HistoryLink, ResourceChrome};
use super::resource_media::snapshot_media_block;
use super::resource_shell::resource_rail;
use super::resource_words::{live_label, open_live_label};
use super::sections::page_header;
use crate::web::db::{Resource, ResourceKind, ResourceSnapshot};
use crate::web::site::SiteContext;

pub struct HistoryPage<'a> {
    pub history: &'a [HistoryLink],
    pub previous_cursor: Option<&'a str>,
    pub next_cursor: Option<&'a str>,
    pub limit: i64,
}

pub fn history_page(
    resource: &Resource,
    chrome: &ResourceChrome,
    page: HistoryPage<'_>,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let cards = if page.history.is_empty() {
        r#"<p class="surface-empty">No saved snapshots yet.</p>"#.to_string()
    } else {
        page.history
            .iter()
            .map(history_row)
            .collect::<Vec<_>>()
            .join("")
    };
    let content = format!(
        "{}<section class=\"stack resource-list\">{}</section><section class=\"section-block history-list-section\"><div class=\"section-head\"><h2>Saved snapshots</h2></div><div class=\"resource-list\">{}</div>{}</section>",
        page_header(
            &format!("History: {}", chrome.title),
            Some(&format!(
                r#"<a href="{}" class="btn">{}</a>"#,
                chrome.current_href,
                open_live_label(chrome.kind),
            )),
            "history-head",
        ),
        live_row(resource, chrome),
        cards,
        pager(
            &chrome.history_href,
            page.previous_cursor,
            page.next_cursor,
            &[("limit", &page.limit.to_string())],
        )
    );
    shell(
        &format!("History: {}", chrome.title),
        chrome,
        &chrome.history_href,
        &content,
        is_admin,
        site,
        format!("Saved snapshots for {}.", chrome.title),
    )
}

pub fn snapshot_page(
    chrome: &ResourceChrome,
    snapshot: &ResourceSnapshot,
    body_html: &str,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let history_link = if is_admin {
        format!(
            r#"<a href="{}" class="btn">Back to history</a>"#,
            chrome.history_href
        )
    } else {
        String::new()
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<h1>Saved snapshot {}</h1>
<p class="page-summary">{} saved {}.</p>
</div>
<div class="page-actions">
<span class="status-pill">{}</span>
{}
<a href="{}" class="btn">{}</a>
</div>
</header>
{}
<section class="surface resource-surface prose">{}</section>"#,
        snapshot.snapshot_number,
        html_escape(&snapshot.title),
        super::render_time(&snapshot.created_at),
        if snapshot.is_private {
            "Private"
        } else {
            "Public"
        },
        history_link,
        chrome.current_href,
        open_live_label(chrome.kind),
        if snapshot.kind == ResourceKind::Media {
            snapshot_media_block(snapshot)
        } else {
            String::new()
        },
        body_html
    );
    shell(
        &format!(
            "Saved snapshot {}: {}",
            snapshot.snapshot_number, chrome.title
        ),
        chrome,
        &format!("/{}", snapshot.id),
        &content,
        is_admin,
        site,
        format!(
            "Saved snapshot {} for {}.",
            snapshot.snapshot_number, chrome.title
        ),
    )
}

fn shell(
    title: &str,
    chrome: &ResourceChrome,
    active: &str,
    content: &str,
    is_admin: bool,
    site: &SiteContext,
    description: String,
) -> String {
    base(
        &site.page_meta(title, description, false, None),
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &resource_rail(chrome, is_admin, active),
            content,
            "history-page",
            &site.site_name,
        ),
        "",
        "",
    )
}

fn history_row(entry: &HistoryLink) -> String {
    linked_card(
        &entry.href,
        "",
        "",
        &card_body(&entry.label, &entry.summary),
        &card_meta(
            &status_pill(entry.status, ""),
            &meta_line("Saved", &entry.created_at),
        ),
    )
}

fn live_row(resource: &Resource, chrome: &ResourceChrome) -> String {
    linked_card(
        &chrome.current_href,
        "",
        "",
        &card_body(live_label(resource.kind), &resource.summary),
        &card_meta(
            &status_pill(chrome.visibility, ""),
            &created_updated_lines(&chrome.created_at, &chrome.updated_at),
        ),
    )
}
