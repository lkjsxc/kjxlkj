//! Shared searchable list page template

use super::layout::{base, html_escape};
use super::model::IndexItem;

pub struct ListPageConfig<'a> {
    pub page_title: &'a str,
    pub eyebrow: &'a str,
    pub summary: &'a str,
    pub path: &'a str,
    pub actions: &'a str,
    pub extra_script: &'a str,
}

pub fn list_page(
    config: &ListPageConfig<'_>,
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
) -> String {
    let rows = if notes.is_empty() {
        r#"<p class="surface-empty">No matching notes.</p>"#.to_string()
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    let content = format!(
        r#"<div class="index-layout">
<header class="index-header">
<div class="page-title-stack">
<p class="eyebrow">{}</p>
<h1>{}</h1>
<p class="page-summary">{}</p>
</div>
<div class="index-actions">{}</div>
</header>
<form class="search-form" method="GET" action="{}">
<label class="visually-hidden" for="search-input">Search notes</label>
<input id="search-input" type="search" name="q" value="{}" placeholder="Search current notes">
<button type="submit" class="btn">Search</button>
</form>
<section class="stack note-list">{rows}</section>
{}
</div>{}"#,
        config.eyebrow,
        config.page_title,
        config.summary,
        config.actions,
        config.path,
        html_escape(query.unwrap_or("")),
        pager(config.path, query, next_cursor),
        config.extra_script,
    );
    base(config.page_title, &content, "", "")
}

fn note_row(note: &IndexItem) -> String {
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

fn pager(path: &str, query: Option<&str>, next_cursor: Option<&str>) -> String {
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
