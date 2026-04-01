//! Dedicated search page template

use super::index::list_rail;
use super::layout::{base, shell_page};
use super::model::IndexItem;
use super::search_form::search_section;
use super::search_results::{results_section, ResultsSection};
use super::sections::page_header;
use crate::web::db::{ListScope, PopularWindow};

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub struct SearchPageModel<'a> {
    pub notes: &'a [IndexItem],
    pub previous_cursor: Option<&'a str>,
    pub next_cursor: Option<&'a str>,
    pub query: Option<&'a str>,
    pub limit: i64,
    pub sort: &'a str,
    pub scope: &'a ListScope,
    pub popular_window: PopularWindow,
    pub is_admin: bool,
}

pub fn search_page(model: SearchPageModel<'_>) -> String {
    let query = model.query.unwrap_or("").trim();
    let has_query = !query.is_empty();
    let extra_script = if model.is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        "{}{}{}",
        page_header("Search", None, "search-head"),
        search_section(
            query,
            model.sort,
            has_query,
            model.scope,
            model.popular_window
        ),
        results_section(ResultsSection {
            notes: model.notes,
            previous_cursor: model.previous_cursor,
            next_cursor: model.next_cursor,
            query,
            limit: model.limit,
            sort: model.sort,
            has_query,
            scope: model.scope,
            popular_window: model.popular_window,
        }),
    );
    base(
        "Search",
        &shell_page(
            if model.is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "search",
                rail_primary_action(model.is_admin),
                rail_actions(model.is_admin),
                model.is_admin,
            ),
            &content,
            "index-layout",
        ),
        "",
        &extra_script,
    )
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
