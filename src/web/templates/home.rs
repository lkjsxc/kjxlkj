//! Homepage template

use super::index::list_rail;
use super::layout::{base, shell_page};
use super::list_sections::{
    browse_card, note_grid_section, popular_window_switch, quick_search_section,
};
use super::model::IndexItem;
use super::sections::page_header;
use crate::core::render_markdown;
use crate::web::db::PopularWindow;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn home_page(
    intro_markdown: &str,
    popular: &[IndexItem],
    recent: &[IndexItem],
    favorites: &[IndexItem],
    window: PopularWindow,
    is_admin: bool,
) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        "{}{}{}{}{}",
        page_header("Home", None, "home-head"),
        intro_block(intro_markdown),
        quick_search_section(),
        note_grid_section(
            "Popular notes",
            popular,
            "No popular notes yet.",
            "note-section",
            Some(&popular_window_switch("/", window)),
            None,
        ),
        note_grid_section(
            "Recently updated",
            recent,
            "No notes yet.",
            "note-section",
            None,
            Some(browse_card()),
        )
    ) + &note_grid_section(
        "Favorites",
        favorites,
        "No favorites yet.",
        "note-section",
        None,
        None,
    );
    base(
        "Home",
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "home",
                rail_primary_action(is_admin),
                rail_actions(is_admin),
                is_admin,
            ),
            &content,
            "home-page",
        ),
        "",
        &extra_script,
    )
}

fn intro_block(markdown: &str) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }
    format!(
        r#"<section class="page-intro prose">{}</section>"#,
        render_markdown(markdown)
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
