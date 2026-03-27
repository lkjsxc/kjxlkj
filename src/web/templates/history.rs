//! History page templates

use super::layout::{base, shell_page};
use super::model::{HistoryLink, NoteChrome};
use super::note_shell::note_rail;
use crate::core::{derive_title, render_markdown};
use crate::web::db::{Record, RecordRevision};

pub fn history_page(
    record: &Record,
    chrome: &NoteChrome,
    history: &[HistoryLink],
    is_admin: bool,
) -> String {
    let rail = note_rail(chrome, is_admin, &chrome.history_href);
    let rows = if history.is_empty() {
        r#"<p class="surface-empty">No saved revisions yet.</p>"#.to_string()
    } else {
        format!(
            "{}{}",
            current_row(record, chrome),
            history.iter().map(history_row).collect::<Vec<_>>().join("")
        )
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">History</p>
<h1>{}</h1>
</div>
<div class="page-actions"><a href="/{}" class="btn">Current note</a></div>
</header>
<section class="stack">{}</section>"#,
        chrome.title, record.id, rows
    );
    base(
        &format!("History - {}", chrome.title),
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &rail,
            &content,
            "history-page",
        ),
        "",
        "",
    )
}

pub fn revision_page(
    record: &Record,
    chrome: &NoteChrome,
    revision: &RecordRevision,
    is_admin: bool,
) -> String {
    let active = format!("/{}/history/{}", record.id, revision.revision_number);
    let rail = note_rail(chrome, is_admin, &active);
    let title = derive_title(&revision.body);
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">Snapshot</p>
<h1>{}</h1>
<p class="page-summary">Saved {}.</p>
</div>
<div class="page-actions">
<span class="status-pill">{}</span>
<a href="{}" class="btn">Back to history</a>
<a href="/{}" class="btn">Current note</a>
</div>
</header>
<section class="surface note-surface prose">{}</section>"#,
        title,
        super::render_time(&revision.created_at),
        if revision.is_private {
            "Private"
        } else {
            "Public"
        },
        chrome.history_href,
        record.id,
        render_markdown(&revision.body)
    );
    base(
        &format!("Revision {} - {}", revision.revision_number, chrome.title),
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &rail,
            &content,
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
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</a>"#,
        entry.href, entry.label, entry.created_at, entry.status, entry.created_at
    )
}

fn current_row(record: &Record, chrome: &NoteChrome) -> String {
    format!(
        r#"<a href="/{}" class="index-card note-row">
<div class="card-body">
<p class="card-title">Current note</p>
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</a>"#,
        record.id, chrome.updated_at, chrome.visibility, chrome.updated_at
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::NoteChrome;
    use chrono::Utc;

    fn sample_record() -> Record {
        Record {
            id: "Q29udHJhY3RSdW50aW1lMQ".to_string(),
            title: "Demo".to_string(),
            summary: "Body".to_string(),
            body: "# Demo\n\nBody".to_string(),
            is_private: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn sample_chrome() -> NoteChrome {
        NoteChrome {
            id: "Q29udHJhY3RSdW50aW1lMQ".to_string(),
            title: "Demo".to_string(),
            current_href: "/Q29udHJhY3RSdW50aW1lMQ".to_string(),
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            visibility: "Public",
            previous: None,
            next: None,
            history_href: "/Q29udHJhY3RSdW50aW1lMQ/history".to_string(),
        }
    }

    #[test]
    fn history_page_lists_current_note_and_revisions() {
        let html = history_page(
            &sample_record(),
            &sample_chrome(),
            &[HistoryLink {
                href: "/Q29udHJhY3RSdW50aW1lMQ/history/2".to_string(),
                label: "Revision 2".to_string(),
                created_at: "2026-03-26 08:00 UTC".to_string(),
                status: "Public",
            }],
            false,
        );
        assert!(html.contains("History"));
        assert!(html.contains("Current note"));
        assert!(html.contains("/Q29udHJhY3RSdW50aW1lMQ/history/2"));
        assert!(html.contains("Current note"));
    }
}
