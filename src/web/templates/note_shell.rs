//! Shared note and history rail rendering

use super::model::{HistoryLink, NavLink, NoteChrome};

pub fn note_rail(chrome: &NoteChrome, is_admin: bool, active_href: &str) -> String {
    format!(
        r#"<section class="rail-section">
<h2>Current note</h2>
<div class="rail-list">
<a href="{}" class="rail-link{}">
<small>Current</small>
<span data-live-title>{}</span>
</a>
<div class="rail-facts">
<p><strong>Created</strong><span>{}</span></p>
<p><strong>Updated</strong><span>{}</span></p>
<p><strong>Visibility</strong><span data-live-visibility>{}</span></p>
</div>
</div>
</section>
<section class="rail-section">
<h2>Navigate</h2>
<div class="rail-list">{}{}</div>
</section>
<section class="rail-section">
<h2>History</h2>
<div class="rail-list">{}<a href="{}" class="rail-link{}"><small>History</small><span>All revisions</span></a></div>
</section>
{}"#,
        chrome.current_href,
        if active_href == chrome.current_href {
            " active"
        } else {
            ""
        },
        chrome.title,
        chrome.created_at,
        chrome.updated_at,
        chrome.visibility,
        optional_link(chrome.previous.as_ref(), "No older accessible note."),
        optional_link(chrome.next.as_ref(), "No newer accessible note."),
        history_links(&chrome.history, active_href),
        chrome.history_href,
        if active_href == chrome.history_href {
            " active"
        } else {
            ""
        },
        action_section(&chrome.id, is_admin)
    )
}

fn history_links(history: &[HistoryLink], active_href: &str) -> String {
    if history.is_empty() {
        return r#"<p class="rail-empty">No saved revisions yet.</p>"#.to_string();
    }
    history
        .iter()
        .take(5)
        .map(|entry| {
            format!(
                r#"<a href="{}" class="rail-link{}"><small>{}</small><span>{}</span><small>{}</small></a>"#,
                entry.href,
                if entry.active || entry.href == active_href { " active" } else { "" },
                entry.status,
                entry.label,
                entry.created_at
            )
        })
        .collect()
}

fn optional_link(link: Option<&NavLink>, empty: &str) -> String {
    link.map(|item| note_link(item, false))
        .unwrap_or_else(|| format!(r#"<p class="rail-empty">{empty}</p>"#))
}

fn note_link(link: &NavLink, active: bool) -> String {
    format!(
        r#"<a href="{}" class="rail-link{}"><small>{}</small><span>{}</span><small>{}</small></a>"#,
        link.href,
        if active { " active" } else { "" },
        link.relation,
        link.title,
        link.created_at
    )
}

fn action_section(id: &str, is_admin: bool) -> String {
    if is_admin {
        format!(
            r#"<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">
<a href="/admin" class="btn">Dashboard</a>
<button type="button" class="btn" onclick="createNote()">New note</button>
<button type="button" class="btn btn-danger" onclick="deleteNote('{id}')">Delete note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>
</div>
</section>"#
        )
    } else {
        r#"<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">
<a href="/" class="btn">Home</a>
<a href="/login" class="btn">Admin sign in</a>
</div>
</section>"#
            .to_string()
    }
}
