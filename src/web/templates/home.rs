//! Public root list template

use super::index::{list_page, ListPageConfig};
use super::model::{IndexItem, RecentLink};

pub fn home_page(
    recent: &[RecentLink],
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
    is_admin: bool,
) -> String {
    let actions = if is_admin {
        r#"<a href="/admin" class="btn">Dashboard</a>"#
    } else {
        r#"<a href="/login" class="btn">Admin sign in</a>"#
    };
    list_page(
        &ListPageConfig {
            page_title: "Public notes",
            eyebrow: "Index",
            summary: "Search current public note titles and content.",
            path: "/",
            mode_label: if is_admin { "Admin" } else { "Guest" },
            scope_title: "Public index",
            scope_summary: "Public notes remain searchable at scale while the rail keeps scope and shortcuts visible.",
            actions,
            extra_script: "",
        },
        recent,
        notes,
        next_cursor,
        query,
    )
}
