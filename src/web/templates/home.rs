//! Homepage template

use super::index::{list_rail, note_row};
use super::layout::{base, shell_page};
use super::model::IndexItem;

const ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn home_page(recent: &[IndexItem], favorites: &[IndexItem], is_admin: bool) -> String {
    let extra_script = if is_admin {
        format!(r#"<script>{ACTIONS_JS}</script>"#)
    } else {
        String::new()
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack"><h1>Home</h1></div>
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
{}{}"#,
        note_section(
            "Recently updated",
            recent,
            "No notes yet.",
            Some(browse_card()),
        ),
        note_section("Favorites", favorites, "No favorites yet.", None),
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

fn note_section(
    title: &str,
    notes: &[IndexItem],
    empty: &str,
    extra_card: Option<String>,
) -> String {
    let mut cards = if notes.is_empty() {
        vec![empty_card(empty)]
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>()
    };
    if let Some(card) = extra_card {
        cards.push(card);
    }
    format!(
        r#"<section class="surface section-block">
<div class="section-head"><h2>{title}</h2></div>
<div class="note-list note-grid">{}</div>
</section>"#,
        cards.join("")
    )
}

fn empty_card(message: &str) -> String {
    format!(
        r#"<article class="index-card note-row note-row-empty">
<div class="card-body"><p class="surface-empty">{message}</p></div>
</article>"#
    )
}

fn browse_card() -> String {
    r#"<a href="/search" class="index-card note-row note-row-action">
<div class="card-body">
<p class="card-title">View more notes</p>
<p class="card-summary">Browse every visible note with sorting, search, and pagination.</p>
</div>
<div class="card-meta"><small><span>Open</span>Search</small></div>
</a>"#
        .to_string()
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
