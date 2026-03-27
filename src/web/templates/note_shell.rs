//! Shared note and history rail rendering

use super::layout::{html_escape, primary_nav, rail_section};
use super::model::{HistoryLink, NavLink, NoteChrome};

pub fn note_rail(chrome: &NoteChrome, is_admin: bool, active_href: &str) -> String {
    let mut sections = Vec::new();
    if is_admin {
        sections.push(rail_section("Create", &create_action()));
    }
    sections.extend([
        rail_section("Navigate", &primary_nav("", is_admin)),
        rail_section("Current note", &current_note(chrome, active_href)),
        rail_section("Timeline", &timeline(chrome)),
        rail_section("History", &history(chrome, active_href)),
        rail_section("Actions", &actions(chrome, is_admin)),
    ]);
    sections.join("")
}

fn current_note(chrome: &NoteChrome, active_href: &str) -> String {
    format!(
        r#"<div class="rail-list">
<a href="{}" class="rail-link{}"><small>Current</small><span data-live-title>{}</span></a>
<div class="rail-facts">
<p><strong>Created</strong><span>{}</span></p>
<p><strong>Updated</strong><span>{}</span></p>
<p><strong>Visibility</strong><span data-live-visibility>{}</span></p>
</div>
</div>"#,
        chrome.current_href,
        if active_href == chrome.current_href {
            " active"
        } else {
            ""
        },
        chrome.title,
        chrome.created_at,
        chrome.updated_at,
        chrome.visibility
    )
}

fn timeline(chrome: &NoteChrome) -> String {
    format!(
        r#"<div class="rail-list">{}{}</div>"#,
        optional_link(chrome.previous.as_ref(), "No older accessible note."),
        optional_link(chrome.next.as_ref(), "No newer accessible note.")
    )
}

fn history(chrome: &NoteChrome, active_href: &str) -> String {
    format!(
        r#"<div class="rail-list">{}<a href="{}" class="rail-link{}"><small>Index</small><span>All revisions</span></a></div>"#,
        history_links(&chrome.history, active_href),
        chrome.history_href,
        if active_href == chrome.history_href {
            " active"
        } else {
            ""
        }
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
                if entry.active || entry.href == active_href {
                    " active"
                } else {
                    ""
                },
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
        html_escape(&link.title),
        link.created_at
    )
}

fn actions(chrome: &NoteChrome, is_admin: bool) -> String {
    if is_admin {
        format!(
            r#"<div class="rail-actions">
<button type="button" class="btn btn-danger" onclick="deleteNote('{}')">Delete note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>
</div>"#,
            chrome.id
        )
    } else {
        r#"<div class="rail-actions"><a href="/login" class="btn">Admin sign in</a></div>"#
            .to_string()
    }
}

fn create_action() -> String {
    r#"<div class="rail-actions"><button type="button" class="btn btn-primary" onclick="createNote()">New note</button></div>"#
        .to_string()
}
