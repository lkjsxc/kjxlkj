//! Shared live-resource and history rail rendering

use super::card_frame::{
    card_body, card_meta, created_updated_lines, linked_card, meta_line, static_card,
};
use super::index::admin_create_actions;
use super::layout::{html_escape, primary_nav, project_link_button, rail_section};
use super::model::{NavLink, ResourceChrome};
use super::resource_words::{delete_label, history_summary, live_label};

pub fn resource_rail(chrome: &ResourceChrome, is_admin: bool, active_href: &str) -> String {
    let mut sections = vec![rail_section("navigate", &primary_nav("", is_admin))];
    if is_admin {
        sections.push(rail_section("create", &create_action()));
    }
    sections.push(rail_section(
        "current-resource",
        &current_resource(chrome, active_href),
    ));
    sections.push(rail_section("timeline", &timeline(chrome)));
    sections.push(rail_section("history", &history(chrome, active_href)));
    sections.push(rail_section("project", &project_link()));
    sections.push(rail_section(
        "actions",
        &actions(chrome, is_admin, active_href),
    ));
    sections.join("")
}

pub fn live_resource_rail(chrome: &ResourceChrome, is_admin: bool) -> String {
    let mut sections = vec![rail_section("navigate", &primary_nav("", is_admin))];
    if is_admin {
        sections.push(rail_section("create", &create_action()));
    }
    sections.push(rail_section("project", &project_link()));
    sections.push(rail_section(
        "actions",
        &actions(chrome, is_admin, &chrome.current_href),
    ));
    sections.join("")
}

fn current_resource(chrome: &ResourceChrome, active_href: &str) -> String {
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
<p class="rail-slot-label">{}</p>
{}
</div>
<div class="rail-facts">
<p><strong>Alias</strong><span data-live-alias>{}</span></p>
</div>
</div>"#,
        live_label(chrome.kind),
        linked_card(
            &chrome.current_href,
            " data-current-resource-link",
            &format!(
                "summary-card current-resource-card{}",
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

fn timeline(chrome: &ResourceChrome) -> String {
    format!(
        r#"<div class="timeline-grid">{}{}</div>"#,
        timeline_slot(
            chrome.previous.as_ref(),
            "Prev",
            "No older accessible resource."
        ),
        timeline_slot(
            chrome.next.as_ref(),
            "Next",
            "No newer accessible resource."
        )
    )
}

fn history(chrome: &ResourceChrome, active_href: &str) -> String {
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
        &card_body("History", history_summary(chrome.kind)),
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

fn actions(chrome: &ResourceChrome, is_admin: bool, active_href: &str) -> String {
    if is_admin {
        format!(
            r#"<div class="rail-actions">
<button type="button" class="btn btn-danger" onclick="deleteResource(this, '{}')">{}</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>
</div>"#,
            chrome.id,
            delete_label(chrome.kind),
        )
    } else {
        format!(
            r#"<div class="rail-actions"><a href="/login?return_to={}" class="btn">Admin sign in</a></div>"#,
            html_escape(active_href),
        )
    }
}

fn create_action() -> String {
    format!(
        r#"<div class="rail-actions">{}</div>"#,
        admin_create_actions()
    )
}
