//! Homepage template

use super::index::list_rail;
use super::layout::{base, html_escape, shell_page};
use super::list_sections::{
    browse_card, note_grid_section, popular_window_switch, quick_search_section,
};
use super::model::IndexItem;
use super::sections::page_header;
use crate::core::render_markdown;
use crate::web::db::{AppSettings, PopularWindow};

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn home_page(
    settings: &AppSettings,
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
        "{}{}{}{}",
        page_header("Home", None, "home-head"),
        intro_block(&settings.home_intro_markdown, is_admin),
        quick_search_section(),
        ordered_sections(settings, popular, recent, favorites, window),
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

fn ordered_sections(
    settings: &AppSettings,
    popular: &[IndexItem],
    recent: &[IndexItem],
    favorites: &[IndexItem],
    window: PopularWindow,
) -> String {
    let mut sections = Vec::new();
    if settings.home_popular_visible {
        sections.push((
            settings.home_popular_position,
            note_grid_section(
                "Popular notes",
                popular,
                "No popular notes yet.",
                "note-section home-popular-section",
                Some(&popular_window_switch("/", window)),
                Some(browse_card(
                    &format!(
                        "/search?scope=popular&popular_window={}&sort=popular",
                        window.as_str()
                    ),
                    "View more notes",
                    "Browse notes ordered by the current popularity window.",
                    "Popular",
                )),
            ),
        ));
    }
    if settings.home_recent_visible {
        sections.push((
            settings.home_recent_position,
            note_grid_section(
                "Recently updated",
                recent,
                "No notes yet.",
                "note-section home-recent-section",
                None,
                Some(browse_card(
                    "/search?sort=updated_desc",
                    "View more notes",
                    "Browse all visible notes with search, sorting, and page navigation.",
                    "Search",
                )),
            ),
        ));
    }
    if settings.home_favorite_visible {
        sections.push((
            settings.home_favorite_position,
            note_grid_section(
                "Favorites",
                favorites,
                "No favorites yet.",
                "note-section",
                None,
                Some(browse_card(
                    "/search?scope=favorites&sort=favorite_order",
                    "View more notes",
                    "Browse favorited notes in the saved favorite order.",
                    "Favorites",
                )),
            ),
        ));
    }
    sections.sort_by_key(|(position, _)| *position);
    sections
        .into_iter()
        .map(|(_, html)| html)
        .collect::<Vec<_>>()
        .join("")
}

fn intro_block(markdown: &str, is_admin: bool) -> String {
    let rendered = if markdown.trim().is_empty() {
        String::new()
    } else {
        format!(
            r#"<section class="page-intro prose">{}</section>"#,
            render_markdown(markdown)
        )
    };
    let editor = if is_admin {
        format!(
            r#"<form class="settings-grid home-intro-form" method="POST" action="/settings/home-intro">
<label class="form-group settings-wide"><span>Home intro</span>
<textarea name="home_intro_markdown" rows="5" placeholder="Optional homepage introduction">{}</textarea></label>
<button type="submit" class="btn">Save intro</button>
</form>"#,
            html_escape(markdown)
        )
    } else {
        String::new()
    };
    if rendered.is_empty() && editor.is_empty() {
        String::new()
    } else {
        format!(r#"<div class="home-intro-stack">{rendered}{editor}</div>"#)
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
