//! Shared browse list templates

use super::layout::{html_escape, primary_nav, rail_section};
use super::model::IndexItem;

pub(crate) fn note_row(note: &IndexItem) -> String {
    format!(
        r#"<a href="{}" class="index-card note-row" data-note-id="{}">
<div class="card-body">
<p class="card-title">{}</p>
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
<div class="card-badges">{}</div>
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
</div>
</a>"#,
        note.href,
        note.id,
        html_escape(&note.title),
        html_escape(&note.summary),
        card_badges(note),
        note.created_at,
        note.updated_at
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
    if !rail_actions.is_empty() {
        sections.push(rail_section(
            "actions",
            &format!(r#"<div class="rail-actions">{rail_actions}</div>"#),
        ));
    }
    sections.join("")
}

pub(crate) fn pager(
    path: &str,
    previous_cursor: Option<&str>,
    next_cursor: Option<&str>,
    fields: &[(&str, &str)],
) -> String {
    format!(
        r#"<div class="pager-nav">{}{}</div>"#,
        page_button(path, previous_cursor, "prev", "Previous", fields),
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
        badges
            .push(r#"<span class="status-pill status-pill-favorite">Favorite</span>"#.to_string());
    }
    if let Some(item) = note.visibility {
        badges.push(format!(r#"<span class="status-pill">{item}</span>"#));
    }
    badges.join("")
}
