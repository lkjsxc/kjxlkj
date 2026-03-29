//! Dedicated search page template

use super::index::{list_rail, note_row, pager};
use super::layout::{base, html_escape, shell_page};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn search_page(
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
    is_admin: bool,
) -> String {
    let query = query.unwrap_or("").trim();
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let results = if query.is_empty() {
        r#"<section class="surface"><p class="surface-empty">Enter a word or phrase.</p></section>"#
            .to_string()
    } else if notes.is_empty() {
        r#"<section class="surface"><p class="surface-empty">No matching notes.</p></section>"#
            .to_string()
    } else {
        format!(
            r#"<section class="stack note-list">{}</section>{}"#,
            notes.iter().map(note_row).collect::<Vec<_>>().join(""),
            pager("/search", Some(query), next_cursor)
        )
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack"><h1>Search</h1></div>
</header>
<section class="surface search-surface">
<form class="search-form" method="GET" action="/search">
<label for="search-page-input">Search notes</label>
<div class="search-row">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search aliases, titles, and bodies">
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>
</section>
{}"#,
        html_escape(query),
        results
    );
    base(
        "Search",
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "search",
                rail_primary_action(is_admin),
                rail_actions(is_admin),
                is_admin,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::IndexItem;

    fn sample_item() -> IndexItem {
        IndexItem {
            href: "/Q29udHJhY3RSdW50aW1lMQ".to_string(),
            title: "Orbit Ledger".to_string(),
            summary: "Shared release.".to_string(),
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            is_favorite: true,
            visibility: Some("Public"),
        }
    }

    #[test]
    fn search_page_prompts_without_query() {
        let html = search_page(&[], None, None, false);
        assert!(html.contains("Enter a word or phrase."));
        assert!(!html.contains("rail-search"));
    }

    #[test]
    fn search_page_keeps_query_in_main_form() {
        let html = search_page(&[sample_item()], Some("cursor"), Some("orbit"), true);
        assert!(html.contains("name=\"q\" value=\"orbit\""));
        assert!(html.contains("New note"));
        assert!(!html.contains("Browse notes"));
    }
}
