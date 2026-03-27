//! Public root list template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("editor.js");

pub fn home_page(notes: &[IndexItem], next_cursor: Option<&str>, is_admin: bool) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let header_actions = if is_admin {
        r#"<a href="/search" class="btn btn-primary">Search</a><a href="/admin" class="btn">Admin workspace</a>"#
    } else {
        r#"<a href="/search" class="btn btn-primary">Search</a><a href="/login" class="btn">Admin sign in</a>"#
    };
    let rail_primary_action = if is_admin {
        r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>"#
    } else {
        ""
    };
    let rail_actions = if is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#
    } else {
        r#"<a href="/login" class="btn">Admin sign in</a>"#
    };
    list_page(
        &ListPageConfig {
            page_title: "Public notes",
            eyebrow: if is_admin { "Public browse" } else { "Browse" },
            summary: "Latest public notes in a dense flat list.",
            path: "/",
            mode_label: if is_admin { "Admin" } else { "Guest" },
            scope_title: "Public index",
            scope_summary:
                "Browse current public notes. Use Search for server-side full-text lookup.",
            active_nav: "home",
            rail_primary_action,
            header_actions,
            rail_actions,
            is_admin,
            extra_script: &extra_script,
        },
        notes,
        next_cursor,
    )
}
