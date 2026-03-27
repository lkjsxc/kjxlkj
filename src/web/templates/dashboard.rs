//! Admin dashboard template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn admin_page(notes: &[IndexItem], next_cursor: Option<&str>) -> String {
    let extra_script = format!(r#"<script>{ACTIONS_JS}</script>"#);
    let header_actions = r#"<a href="/search" class="btn btn-primary">Search</a>"#;
    let rail_primary_action =
        r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>"#;
    let rail_actions = r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#;
    list_page(
        &ListPageConfig {
            page_title: "Admin notes",
            eyebrow: "Admin browse",
            summary: "Browse public and private notes without leaving the shared shell.",
            path: "/admin",
            mode_label: "Admin",
            scope_title: "Admin index",
            scope_summary: "Browse current notes here. Use Search for full-text lookup across titles and bodies.",
            active_nav: "admin",
            rail_primary_action,
            header_actions,
            rail_actions,
            is_admin: true,
            extra_script: &extra_script,
        },
        notes,
        next_cursor,
    )
}
