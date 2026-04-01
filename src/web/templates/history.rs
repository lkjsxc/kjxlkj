//! History page templates

use super::index::pager;
use super::layout::{base, shell_page};
use super::model::{HistoryLink, NoteChrome};
use super::note_shell::note_rail;
use super::sections::page_header;
use crate::core::{derive_summary, derive_title, render_markdown};
use crate::web::db::{Record, RecordRevision};

pub fn history_page(
    record: &Record,
    chrome: &NoteChrome,
    history: &[HistoryLink],
    previous_cursor: Option<&str>,
    next_cursor: Option<&str>,
    limit: i64,
    is_admin: bool,
) -> String {
    let current_note_action = format!(
        r#"<a href="{}" class="btn">Current note</a>"#,
        chrome.current_href
    );
    let cards = if history.is_empty() {
        r#"<p class="surface-empty">No saved revisions yet.</p>"#.to_string()
    } else {
        history.iter().map(history_row).collect::<Vec<_>>().join("")
    };
    let current_note_card = format!(
        r#"<section class="stack note-list">{}</section>"#,
        current_row(record, chrome)
    );
    let history_section = format!(
        r#"<section class="section-block history-list-section">
<div class="section-head"><h2>History</h2></div>
<div class="note-list">{cards}</div>
{}
</section>"#,
        pager(
            &chrome.history_href,
            previous_cursor,
            next_cursor,
            &[("limit", &limit.to_string())],
        )
    );
    let content = format!(
        "{}{}{}",
        page_header(&chrome.title, Some(&current_note_action), "history-head"),
        current_note_card,
        history_section,
    );
    shell(
        &format!("History - {}", chrome.title),
        chrome,
        &chrome.history_href,
        &content,
        is_admin,
    )
}

pub fn revision_page(
    _record: &Record,
    chrome: &NoteChrome,
    revision: &RecordRevision,
    is_admin: bool,
) -> String {
    let active = format!(
        "{}/history/{}",
        chrome.current_href, revision.revision_number
    );
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<h1>{}</h1>
<p class="page-summary">Saved {}.</p>
</div>
<div class="page-actions">
<span class="status-pill">{}</span>
<a href="{}" class="btn">Back to history</a>
<a href="{}" class="btn">Current note</a>
</div>
</header>
<section class="surface note-surface prose">{}</section>"#,
        derive_title(&revision.body),
        super::render_time(&revision.created_at),
        if revision.is_private {
            "Private"
        } else {
            "Public"
        },
        chrome.history_href,
        chrome.current_href,
        render_markdown(&revision.body)
    );
    shell(
        &format!("Revision {} - {}", revision.revision_number, chrome.title),
        chrome,
        &active,
        &content,
        is_admin,
    )
}

fn shell(title: &str, chrome: &NoteChrome, active: &str, content: &str, is_admin: bool) -> String {
    base(
        title,
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &note_rail(chrome, is_admin, active),
            content,
            "history-page",
        ),
        "",
        "",
    )
}

fn history_row(entry: &HistoryLink) -> String {
    format!(
        r#"<a href="{}" class="index-card note-row">
<div class="card-body">
<p class="card-title">{}</p>
<p class="card-summary">Saved {}</p>
</div>
<div class="card-meta"><span class="status-pill">{}</span></div>
</a>"#,
        entry.href, entry.label, entry.created_at, entry.status
    )
}

fn current_row(record: &Record, chrome: &NoteChrome) -> String {
    format!(
        r#"<a href="{}" class="index-card note-row">
<div class="card-body">
<p class="card-title">Current note</p>
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
<span class="status-pill">{}</span>
<small><span>Updated</span>{}</small>
</div>
</a>"#,
        chrome.current_href,
        derive_summary(&record.body),
        chrome.visibility,
        chrome.updated_at
    )
}
