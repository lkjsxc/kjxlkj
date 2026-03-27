//! Dedicated search page template

use super::index::{note_row, pager, ListPageConfig};
use super::layout::{base, html_escape, primary_nav, rail_section, shell_page};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn search_page(
    notes: &[IndexItem],
    next_cursor: Option<&str>,
    query: Option<&str>,
    is_admin: bool,
) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let query = query.unwrap_or("").trim();
    let rows = if query.is_empty() {
        r#"<section class="surface"><p class="surface-empty">Enter a word or phrase to search current titles and bodies.</p></section>"#
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
<div class="page-title-stack">
<p class="eyebrow">Search</p>
<h1>Find notes</h1>
<p class="page-summary">{}</p>
</div>
<div class="page-actions">{}</div>
</header>
<section class="surface search-surface">
<form class="search-form" method="GET" action="/search">
<label for="search-page-input">Search notes</label>
<div class="search-row">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search titles and bodies">
<button type="submit" class="btn btn-primary">Run search</button>
</div>
</form>
<p class="search-copy">{}</p>
</section>
{}"#,
        if is_admin {
            "Search public and private notes from one page."
        } else {
            "Search current public notes from one page."
        },
        header_actions(is_admin),
        html_escape(query),
        if query.is_empty() {
            "Search is server-side and query-param driven."
        } else {
            "Results keep the same dense language as browse pages."
        },
        rows
    );
    base(
        "Search",
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &rail(&ListPageConfig {
                page_title: "",
                eyebrow: "",
                summary: "",
                path: "/search",
                mode_label: "",
                scope_title: "Search",
                scope_summary: if is_admin {
                    "Search public and private titles and bodies."
                } else {
                    "Search public titles and bodies."
                },
                active_nav: "search",
                rail_primary_action: rail_primary_action(is_admin),
                header_actions: "",
                rail_actions: rail_actions(is_admin),
                is_admin,
                extra_script: "",
            }),
            &content,
            "index-layout",
        ),
        "",
        &extra_script,
    )
}

fn rail(config: &ListPageConfig<'_>) -> String {
    let mut sections = Vec::new();
    if !config.rail_primary_action.is_empty() {
        sections.push(rail_section(
            "Create",
            &format!(
                r#"<div class="rail-actions">{}</div>"#,
                config.rail_primary_action
            ),
        ));
    }
    sections.extend([
        rail_section("Navigate", &primary_nav(config.active_nav, config.is_admin)),
        rail_section(
            "Scope",
            &format!(
                r#"<div class="rail-copy"><strong>{}</strong><p>{}</p></div>"#,
                config.scope_title, config.scope_summary
            ),
        ),
        rail_section(
            "Actions",
            &format!(r#"<div class="rail-actions">{}</div>"#, config.rail_actions),
        ),
    ]);
    sections.join("")
}

fn header_actions(is_admin: bool) -> &'static str {
    if is_admin {
        r#"<a href="/admin" class="btn">Admin workspace</a>"#
    } else {
        r#"<a href="/" class="btn">Browse notes</a><a href="/login" class="btn">Admin sign in</a>"#
    }
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
            visibility: Some("Public"),
        }
    }

    #[test]
    fn search_page_prompts_without_query() {
        let html = search_page(&[], None, None, false);
        assert!(html.contains("Enter a word or phrase"));
        assert!(!html.contains("rail-search"));
    }

    #[test]
    fn search_page_keeps_query_in_main_form() {
        let html = search_page(&[sample_item()], Some("cursor"), Some("orbit"), true);
        assert!(html.contains("name=\"q\" value=\"orbit\""));
        assert!(html.contains("New note"));
    }
}
