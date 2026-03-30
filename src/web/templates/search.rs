//! Dedicated search page template

use super::index::{list_rail, note_row, pager};
use super::layout::{base, html_escape, shell_page};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn search_page(
    notes: &[IndexItem],
    previous_cursor: Option<&str>,
    next_cursor: Option<&str>,
    query: Option<&str>,
    limit: i64,
    sort: &str,
    is_admin: bool,
) -> String {
    let query = query.unwrap_or("").trim();
    let has_query = !query.is_empty();
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack"><h1>Search</h1></div>
</header>
<section class="surface section-block search-surface">
<div class="section-head"><h2>Search notes</h2></div>
<form class="search-form" method="GET" action="/search">
<label for="search-page-input" class="visually-hidden">Search notes</label>
<div class="search-grid">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search aliases, titles, and bodies">
<div class="search-state-card"><small>Query</small><strong>{}</strong></div>
<label class="form-group search-sort" for="search-sort">
<span>Sort</span>
<select id="search-sort" name="sort">{}</select>
</label>
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>
</section>
{}"#,
        html_escape(query),
        query_display(query),
        sort_options(sort, has_query),
        results_section(
            notes,
            previous_cursor,
            next_cursor,
            query,
            limit,
            sort,
            has_query,
        ),
    );
    base(
        "Search",
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "search",
                rail_primary_action(is_admin),
                rail_actions(is_admin),
                is_admin,
            ),
            &content,
            "index-layout",
        ),
        "",
        &extra_script,
    )
}

fn results_section(
    notes: &[IndexItem],
    previous_cursor: Option<&str>,
    next_cursor: Option<&str>,
    query: &str,
    limit: i64,
    sort: &str,
    has_query: bool,
) -> String {
    let title = if has_query {
        format!(r#"Results for “{}”"#, html_escape(query))
    } else {
        "All notes".to_string()
    };
    let cards = if notes.is_empty() {
        format!(
            r#"<p class="surface-empty">{}</p>"#,
            if has_query {
                "No matching notes."
            } else {
                "No notes yet."
            }
        )
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    let limit = limit.to_string();
    let pager = pager(
        "/search",
        previous_cursor,
        next_cursor,
        &[("q", query), ("sort", sort), ("limit", &limit)],
    );
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>{title}</h2></div>
<div class="note-list note-grid">{cards}</div>
{pager}
</section>"#
    )
}

fn query_display(query: &str) -> String {
    if query.is_empty() {
        "All notes".to_string()
    } else {
        html_escape(query)
    }
}

fn sort_options(selected: &str, has_query: bool) -> String {
    sort_catalog(has_query)
        .into_iter()
        .map(|(value, label)| {
            format!(
                r#"<option value="{value}"{}>{label}</option>"#,
                if value == selected { " selected" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn sort_catalog(has_query: bool) -> Vec<(&'static str, &'static str)> {
    let mut items = vec![
        ("updated_desc", "Recently updated"),
        ("updated_asc", "Oldest updates"),
        ("created_desc", "Newest created"),
        ("created_asc", "Oldest created"),
        ("title_asc", "Title A-Z"),
        ("title_desc", "Title Z-A"),
    ];
    if has_query {
        items.insert(0, ("relevance", "Relevance"));
    }
    items
}

fn rail_primary_action(is_admin: bool) -> &'static str {
    if is_admin {
        r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>"#
    } else {
        ""
    }
}

fn rail_actions(is_admin: bool) -> &'static str {
    if is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#
    } else {
        r#"<a href="/login" class="btn">Admin sign in</a>"#
    }
}
