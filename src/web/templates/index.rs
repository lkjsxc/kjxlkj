//! Shared searchable list page template

use super::layout::{base, primary_nav, rail_section, shell_page};
use super::model::IndexItem;

pub struct ListPageConfig<'a> {
    pub page_title: &'a str,
    pub eyebrow: &'a str,
    pub summary: &'a str,
    pub path: &'a str,
    pub mode_label: &'a str,
    pub scope_title: &'a str,
    pub scope_summary: &'a str,
    pub active_nav: &'a str,
    pub rail_primary_action: &'a str,
    pub header_actions: &'a str,
    pub rail_actions: &'a str,
    pub is_admin: bool,
    pub extra_script: &'a str,
}

pub fn list_page(
    config: &ListPageConfig<'_>,
    notes: &[IndexItem],
    next_cursor: Option<&str>,
) -> String {
    let rows = if notes.is_empty() {
        r#"<p class="surface-empty">No matching notes.</p>"#.to_string()
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    let rail = rail(config);
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">{}</p>
<h1>{}</h1>
<p class="page-summary">{}</p>
</div>
<div class="page-actions">{}</div>
</header>
<section class="surface scope-card">
<p class="scope-label">{}</p>
<p class="page-summary">{}</p>
</section>
<section class="stack note-list">{rows}</section>
{}{}"#,
        config.eyebrow,
        config.page_title,
        config.summary,
        config.header_actions,
        config.scope_title,
        config.scope_summary,
        pager(config.path, None, next_cursor),
        config.extra_script,
    );
    base(
        config.page_title,
        &shell_page(config.mode_label, &rail, &content, "index-layout"),
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
<small>Created {}</small>
<small>Updated {}</small>
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

fn rail(config: &ListPageConfig<'_>) -> String {
    let mut sections = Vec::new();
    if !config.rail_primary_action.is_empty() {
        sections.push(rail_section(
            "Create",
            &format!(
                r#"<div class="rail-actions">{}</div>"#,
                config.rail_primary_action
            ),
        ));
    }
    sections.push(rail_section(
        "Navigate",
        &primary_nav(config.active_nav, config.is_admin),
    ));
    sections.push(rail_section(
        "Scope",
        &format!(
            r#"<div class="rail-copy"><strong>{}</strong><p>{}</p></div>"#,
            config.scope_title, config.scope_summary
        ),
    ));
    if !config.rail_actions.is_empty() {
        sections.push(rail_section(
            "Actions",
            &format!(r#"<div class="rail-actions">{}</div>"#, config.rail_actions),
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
                        super::layout::html_escape(value)
                    ))
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default()
}
