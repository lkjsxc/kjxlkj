//! Shared index-style sections

use super::index::note_row;
use super::model::IndexItem;
use super::sections::{section, section_with_actions};
use crate::web::db::PopularWindow;

pub fn quick_search_section() -> String {
    section(
        "Quick search",
        r#"<form class="search-form" method="GET" action="/search">
<label for="home-search-input" class="visually-hidden">Quick search</label>
<div class="search-row">
<input id="home-search-input" type="search" name="q" placeholder="Search aliases, titles, and bodies">
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>"#,
        "search-section",
    )
}

pub fn note_grid_section(
    title: &str,
    notes: &[IndexItem],
    empty: &str,
    class_name: &str,
    actions: Option<&str>,
    extra_card: Option<String>,
) -> String {
    section_with_actions(
        title,
        actions,
        &note_grid_body(notes, empty, extra_card),
        class_name,
    )
}

pub fn note_grid_body(notes: &[IndexItem], empty: &str, extra_card: Option<String>) -> String {
    let mut cards = if notes.is_empty() {
        vec![empty_card(empty)]
    } else {
        notes.iter().map(note_row).collect::<Vec<_>>()
    };
    if let Some(card) = extra_card {
        cards.push(card);
    }
    format!(r#"<div class="note-list note-grid">{}</div>"#, cards.join(""))
}

pub fn browse_card() -> String {
    view_more_card(
        "/search",
        "View more notes",
        "Browse all visible notes with search, sorting, and page navigation.",
        "Search",
    )
}

pub fn recent_browse_card() -> String {
    browse_card()
}

pub fn favorite_browse_card() -> String {
    view_more_card(
        "/search?scope=favorites",
        "View more notes",
        "Browse favorites in favorite order with search and page navigation.",
        "Favorites",
    )
}

pub fn popular_browse_card(window: PopularWindow) -> String {
    view_more_card(
        &format!(
            "/search?sort=popular_desc&popular_window={}",
            window.as_str()
        ),
        "View more notes",
        "Browse more popularity-ranked notes in the current rolling window.",
        window.as_str(),
    )
}

pub fn view_more_card(href: &str, title: &str, summary: &str, meta: &str) -> String {
    format!(
        r#"<a href="{href}" class="index-card note-row note-row-action">
<div class="card-body">
<p class="card-title">{title}</p>
<p class="card-summary">{summary}</p>
</div>
<div class="card-meta"><small><span>Open</span>{meta}</small></div>
</a>"#
    )
}

fn empty_card(message: &str) -> String {
    format!(
        r#"<article class="index-card note-row note-row-empty">
<div class="card-body"><p class="surface-empty">{message}</p></div>
</article>"#
    )
}
