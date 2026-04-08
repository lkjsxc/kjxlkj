//! Shared note and history rail rendering

use super::card_frame::{
    card_body, card_meta, created_updated_lines, linked_card, meta_line, static_card,
};
use super::layout::{html_escape, primary_nav, project_link_button, rail_section};
use super::model::{NavLink, NoteChrome};

pub fn note_rail(chrome: &NoteChrome, is_admin: bool, active_href: &str) -> String {
    let mut sections = vec![rail_section("navigate", &primary_nav("", is_admin))];
    if is_admin {
        sections.push(rail_section("create", &create_action()));
    }
    sections.push(rail_section(
        "current-note",
        &current_note(chrome, active_href),
    ));
    sections.push(rail_section("timeline", &timeline(chrome)));
    sections.push(rail_section("history", &history(chrome, active_href)));
    sections.push(rail_section("project", &project_link()));
    sections.push(rail_section("actions", &actions(chrome, is_admin)));
    sections.join("")
}

fn current_note(chrome: &NoteChrome, active_href: &str) -> String {
    let card_body = format!(
        r#"<div class="card-body"><p class="card-title" data-live-title>{}</p><p class="card-summary">{}</p></div>"#,
        html_escape(&chrome.title),
        html_escape(&chrome.summary)
    );
    let card_meta = card_meta(
        &format!(
            r#"<span class="status-pill" data-live-visibility>{}</span>"#,
            chrome.visibility
        ),
        &created_updated_lines(&chrome.created_at, &chrome.updated_at),
    );
    format!(
        r#"<div class="rail-stack">
<div class="rail-slot">
<p class="rail-slot-label">Live note</p>
{}
</div>
<div class="rail-facts">
<p><strong>Alias</strong><span data-live-alias>{}</span></p>
</div>
</div>"#,
        linked_card(
            &chrome.current_href,
            " data-current-note-link",
            &format!(
                "summary-card current-note-card{}",
                if active_href == chrome.current_href {
                    " summary-card-active"
                } else {
                    ""
                }
            ),
            &card_body,
            &card_meta,
        ),
        chrome.alias.as_deref().unwrap_or("None"),
    )
}

fn timeline(chrome: &NoteChrome) -> String {
    format!(
        r#"<div class="timeline-grid">{}{}</div>"#,
        timeline_slot(
            chrome.previous.as_ref(),
            "Prev",
            "No older accessible note."
        ),
        timeline_slot(chrome.next.as_ref(), "Next", "No newer accessible note.")
    )
}

fn history(chrome: &NoteChrome, active_href: &str) -> String {
    linked_card(
        &chrome.history_href,
        " data-history-link",
        &format!(
            "summary-card history-card{}",
            if active_href == chrome.history_href {
                " summary-card-active"
            } else {
                ""
            }
        ),
        &card_body("History", "Browse the live note and saved snapshots."),
        &card_meta("", ""),
    )
}

fn timeline_slot(link: Option<&NavLink>, relation: &str, empty: &str) -> String {
    format!(
        r#"<div class="rail-slot timeline-slot"><p class="rail-slot-label">{relation}</p>{}</div>"#,
        link.map(note_link)
            .unwrap_or_else(|| missing_timeline_card(empty))
    )
}

fn project_link() -> String {
    format!(
        r#"<div class="rail-actions">{}</div>"#,
        project_link_button()
    )
}

fn note_link(link: &NavLink) -> String {
    linked_card(
        &link.href,
        "",
        "summary-card timeline-card",
        &card_body(&link.title, &link.summary),
        &card_meta("", &meta_line("Created", &link.created_at)),
    )
}

fn missing_timeline_card(empty: &str) -> String {
    static_card(
        r#" aria-disabled="true""#,
        "summary-card timeline-card summary-card-muted",
        &card_body(empty, ""),
        &card_meta("", ""),
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
            alias: Some("orbit-ledger".to_string()),
            title: "Orbit Ledger".to_string(),
            summary: "Shared release note for the orbit ledger.".to_string(),
            current_href: "/demo".to_string(),
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            is_favorite: true,
            visibility: "Public",
            previous,
            next,
            history_href: "/demo/history".to_string(),
        }
    }

    #[test]
    fn note_rail_keeps_single_history_card() {
        let html = note_rail(&chrome(None, None), true, "/demo");
        let history = html.find("History").unwrap();
        let github = html.find("Open GitHub").unwrap();
        let delete = html.find("Delete note").unwrap();
        let logout = html.find("Logout").unwrap();
        assert!(html.contains("History"));
        assert!(html.contains("Open GitHub"));
        assert!(html.contains("<span>Created</span>"));
        assert!(html.contains("<span>Updated</span>"));
        assert!(history < github && github < delete && delete < logout);
    }

    #[test]
    fn note_rail_renders_disabled_timeline_cards() {
        let html = note_rail(&chrome(None, None), false, "/demo");
        assert!(html.contains("No older accessible note."));
        assert!(html.contains("No newer accessible note."));
        assert!(!html.contains("CREATE"));
        assert!(html.contains("timeline-grid"));
    }
}
