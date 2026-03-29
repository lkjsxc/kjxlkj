//! Homepage template

use super::index::{list_rail, note_row};
use super::layout::{base, shell_page};
use super::model::IndexItem;
use crate::web::db::NoteStats;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn home_page(
    stats: &NoteStats,
    recent: &[IndexItem],
    favorites: &[IndexItem],
    is_admin: bool,
) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<h1>Home</h1>
<p class="page-summary">Recent notes, favorites, and the current cadence.</p>
</div>
</header>
<section class="surface search-surface">
<form class="search-form" method="GET" action="/search">
<label for="home-search-input">Quick search</label>
<div class="search-row">
<input id="home-search-input" type="search" name="q" placeholder="Search aliases, titles, and bodies">
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>
</section>
{}
{}
{}"#,
        stats_grid(stats),
        note_section("Recently updated", recent, "No notes yet.", "note-grid"),
        note_section("Favorites", favorites, "No favorites yet.", "note-grid"),
    );
    base(
        "Home",
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &list_rail("home", rail_primary_action(is_admin), rail_actions(is_admin), is_admin),
            &content,
            "home-page",
        ),
        "",
        &extra_script,
    )
}

fn stats_grid(stats: &NoteStats) -> String {
    format!(
        r#"<section class="stats-grid">
{}
{}
{}
{}
{}
{}
</section>"#,
        stat_card("Notes", stats.total),
        stat_card("Public", stats.public_count),
        stat_card("Favorites", stats.favorite_count),
        stat_card("This month", stats.updated_this_month),
        stat_card("Created this month", stats.created_this_month),
        stat_card("Created this year", stats.created_this_year),
    )
}

fn note_section(title: &str, notes: &[IndexItem], empty: &str, list_class: &str) -> String {
    let rows = if notes.is_empty() {
        format!(r#"<p class="surface-empty">{empty}</p>"#)
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>().join("")
    };
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>{title}</h2></div>
<div class="note-list {list_class}">{rows}</div>
</section>"#
    )
}

fn stat_card(label: &str, value: i64) -> String {
    format!(
        r#"<article class="surface stat-card"><small>{label}</small><strong>{value}</strong></article>"#
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
