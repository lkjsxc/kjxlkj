//! Shared searchable list page template

use super::layout::{base, html_escape, shell_page};
use super::model::{IndexItem, RecentLink};

pub struct ListPageConfig<'a> {
    pub page_title: &'a str,
    pub eyebrow: &'a str,
    pub summary: &'a str,
    pub path: &'a str,
    pub mode_label: &'a str,
    pub scope_title: &'a str,
    pub scope_summary: &'a str,
    pub actions: &'a str,
    pub extra_script: &'a str,
}

pub fn list_page(
    config: &ListPageConfig<'_>,
    recent: &[RecentLink],
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
) -> String {
    let rows = if notes.is_empty() {
        r#"<p class="surface-empty">No matching notes.</p>"#.to_string()
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    let rail = rail(config, recent, query);
    let content = format!(
        r#"<header class="index-header">
<div class="page-title-stack">
<p class="eyebrow">{}</p>
<h1>{}</h1>
<p class="page-summary">{}</p>
</div>
<div class="title-tags">{}</div>
</header>
<section class="stack note-list">{rows}</section>
{}{}"#,
        config.eyebrow,
        config.page_title,
        query_summary(config.summary, query),
        query_tag(query),
        pager(config.path, query, next_cursor),
        config.extra_script,
    );
    base(
        config.page_title,
        &shell_page(config.mode_label, &rail, &content, "index-layout"),
        "",
        "",
    )
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

fn rail(config: &ListPageConfig<'_>, recent: &[RecentLink], query: Option<&str>) -> String {
    format!(
        r#"<section class="rail-section">
<h2>Search</h2>
<form class="rail-search" method="GET" action="{}">
<label class="visually-hidden" for="rail-search-input">Search notes</label>
<input id="rail-search-input" type="search" name="q" value="{}" placeholder="Search current notes">
<button type="submit" class="btn">Search</button>
</form>
</section>
<section class="rail-section">
<h2>Scope</h2>
<div class="rail-copy"><strong>{}</strong><p>{}</p></div>
</section>
<section class="rail-section">
<h2>Recent</h2>
<div class="rail-list">{}</div>
</section>
<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">{}</div>
</section>"#,
        config.path,
        html_escape(query.unwrap_or("")),
        config.scope_title,
        config.scope_summary,
        recent_links(recent),
        config.actions
    )
}

fn recent_links(recent: &[RecentLink]) -> String {
    if recent.is_empty() {
        return r#"<p class="rail-empty">No accessible notes yet.</p>"#.to_string();
    }
    recent
        .iter()
        .map(|item| {
            format!(
                r#"<a href="{}" class="rail-link"><span>{}</span><small>{}</small>{}</a>"#,
                item.href,
                item.title,
                item.updated_at,
                item.visibility
                    .map(|value| format!(r#"<small>{value}</small>"#))
                    .unwrap_or_default()
            )
        })
        .collect()
}

fn query_summary(summary: &str, query: Option<&str>) -> String {
    query
        .filter(|item| !item.is_empty())
        .map(|item| format!(r#"{} Matching “{}”."#, summary, html_escape(item)))
        .unwrap_or_else(|| summary.to_string())
}

fn query_tag(query: Option<&str>) -> String {
    query
        .filter(|item| !item.is_empty())
        .map(|item| {
            format!(
                r#"<span class="status-pill">q: {}</span>"#,
                html_escape(item)
            )
        })
        .unwrap_or_default()
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
