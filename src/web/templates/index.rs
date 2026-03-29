//! Shared browse list templates

use super::layout::{html_escape, primary_nav, rail_section};
use super::model::IndexItem;

pub(crate) fn note_row(note: &IndexItem) -> String {
    format!(
        r#"<a href="{}" class="index-card note-row">
<div class="card-body">
<p class="card-title">{}</p>
<p class="card-summary">{}</p>
</div>
<div class="card-meta">
{}
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
</div>
</a>"#,
        note.href,
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

pub(crate) fn pager(path: &str, query: Option<&str>, next_cursor: Option<&str>) -> String {
    next_cursor
        .map(|cursor| {
            format!(
                r#"<form class="pager" method="GET" action="{path}">
<input type="hidden" name="cursor" value="{cursor}">
{}
<button type="submit" class="btn">More notes</button>
</form>"#,
                query
                    .map(|value| format!(
                        r#"<input type="hidden" name="q" value="{}">"#,
                        html_escape(value)
                    ))
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default()
}

fn card_badges(note: &IndexItem) -> String {
    let mut badges = Vec::new();
    if note.is_favorite {
        badges.push(r#"<span class="status-pill status-pill-favorite">Favorite</span>"#.to_string());
    }
    if let Some(item) = note.visibility {
        badges.push(format!(r#"<span class="status-pill">{item}</span>"#));
    }
    badges.join("")
}
