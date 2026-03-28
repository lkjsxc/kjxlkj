//! Shared note and history rail rendering

use super::layout::{html_escape, primary_nav, rail_section};
use super::model::{NavLink, NoteChrome};

pub fn note_rail(chrome: &NoteChrome, is_admin: bool, active_href: &str) -> String {
    let mut sections = Vec::new();
    if is_admin {
        sections.push(rail_section("create", &create_action()));
    }
    sections.extend([
        rail_section("navigate", &primary_nav("", is_admin)),
        rail_section("current-note", &current_note(chrome, active_href)),
        rail_section("timeline", &timeline(chrome)),
        rail_section("history", &history(chrome, active_href)),
        rail_section("actions", &actions(chrome, is_admin)),
    ]);
    sections.join("")
}

fn current_note(chrome: &NoteChrome, active_href: &str) -> String {
    format!(
        r#"<div class="rail-list">
<a href="{}" class="rail-link{}"><span data-live-title>{}</span><small>Current note</small></a>
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
        timeline_card(
            chrome.previous.as_ref(),
            "Prev",
            "No older accessible note."
        ),
        timeline_card(chrome.next.as_ref(), "Next", "No newer accessible note.")
    )
}

fn history(chrome: &NoteChrome, active_href: &str) -> String {
    format!(
        r#"<div class="rail-list"><a href="{}" class="rail-link{}"><span>All history</span></a></div>"#,
        chrome.history_href,
        if active_href == chrome.history_href {
            " active"
        } else {
            ""
        }
    )
}

fn timeline_card(link: Option<&NavLink>, relation: &str, empty: &str) -> String {
    link.map(note_link)
        .unwrap_or_else(|| missing_timeline_card(relation, empty))
}

fn missing_timeline_card(relation: &str, empty: &str) -> String {
    format!(
        r#"<article class="rail-link rail-link-muted" aria-disabled="true"><small>{relation}</small><span>{empty}</span></article>"#
    )
}

fn note_link(link: &NavLink) -> String {
    format!(
        r#"<a href="{}" class="rail-link"><small>{}</small><span>{}</span><small>{}</small></a>"#,
        link.href,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn chrome(previous: Option<NavLink>, next: Option<NavLink>) -> NoteChrome {
        NoteChrome {
            id: "demo".to_string(),
            title: "Orbit Ledger".to_string(),
            current_href: "/demo".to_string(),
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            visibility: "Public",
            previous,
            next,
            history_href: "/demo/history".to_string(),
        }
    }

    #[test]
    fn note_rail_keeps_single_history_card() {
        let html = note_rail(&chrome(None, None), true, "/demo");
        assert!(html.contains("All history"));
        assert!(!html.contains("View every visible revision"));
    }

    #[test]
    fn note_rail_renders_disabled_timeline_cards() {
        let html = note_rail(&chrome(None, None), false, "/demo");
        assert!(html.contains("No older accessible note."));
        assert!(html.contains("No newer accessible note."));
        assert!(!html.contains("CREATE"));
    }
}
