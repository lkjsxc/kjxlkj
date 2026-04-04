//! Admin dashboard template

use super::dashboard_favorites::favorite_order_section;
use super::index::list_rail;
use super::layout::{base, shell_page};
use super::list_sections::{
    favorite_browse_card, note_grid_section, popular_browse_card, popular_window_switch,
    recent_browse_card,
};
use super::model::IndexItem;
use super::sections::{page_header, section};
use crate::web::db::{AppSettings, NoteStats, PopularWindow};

const ACTIONS_JS: &str = include_str!("note_actions.js");
const FAVORITE_ORDER_JS: &str = include_str!("favorite_order.js");

pub fn admin_page(
    stats: &NoteStats,
    settings: &AppSettings,
    popular: &[IndexItem],
    recent: &[IndexItem],
    favorites: &[IndexItem],
    window: PopularWindow,
) -> String {
    let content = format!(
        "{}{}<div class=\"dashboard-stack\">{}{}{}</div>",
        page_header("Dashboard", None, "dashboard-head"),
        stats_grid(stats),
        settings_panel(settings),
        note_grid_section(
            "Popular notes",
            popular,
            "No popular notes yet.",
            "note-section",
            Some(&popular_window_switch("/admin", window)),
            Some(popular_browse_card(window)),
        ),
        note_grid_section(
            "Recently updated",
            recent,
            "No notes yet.",
            "note-section",
            None,
            Some(recent_browse_card()),
        ),
    ) + &favorite_order_section(favorites, &favorite_browse_card());
    base(
        "Dashboard",
        &shell_page(
            "Admin",
            &list_rail(
                "admin",
                r#"<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>"#,
                r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#,
                true,
            ),
            &content,
            "dashboard-page",
        ),
        "",
        &format!(r#"<script>{ACTIONS_JS}</script><script>{FAVORITE_ORDER_JS}</script>"#),
    )
}

fn stats_grid(stats: &NoteStats) -> String {
    format!(
        r#"<section class="stats-grid">
{}{}{}{}{}{}{}{}{}{}
</section>"#,
        stat_card("Notes", stats.total),
        stat_card("Public", stats.public_count),
        stat_card("Private", stats.private_count),
        stat_card("Favorites", stats.favorite_count),
        stat_card("Updated this month", stats.updated_this_month),
        stat_card("Updated this year", stats.updated_this_year),
        stat_card("Views total", stats.view_count_total),
        stat_card("Views 7d", stats.view_count_7d),
        stat_card("Views 30d", stats.view_count_30d),
        stat_card("Views 90d", stats.view_count_90d),
    )
}

fn settings_panel(settings: &AppSettings) -> String {
    let hero_state = if settings.home_intro_markdown.trim().is_empty() {
        "Hidden"
    } else {
        "Configured"
    };
    section(
        "Settings",
        &format!(
            r#"<div class="settings-summary-grid">
<article class="surface settings-summary-card"><small>Home hero</small><strong>{hero_state}</strong></article>
<article class="surface settings-summary-card"><small>Session timeout</small><strong>{}</strong></article>
<article class="surface settings-summary-card"><small>New notes</small><strong>{}</strong></article>
<article class="surface settings-summary-card"><small>Search page size</small><strong>{}</strong></article>
<article class="surface settings-summary-card"><small>Home order</small><strong>{}</strong></article>
</div>
<a href="/admin/settings" class="btn btn-primary">Open settings</a>"#,
            session_timeout_label(settings.session_timeout_minutes),
            if settings.default_new_note_is_private {
                "Private by default"
            } else {
                "Public by default"
            },
            settings.search_results_per_page,
            home_order(settings),
        ),
        "settings-section",
    )
}

fn session_timeout_label(minutes: i64) -> String {
    if minutes % 1440 == 0 {
        format!("{}d", minutes / 1440)
    } else if minutes % 60 == 0 {
        format!("{}h", minutes / 60)
    } else {
        format!("{minutes}m")
    }
}

fn home_order(settings: &AppSettings) -> String {
    let mut items = vec![
        (settings.home_popular_position, "Popular"),
        (settings.home_recent_position, "Recent"),
        (settings.home_favorite_position, "Favorites"),
    ];
    items.sort_by_key(|(position, _)| *position);
    items
        .into_iter()
        .map(|(_, label)| label)
        .collect::<Vec<_>>()
        .join(" / ")
}

fn stat_card(label: &str, value: i64) -> String {
    format!(
        r#"<article class="surface stat-card"><small>{label}</small><strong>{value}</strong></article>"#
    )
}
