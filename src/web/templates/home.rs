//! Public root list template

use super::index::{list_page, ListPageConfig};
use super::model::IndexItem;

pub fn home_page(
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
            actions,
            extra_script: "",
        },
        notes,
        next_cursor,
        query,
    )
}
