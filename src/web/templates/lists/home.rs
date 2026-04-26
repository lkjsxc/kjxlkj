//! Homepage template

use super::index::{admin_create_actions, list_rail};
use super::layout::{base, html_escape, shell_page};
use super::list_sections::{
    favorite_browse_card, note_grid_section, quick_search_section, recent_browse_card,
};
use super::model::IndexItem;
use super::popular_sections::home_popular_section;
use crate::web::db::{AppSettings, PopularWindow};
use crate::web::site::SiteContext;

const ACTIONS_JS: &str = include_str!("../scripts/resource_actions.js");
const POPULAR_JS: &str = include_str!("../scripts/popular_window.js");

pub struct HomeView<'a> {
    pub settings: &'a AppSettings,
    pub intro_html: &'a str,
    pub popular: &'a [IndexItem],
    pub recent: &'a [IndexItem],
    pub favorites: &'a [IndexItem],
    pub window: PopularWindow,
    pub is_admin: bool,
    pub guest_login_href: &'a str,
    pub site: &'a SiteContext,
}

pub fn home_page(view: HomeView<'_>) -> String {
    let admin_script = if view.is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let rail_actions = rail_actions(view.is_admin, view.guest_login_href);
    let content = format!(
        "{}{}{}",
        intro_block(view.intro_html),
        quick_search_section(),
        home_sections(
            view.settings,
            view.popular,
            view.recent,
            view.favorites,
            view.window
        ),
    );
    let admin_actions = view.is_admin.then(admin_create_actions);
    base(
        &view.site.page_meta(
            "Home",
            view.site.site_description.clone(),
            !view.is_admin,
            Some("/"),
        ),
        &shell_page(
            if view.is_admin { "Admin" } else { "Guest" },
            &list_rail(
                "home",
                admin_actions
                    .as_deref()
                    .unwrap_or_else(|| rail_primary_action(view.is_admin)),
                &rail_actions,
                view.is_admin,
            ),
            &content,
            "home-page",
            &view.site.site_name,
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
                "No resources yet.",
                "resource-section",
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
                "No favorite resources yet.",
                "resource-section",
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

fn intro_block(html: &str) -> String {
    if html.trim().is_empty() {
        return String::new();
    }
    format!(
        r#"<section class="page-intro prose home-hero">{}</section>"#,
        html
    )
}

fn rail_primary_action(is_admin: bool) -> &'static str {
    if is_admin {
        unreachable!()
    } else {
        ""
    }
}

fn rail_actions(is_admin: bool, guest_login_href: &str) -> String {
    if is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#
            .to_string()
    } else {
        format!(
            r#"<a href="{}" class="btn">Admin sign in</a>"#,
            html_escape(guest_login_href),
        )
    }
}
