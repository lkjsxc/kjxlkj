//! Admin dashboard template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn admin_page(notes: &[IndexItem], next_cursor: Option<&str>) -> String {
    let extra_script = format!(r#"<script>{ACTIONS_JS}</script>"#);
    let rail_primary_action =
        r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>"#;
    let rail_actions = r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#;
    list_page(
        &ListPageConfig {
            page_title: "Admin notes",
            path: "/admin",
            mode_label: "Admin",
            active_nav: "admin",
            rail_primary_action,
            rail_actions,
            list_class: "",
            empty_text: "No notes yet.",
            is_admin: true,
            extra_script: &extra_script,
        },
        notes,
        next_cursor,
    )
}
