//! Admin dashboard template

use super::index::{list_rail, note_row, pager};
use super::layout::{base, shell_page};
use super::model::IndexItem;
use crate::web::db::{AppSettings, NoteStats};

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn admin_page(
    stats: &NoteStats,
    settings: &AppSettings,
    recent: &[IndexItem],
    favorites: &[IndexItem],
    library: &[IndexItem],
    next_cursor: Option<&str>,
) -> String {
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<h1>Dashboard</h1>
<p class="page-summary">Stats, settings, quick access, and the full library.</p>
</div>
</header>
{}
<section class="dashboard-panels">
{}
{}
</section>
{}
{}
<section class="surface section-block">
<div class="section-head"><h2>Library</h2></div>
<div class="stack note-list">{}</div>
{}
</section>"#,
        stats_grid(stats),
        settings_form(settings),
        local_preferences_panel(),
        note_section("Recently updated", recent, "No notes yet."),
        note_section("Favorites", favorites, "No favorites yet."),
        library_rows(library, "No notes yet."),
        pager("/admin", None, next_cursor),
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
{}{}{}{}{}{}{}{}
</section>"#,
        stat_card("Notes", stats.total),
        stat_card("Public", stats.public_count),
        stat_card("Private", stats.private_count),
        stat_card("Favorites", stats.favorite_count),
        stat_card("Created this month", stats.created_this_month),
        stat_card("Updated this month", stats.updated_this_month),
        stat_card("Created this year", stats.created_this_year),
        stat_card("Updated this year", stats.updated_this_year),
    )
}

fn settings_form(settings: &AppSettings) -> String {
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>Settings</h2></div>
<form class="settings-grid" method="POST" action="/admin/settings">
<label class="form-group"><span>Home recent count</span><input type="number" name="home_recent_limit" min="1" max="24" value="{}"></label>
<label class="form-group"><span>Home favorite count</span><input type="number" name="home_favorite_limit" min="1" max="24" value="{}"></label>
<label class="form-group"><span>Search page size</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>
<button type="submit" class="btn btn-primary">Save settings</button>
</form>
</section>"#,
        settings.home_recent_limit,
        settings.home_favorite_limit,
        settings.search_results_per_page
    )
}

fn local_preferences_panel() -> &'static str {
    r#"<section class="surface section-block">
<div class="section-head"><h2>Local editor preferences</h2></div>
<label class="check-row"><input type="checkbox" data-local-setting="vim-mode"><span>Enable Vim mode in this browser</span></label>
</section>"#
}

fn note_section(title: &str, notes: &[IndexItem], empty: &str) -> String {
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>{title}</h2></div>
<div class="note-list note-grid">{}</div>
</section>"#,
        library_rows(notes, empty)
    )
}

fn library_rows(notes: &[IndexItem], empty: &str) -> String {
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
