//! Admin dashboard template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

const EDITOR_JS: &str = include_str!("editor.js");

pub fn admin_page(notes: &[IndexItem], next_cursor: Option<&str>, query: Option<&str>) -> String {
    let actions = r#"<button type="button" class="btn" onclick="createNote()">New note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#;
    list_page(
        &ListPageConfig {
            page_title: "Admin notes",
            eyebrow: "Dashboard",
            summary: "Search across current titles and bodies for public and private notes.",
            path: "/admin",
            actions,
            extra_script: &format!(r#"<script>{EDITOR_JS}</script>"#),
        },
        notes,
        next_cursor,
        query,
    )
}
