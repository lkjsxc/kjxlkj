//! Shared note and history rail rendering

use super::model::{HistoryLink, NavLink, NoteChrome};

pub fn note_rail(chrome: &NoteChrome, is_admin: bool, active_href: &str) -> String {
    format!(
        r#"<section class="rail-section">
<h2>Current note</h2>
<div class="rail-list">
{}
<div class="rail-facts">
<p><strong>Slug</strong><span>{}</span></p>
<p><strong>Created</strong><span>{}</span></p>
<p><strong>Updated</strong><span>{}</span></p>
<p><strong>Visibility</strong><span>{}</span></p>
</div>
</div>
</section>
<section class="rail-section">
<h2>Navigate</h2>
<div class="rail-list">{}{}</div>
</section>
<section class="rail-section">
<h2>History</h2>
<div class="rail-list">{}<a href="{}" class="rail-link"><span>All history</span><small>Open full index</small></a></div>
</section>
{}"#,
        note_link(
            &NavLink {
                href: format!("/{}", chrome.slug),
                label: chrome.title.clone(),
                meta: "Current version".to_string(),
            },
            active_href == format!("/{}", chrome.slug),
        ),
        chrome.slug,
        chrome.created_at,
        chrome.updated_at,
        chrome.visibility,
        optional_link(chrome.previous.as_ref(), "No older accessible note."),
        optional_link(chrome.next.as_ref(), "No newer accessible note."),
        history_links(&chrome.history, active_href),
        chrome.history_href,
        action_section(chrome.slug.as_str(), is_admin)
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
                r#"<a href="{}" class="rail-link{}"><span>{}</span><small>{} · {}</small></a>"#,
                entry.href,
                if entry.active || entry.href == active_href {
                    " active"
                } else {
                    ""
                },
                entry.label,
                entry.status,
                entry.meta
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
        r#"<a href="{}" class="rail-link{}"><span>{}</span><small>{}</small></a>"#,
        link.href,
        if active { " active" } else { "" },
        link.label,
        link.meta
    )
}

fn action_section(slug: &str, is_admin: bool) -> String {
    let actions = if is_admin {
        format!(
            r#"<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">
<a href="/admin" class="btn btn-primary">Dashboard</a>
<button type="button" class="btn" onclick="createNote()">New note</button>
<button type="button" class="btn btn-danger" onclick="deleteNote('{slug}')">Delete note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>
</div>
</section>"#
        )
    } else {
        r#"<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">
<a href="/" class="btn btn-primary">Home</a>
<a href="/login" class="btn">Admin sign in</a>
</div>
</section>"#
            .to_string()
    };
    actions
}
