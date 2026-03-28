//! Public root list template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn home_page(notes: &[IndexItem], next_cursor: Option<&str>, is_admin: bool) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
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
            path: "/",
            mode_label: if is_admin { "Admin" } else { "Guest" },
            active_nav: "home",
            rail_primary_action,
            rail_actions,
            list_class: "public-note-grid",
            empty_text: "No public notes yet.",
            is_admin,
            extra_script: &extra_script,
        },
        notes,
        next_cursor,
    )
}
