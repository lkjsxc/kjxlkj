//! Homepage template

use super::index::{admin_create_actions, list_rail};
use super::layout::{base, shell_page};
use super::list_sections::{
    favorite_browse_card, note_grid_section, quick_search_section, recent_browse_card,
};
use super::model::IndexItem;
use super::popular_sections::home_popular_section;
use crate::core::render_markdown;
use crate::web::db::{AppSettings, PopularWindow};
use crate::web::site::SiteContext;

const ACTIONS_JS: &str = include_str!("note_actions.js");
const POPULAR_JS: &str = include_str!("popular_window.js");

pub fn home_page(
    settings: &AppSettings,
    popular: &[IndexItem],
    recent: &[IndexItem],
    favorites: &[IndexItem],
    window: PopularWindow,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let admin_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        "{}{}{}",
        intro_block(&settings.home_intro_markdown),
        quick_search_section(),
        home_sections(settings, popular, recent, favorites, window),
    );
    let admin_actions = is_admin.then(admin_create_actions);
    base(
        &site.page_meta("Home", site.site_description.clone(), !is_admin, Some("/")),
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "home",
                admin_actions
                    .as_deref()
                    .unwrap_or_else(|| rail_primary_action(is_admin)),
                rail_actions(is_admin),
                is_admin,
            ),
            &content,
            "home-page",
            &site.site_name,
        ),
        "",
        &format!(r#"<script>{POPULAR_JS}</script>{admin_script}"#),
    )
}

fn home_sections(
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
            home_popular_section(popular, window),
        ));
    }
    if settings.home_recent_visible {
        sections.push((
            settings.home_recent_position,
            note_grid_section(
                "Recently updated",
                recent,
                "No notes yet.",
                "note-section",
                None,
                Some(recent_browse_card()),
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
                Some(favorite_browse_card()),
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

fn intro_block(markdown: &str) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }
    format!(
        r#"<section class="page-intro prose home-hero">{}</section>"#,
        render_markdown(markdown)
    )
}

fn rail_primary_action(is_admin: bool) -> &'static str {
    if is_admin { unreachable!() } else { "" }
}

fn rail_actions(is_admin: bool) -> &'static str {
    if is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#
    } else {
        r#"<a href="/login" class="btn">Admin sign in</a>"#
    }
}
