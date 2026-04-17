//! Dedicated search page template

use super::index::{admin_create_actions, list_rail, note_row, pager};
use super::layout::{base, html_escape, shell_page};
use super::model::IndexItem;
use super::search_form::search_section;
use super::sections::{page_header, section};
use crate::web::site::SiteContext;

const ACTIONS_JS: &str = include_str!("resource_actions.js");

pub struct SearchView<'a> {
    pub notes: &'a [IndexItem],
    pub previous_cursor: Option<&'a str>,
    pub next_cursor: Option<&'a str>,
    pub kind: &'a str,
    pub query: Option<&'a str>,
    pub limit: i64,
    pub scope: &'a str,
    pub sort: &'a str,
    pub is_admin: bool,
    pub guest_login_href: String,
    pub site: &'a SiteContext,
}

pub fn search_page(view: SearchView<'_>) -> String {
    let query = view.query.unwrap_or("").trim();
    let has_query = !query.is_empty();
    let extra_script = if view.is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let rail_actions = rail_actions(&view);
    let content = format!(
        "{}{}{}",
        page_header("Search", None, "search-head"),
        search_section(query, view.kind, view.scope, view.sort, has_query,),
        results_section(&view, query, has_query),
    );
    let admin_actions = view.is_admin.then(admin_create_actions);
    base(
        &view
            .site
            .page_meta("Search", view.site.site_description.clone(), false, None),
        &shell_page(
            if view.is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "search",
                admin_actions
                    .as_deref()
                    .unwrap_or_else(|| rail_primary_action(view.is_admin)),
                &rail_actions,
                view.is_admin,
            ),
            &content,
            "index-layout",
            &view.site.site_name,
        ),
        "",
        &extra_script,
    )
}

fn results_section(view: &SearchView<'_>, query: &str, has_query: bool) -> String {
    let cards = if view.notes.is_empty() {
        format!(
            r#"<p class="surface-empty">{}</p>"#,
            if has_query {
                "No matching resources."
            } else {
                "No resources yet."
            }
        )
    } else {
        view.notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    section(
        if has_query { "Results" } else { "Resources" },
        &format!(
            r#"<div class="resource-list resource-grid">{cards}</div>
{}"#,
            pager(
                "/search",
                view.previous_cursor,
                view.next_cursor,
                &[
                    ("q", query),
                    ("kind", view.kind),
                    ("scope", view.scope),
                    ("sort", view.sort),
                    ("limit", &view.limit.to_string()),
                ],
            )
        ),
        "resource-section",
    )
}

fn rail_primary_action(is_admin: bool) -> &'static str {
    if is_admin {
        unreachable!()
    } else {
        ""
    }
}

fn rail_actions(view: &SearchView<'_>) -> String {
    if view.is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#
            .to_string()
    } else {
        format!(
            r#"<a href="{}" class="btn">Admin sign in</a>"#,
            html_escape(&view.guest_login_href),
        )
    }
}
