//! Admin dashboard template

use super::index::list_rail;
use super::layout::{base, shell_page};
use super::list_sections::{browse_card, note_grid_section, popular_window_switch};
use super::model::IndexItem;
use super::sections::{page_header, section};
use crate::web::db::{AppSettings, NoteStats, PopularWindow};

const ACTIONS_JS: &str = include_str!("note_actions.js");

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
        settings_summary(settings),
        note_grid_section(
            "Popular notes",
            popular,
            "No popular notes yet.",
            "note-section home-popular-section",
            Some(&popular_window_switch("/admin", window)),
            Some(browse_card(
                &format!(
                    "/search?scope=popular&popular_window={}&sort=popular",
                    window.as_str()
                ),
                "View more notes",
                "Browse the full popularity-ranked library on the search page.",
                "Popular",
            )),
        ),
        note_grid_section(
            "Recently updated",
            recent,
            "No notes yet.",
            "note-section home-recent-section",
            None,
            Some(browse_card(
                "/search?sort=updated_desc",
                "View more notes",
                "Browse the full recently updated library on the search page.",
                "Search",
            )),
        ),
    ) + &note_grid_section(
        "Favorites",
        favorites,
        "No favorites yet.",
        "note-section",
        None,
        Some(browse_card(
            "/search?scope=favorites&sort=favorite_order",
            "View more notes",
            "Browse favorites in the saved order on the search page.",
            "Favorites",
        )),
    );
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
        &format!(r#"<script>{ACTIONS_JS}</script>"#),
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

fn settings_summary(settings: &AppSettings) -> String {
    section(
        "Settings",
        &format!(
            r#"<div class="settings-summary-grid">
<article class="surface stat-card"><small>Home order</small><strong>Popular / Recent / Favorites</strong></article>
<article class="surface stat-card"><small>Home counts</small><strong>{} / {} / {}</strong></article>
<article class="surface stat-card"><small>New notes</small><strong>{}</strong></article>
</div>
<a href="/settings" class="btn">Open settings</a>"#,
            settings.home_popular_limit,
            settings.home_recent_limit,
            settings.home_favorite_limit,
            if settings.default_new_note_is_private {
                "Private by default"
            } else {
                "Public by default"
            },
        ),
        "settings-section",
    )
}

fn stat_card(label: &str, value: i64) -> String {
    format!(
        r#"<article class="surface stat-card"><small>{label}</small><strong>{value}</strong></article>"#
    )
}
