//! History page templates

use super::layout::{base, format_date, render_markdown, shell_page};
use super::model::{HistoryLink, NoteChrome};
use super::note_shell::note_rail;
use crate::core::extract_title;
use crate::web::db::{Record, RecordRevision};

pub fn history_page(record: &Record, chrome: &NoteChrome, is_admin: bool) -> String {
    let rail = note_rail(chrome, is_admin, &chrome.history_href);
    let rows = if chrome.history.is_empty() {
        r#"<p class="surface-empty">No saved revisions yet.</p>"#.to_string()
    } else {
        chrome
            .history
            .iter()
            .map(history_row)
            .collect::<Vec<_>>()
            .join("")
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">Revision history</p>
<h1>{}</h1>
</div>
<a href="/{}" class="btn btn-primary">Current note</a>
</header>
<section class="stack">{}</section>"#,
        chrome.title, record.slug, rows
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
    let active = format!("/{}/history/{}", record.slug, revision.revision_number);
    let rail = note_rail(chrome, is_admin, &active);
    let title = extract_title(&revision.body)
        .unwrap_or_else(|| format!("Revision {}", revision.revision_number));
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">Historical snapshot</p>
<h1>{}</h1>
</div>
<div class="page-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</header>
<section class="surface note-surface prose">{}</section>
<footer class="page-tail">
<a href="{}" class="btn">Back to history</a>
<a href="/{}" class="btn btn-primary">Current note</a>
</footer>"#,
        title,
        if revision.is_private {
            "Private"
        } else {
            "Public"
        },
        format_date(&revision.created_at),
        render_markdown(&revision.body),
        chrome.history_href,
        record.slug
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
        r#"<a href="{}" class="index-card">
<div class="card-body">
<p class="card-title">{}</p>
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</a>"#,
        entry.href, entry.label, entry.meta, entry.status, entry.meta
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::NoteChrome;
    use chrono::Utc;

    fn sample_record() -> Record {
        Record {
            slug: "demo-note".to_string(),
            body: "# Demo\n\nBody".to_string(),
            is_private: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn sample_chrome() -> NoteChrome {
        NoteChrome {
            title: "Demo".to_string(),
            slug: "demo-note".to_string(),
            created_at: "March 25, 2026 at 01:34 AM".to_string(),
            updated_at: "March 25, 2026 at 01:35 AM".to_string(),
            visibility: "Public",
            previous: None,
            next: None,
            history: vec![HistoryLink {
                href: "/demo-note/history/2".to_string(),
                label: "Revision 2".to_string(),
                meta: "March 24, 2026 at 08:00 PM".to_string(),
                status: "Public",
                active: false,
            }],
            history_href: "/demo-note/history".to_string(),
        }
    }

    #[test]
    fn history_page_lists_revision_links() {
        let html = history_page(&sample_record(), &sample_chrome(), false);
        assert!(html.contains("Revision history"));
        assert!(html.contains("/demo-note/history/2"));
        assert!(html.contains("Current note"));
        assert!(html.contains("aria-label=\"Open navigation\""));
    }

    #[test]
    fn revision_page_renders_historical_body() {
        let revision = RecordRevision {
            revision_number: 2,
            body: "# Old\n\nSnapshot".to_string(),
            is_private: false,
            created_at: Utc::now(),
        };
        let html = revision_page(&sample_record(), &sample_chrome(), &revision, false);
        assert!(html.contains("Historical snapshot"));
        assert!(html.contains("Snapshot"));
        assert!(html.contains("Back to history"));
    }
}
