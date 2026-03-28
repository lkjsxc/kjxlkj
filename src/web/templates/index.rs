//! Shared browse list templates

use super::layout::{base, html_escape, primary_nav, rail_section, shell_page};
use super::model::IndexItem;

pub struct ListPageConfig<'a> {
    pub page_title: &'a str,
    pub path: &'a str,
    pub mode_label: &'a str,
    pub active_nav: &'a str,
    pub rail_primary_action: &'a str,
    pub rail_actions: &'a str,
    pub list_class: &'a str,
    pub empty_text: &'a str,
    pub is_admin: bool,
    pub extra_script: &'a str,
}

pub fn list_page(
    config: &ListPageConfig<'_>,
    notes: &[IndexItem],
    next_cursor: Option<&str>,
) -> String {
    let rows = if notes.is_empty() {
        format!(r#"<p class="surface-empty">{}</p>"#, config.empty_text)
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack"><h1>{}</h1></div>
</header>
<section class="stack note-list {}">{rows}</section>
{}{}"#,
        config.page_title,
        config.list_class,
        pager(config.path, None, next_cursor),
        config.extra_script,
    );
    base(
        config.page_title,
        &shell_page(
            config.mode_label,
            &list_rail(
                config.active_nav,
                config.rail_primary_action,
                config.rail_actions,
                config.is_admin,
            ),
            &content,
            "index-layout",
        ),
        "",
        "",
    )
}

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
        note.title,
        note.summary,
        note.visibility
            .map(|item| format!(r#"<span class="status-pill">{item}</span>"#))
            .unwrap_or_default(),
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
    let mut sections = Vec::new();
    if !rail_primary_action.is_empty() {
        sections.push(rail_section(
            "create",
            &format!(r#"<div class="rail-actions">{rail_primary_action}</div>"#),
        ));
    }
    sections.push(rail_section("navigate", &primary_nav(active_nav, is_admin)));
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
