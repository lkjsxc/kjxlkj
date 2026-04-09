//! Shared browse list templates

use super::card_frame::{
    card_body, card_meta, created_updated_lines, linked_card, meta_line, status_pill,
};
use super::layout::{html_escape, primary_nav, project_link_button, rail_section};
use super::model::IndexItem;

pub(crate) fn note_row(note: &IndexItem) -> String {
    linked_card(
        &note.href,
        &format!(r#" data-note-id="{}""#, note.id),
        "",
        &card_body(&note.title, &note.summary),
        &card_meta(
            &card_badges(note),
            &format!(
                "{}{}",
                card_metrics(note),
                created_updated_lines(&note.created_at, &note.updated_at)
            ),
        ),
    )
}

pub(crate) fn list_rail(
    active_nav: &str,
    rail_primary_action: &str,
    rail_actions: &str,
    is_admin: bool,
) -> String {
    let mut sections = vec![rail_section("navigate", &primary_nav(active_nav, is_admin))];
    if !rail_primary_action.is_empty() {
        sections.push(rail_section(
            "create",
            &format!(r#"<div class="rail-actions">{rail_primary_action}</div>"#),
        ));
    }
    sections.push(rail_section(
        "project",
        &format!(
            r#"<div class="rail-actions">{}</div>"#,
            project_link_button()
        ),
    ));
    if !rail_actions.is_empty() {
        sections.push(rail_section(
            "actions",
            &format!(r#"<div class="rail-actions">{rail_actions}</div>"#),
        ));
    }
    sections.join("")
}

pub(crate) fn admin_create_actions() -> String {
    r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>
<a href="/admin/media/new" class="btn">New media</a>"#
        .to_string()
}

pub(crate) fn pager(
    path: &str,
    previous_cursor: Option<&str>,
    next_cursor: Option<&str>,
    fields: &[(&str, &str)],
) -> String {
    format!(
        r#"<div class="pager-nav">{}{}</div>"#,
        page_button(path, previous_cursor, "prev", "Prev", fields),
        page_button(path, next_cursor, "next", "Next", fields),
    )
}

fn hidden_input(name: &str, value: &str) -> String {
    format!(
        r#"<input type="hidden" name="{}" value="{}">"#,
        html_escape(name),
        html_escape(value)
    )
}

fn page_button(
    path: &str,
    cursor: Option<&str>,
    direction: &str,
    label: &str,
    fields: &[(&str, &str)],
) -> String {
    let hidden = fields
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(name, value)| hidden_input(name, value))
        .collect::<Vec<_>>()
        .join("");
    cursor.map_or_else(
        || {
            format!(
                r#"<div class="pager"><button type="button" class="btn" disabled>{label}</button></div>"#
            )
        },
        |cursor| {
            format!(
                r#"<form class="pager" method="GET" action="{path}">
{hidden}
<input type="hidden" name="direction" value="{direction}">
<input type="hidden" name="cursor" value="{cursor}">
<button type="submit" class="btn">{label}</button>
</form>"#
            )
        },
    )
}

fn card_badges(note: &IndexItem) -> String {
    let mut badges = Vec::new();
    if note.is_favorite {
        badges.push(status_pill("Favorite", "status-pill-favorite"));
    }
    if let Some(item) = note.visibility {
        badges.push(status_pill(item, ""));
    }
    badges.join("")
}

fn card_metrics(note: &IndexItem) -> String {
    note.metrics
        .iter()
        .map(|metric| meta_line(&metric.label, &metric.value))
        .collect::<Vec<_>>()
        .join("")
}
