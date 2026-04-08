//! History page templates

use super::card_frame::{
    card_body, card_meta, created_updated_lines, linked_card, meta_line, status_pill,
};
use super::index::pager;
use super::layout::{base, html_escape, shell_page};
use super::model::{HistoryLink, NoteChrome};
use super::note_shell::note_rail;
use super::sections::page_header;
use crate::web::db::{Record, RecordSnapshot};
use crate::web::site::SiteContext;

pub struct HistoryPage<'a> {
    pub history: &'a [HistoryLink],
    pub previous_cursor: Option<&'a str>,
    pub next_cursor: Option<&'a str>,
    pub limit: i64,
}

pub fn history_page(
    record: &Record,
    chrome: &NoteChrome,
    page: HistoryPage<'_>,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let live_note_action = format!(
        r#"<a href="{}" class="btn">Open live note</a>"#,
        chrome.current_href
    );
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
        "{}<section class=\"stack note-list\">{}</section><section class=\"section-block history-list-section\"><div class=\"section-head\"><h2>Saved snapshots</h2></div><div class=\"note-list\">{}</div>{}</section>",
        page_header(&format!("History: {}", chrome.title), Some(&live_note_action), "history-head"),
        live_row(record, chrome),
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
    chrome: &NoteChrome,
    snapshot: &RecordSnapshot,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<h1>Saved snapshot {}</h1>
<p class="page-summary">{} saved {}.</p>
</div>
<div class="page-actions">
<span class="status-pill">{}</span>
<a href="{}" class="btn">Back to history</a>
<a href="{}" class="btn">Open live note</a>
</div>
</header>
<section class="surface note-surface prose">{}</section>"#,
        snapshot.snapshot_number,
        html_escape(&snapshot.title),
        super::render_time(&snapshot.created_at),
        if snapshot.is_private {
            "Private"
        } else {
            "Public"
        },
        chrome.history_href,
        chrome.current_href,
        crate::core::render_markdown(&snapshot.body)
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
    chrome: &NoteChrome,
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
            &note_rail(chrome, is_admin, active),
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

fn live_row(record: &Record, chrome: &NoteChrome) -> String {
    linked_card(
        &chrome.current_href,
        "",
        "",
        &card_body("Live note", &record.summary),
        &card_meta(
            &status_pill(chrome.visibility, ""),
            &created_updated_lines(&chrome.created_at, &chrome.updated_at),
        ),
    )
}
