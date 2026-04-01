//! Dedicated search page template

use super::index::{list_rail, note_row, pager};
use super::layout::{base, html_escape, shell_page};
use super::model::IndexItem;
use super::sections::{page_header, section};

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub struct SearchView<'a> {
    pub notes: &'a [IndexItem],
    pub previous_cursor: Option<&'a str>,
    pub next_cursor: Option<&'a str>,
    pub query: Option<&'a str>,
    pub limit: i64,
    pub scope: &'a str,
    pub sort: &'a str,
    pub popular_window: &'a str,
    pub is_admin: bool,
}

pub fn search_page(view: SearchView<'_>) -> String {
    let query = view.query.unwrap_or("").trim();
    let has_query = !query.is_empty();
    let extra_script = if view.is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        "{}{}{}",
        page_header("Search", None, "search-head"),
        search_section(query, view.scope, view.sort, view.popular_window, has_query),
        results_section(&view, query, has_query),
    );
    base(
        "Search",
        &shell_page(
            if view.is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "search",
                rail_primary_action(view.is_admin),
                rail_actions(view.is_admin),
                view.is_admin,
            ),
            &content,
            "index-layout",
        ),
        "",
        &extra_script,
    )
}

fn search_section(
    query: &str,
    scope: &str,
    sort: &str,
    popular_window: &str,
    has_query: bool,
) -> String {
    let query_card = if has_query {
        format!(
            r#"<div class="search-state-card"><small>Query</small><strong>{}</strong></div>"#,
            html_escape(query)
        )
    } else {
        String::new()
    };
    section(
        "Search notes",
        &format!(
            r#"<form class="search-form" method="GET" action="/search">
<label for="search-page-input" class="visually-hidden">Search notes</label>
<div class="search-grid {}">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search aliases, titles, and bodies">
<input type="hidden" name="popular_window" value="{}">
<input type="hidden" name="scope" value="{}">
{}
<label class="form-group search-sort" for="search-sort">
<span class="visually-hidden">Sort</span>
<select id="search-sort" name="sort" aria-label="Sort">{}</select>
</label>
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>"#,
            if has_query { "has-query" } else { "no-query" },
            html_escape(query),
            popular_window,
            scope,
            query_card,
            sort_options(sort, has_query, scope),
        ),
        "search-section",
    )
}

fn results_section(view: &SearchView<'_>, query: &str, has_query: bool) -> String {
    let cards = if view.notes.is_empty() {
        format!(
            r#"<p class="surface-empty">{}</p>"#,
            if has_query {
                "No matching notes."
            } else {
                "No notes yet."
            }
        )
    } else {
        view.notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    section(
        if has_query { "Results" } else { "Notes" },
        &format!(
            r#"<div class="note-list note-grid">{cards}</div>
{}"#,
            pager(
                "/search",
                view.previous_cursor,
                view.next_cursor,
                &[
                    ("q", query),
                    ("scope", view.scope),
                    ("sort", view.sort),
                    ("popular_window", view.popular_window),
                    ("limit", &view.limit.to_string()),
                ],
            )
        ),
        "note-section",
    )
}

fn sort_options(selected: &str, has_query: bool, scope: &str) -> String {
    sort_catalog(has_query, scope)
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

fn sort_catalog(has_query: bool, scope: &str) -> Vec<(&'static str, &'static str)> {
    let mut items = vec![
        ("updated_desc", "Recently updated"),
        ("updated_asc", "Oldest updates"),
        ("created_desc", "Newest created"),
        ("created_asc", "Oldest created"),
        ("title_asc", "Title A-Z"),
        ("title_desc", "Title Z-A"),
        ("popular_desc", "Popular"),
        ("views_total_desc", "Most viewed"),
    ];
    if has_query {
        items.insert(0, ("relevance", "Relevance"));
    }
    if scope == "favorites" {
        items.push(("favorite_position_asc", "Favorite order"));
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
