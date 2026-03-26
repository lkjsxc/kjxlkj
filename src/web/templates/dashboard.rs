//! Admin dashboard template

use super::index::{list_page, ListPageConfig};
use super::model::{IndexItem, RecentLink};

const EDITOR_JS: &str = include_str!("editor.js");

pub fn admin_page(
    recent: &[RecentLink],
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
) -> String {
    let actions = r#"<button type="button" class="btn" onclick="createNote()">New note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#;
    list_page(
        &ListPageConfig {
            page_title: "Admin notes",
            eyebrow: "Dashboard",
            summary: "Search across current titles and bodies for public and private notes.",
            path: "/admin",
            mode_label: "Admin",
            scope_title: "Admin index",
            scope_summary: "The rail keeps search, recent notes, and actions available while the main pane stays dense and scalable.",
            actions,
            extra_script: &format!(r#"<script>{EDITOR_JS}</script>"#),
        },
        recent,
        notes,
        next_cursor,
        query,
    )
}
