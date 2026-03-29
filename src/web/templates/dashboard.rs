//! Admin dashboard template

use super::index::{list_rail, note_row};
use super::layout::{base, shell_page};
use super::model::IndexItem;
use crate::web::db::{AppSettings, NoteStats};

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn admin_page(
    stats: &NoteStats,
    settings: &AppSettings,
    recent: &[IndexItem],
    favorites: &[IndexItem],
) -> String {
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack"><h1>Dashboard</h1></div>
</header>
{}
<section class="dashboard-panels">
{}
{}
</section>"#,
        stats_grid(stats),
        settings_panel(settings),
        note_columns(recent, favorites),
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
{}{}{}{}{}{}
</section>"#,
        stat_card("Notes", stats.total),
        stat_card("Public", stats.public_count),
        stat_card("Private", stats.private_count),
        stat_card("Favorites", stats.favorite_count),
        stat_card("Updated this month", stats.updated_this_month),
        stat_card("Updated this year", stats.updated_this_year),
    )
}

fn settings_panel(settings: &AppSettings) -> String {
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>Settings</h2></div>
<form class="settings-grid" method="POST" action="/admin/settings">
<label class="form-group"><span>Home recent count</span><input type="number" name="home_recent_limit" min="1" max="24" value="{}"></label>
<label class="form-group"><span>Home favorite count</span><input type="number" name="home_favorite_limit" min="1" max="24" value="{}"></label>
<label class="form-group"><span>Search page size</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>
<label class="check-row check-row-field"><input type="checkbox" name="default_vim_mode" {}><span>Default Vim mode for editors</span></label>
<button type="submit" class="btn btn-primary">Save settings</button>
</form>
<label class="form-group local-setting-group" for="local-vim-mode">
<span>This browser</span>
<select id="local-vim-mode" data-local-setting="vim-mode">
<option value="default">Follow dashboard default</option>
<option value="on">Always enable Vim mode</option>
<option value="off">Always disable Vim mode</option>
</select>
</label>
</section>"#,
        settings.home_recent_limit,
        settings.home_favorite_limit,
        settings.search_results_per_page,
        if settings.default_vim_mode {
            "checked"
        } else {
            ""
        },
    )
}

fn note_columns(recent: &[IndexItem], favorites: &[IndexItem]) -> String {
    format!(
        "{}{}",
        note_section("Recently updated", recent, "No notes yet."),
        note_section("Favorites", favorites, "No favorites yet."),
    )
}

fn note_section(title: &str, notes: &[IndexItem], empty: &str) -> String {
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>{title}</h2></div>
<div class="note-list note-grid">{}</div>
</section>"#,
        note_rows(notes, empty)
    )
}

fn note_rows(notes: &[IndexItem], empty: &str) -> String {
    if notes.is_empty() {
        return format!(r#"<p class="surface-empty">{empty}</p>"#);
    }
    notes.iter().map(note_row).collect::<Vec<_>>().join("")
}

fn stat_card(label: &str, value: i64) -> String {
    format!(
        r#"<article class="surface stat-card"><small>{label}</small><strong>{value}</strong></article>"#
    )
}
