//! Search form and scope controls

use super::layout::html_escape;
use super::sections::section_with_actions;
use crate::web::db::{ListScope, PopularWindow};

pub(super) fn search_section(
    query: &str,
    sort: &str,
    has_query: bool,
    scope: &ListScope,
    popular_window: PopularWindow,
) -> String {
    let class_name = if has_query || !matches!(scope, ListScope::All) {
        "has-state"
    } else {
        "no-state"
    };
    section_with_actions(
        "Search notes",
        Some(&scope_actions(scope, popular_window)),
        &format!(
            r#"<form class="search-form" method="GET" action="/search">
<input type="hidden" name="scope" value="{}">
<input type="hidden" name="popular_window" value="{}">
<label for="search-page-input" class="visually-hidden">Search notes</label>
<div class="search-grid {class_name}">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search aliases, titles, and bodies">
{}
<label class="form-group search-sort" for="search-sort">
<span class="visually-hidden">Sort</span>
<select id="search-sort" name="sort" aria-label="Sort">{}</select>
</label>
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>"#,
            scope.as_str(),
            popular_window.as_str(),
            html_escape(query),
            state_cards(query, has_query, scope, popular_window),
            sort_options(sort, has_query, scope),
        ),
        "search-section",
    )
}

pub(super) fn scope_meta(scope: &ListScope, popular_window: PopularWindow) -> Option<&'static str> {
    match scope {
        ListScope::All => None,
        ListScope::Favorites => Some("Favorites"),
        ListScope::Popular => match popular_window {
            PopularWindow::Days7 => Some("Popular 7d"),
            PopularWindow::Days30 => Some("Popular 30d"),
            PopularWindow::Days90 => Some("Popular 90d"),
        },
    }
}

fn state_cards(
    query: &str,
    has_query: bool,
    scope: &ListScope,
    popular_window: PopularWindow,
) -> String {
    let mut cards = Vec::new();
    if has_query {
        cards.push(format!(
            r#"<div class="search-state-card"><small>Query</small><strong>{}</strong></div>"#,
            html_escape(query)
        ));
    }
    if let Some(meta) = scope_meta(scope, popular_window) {
        cards.push(format!(
            r#"<div class="search-state-card"><small>Scope</small><strong>{meta}</strong></div>"#
        ));
    }
    if cards.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="search-state-stack">{}</div>"#,
            cards.join("")
        )
    }
}

fn scope_actions(scope: &ListScope, popular_window: PopularWindow) -> String {
    let mut actions = vec![
        scope_link("All", "/search", matches!(scope, ListScope::All)),
        scope_link(
            "Favorites",
            "/search?scope=favorites&sort=favorite_order",
            matches!(scope, ListScope::Favorites),
        ),
        scope_link(
            "Popular",
            &format!(
                "/search?scope=popular&popular_window={}&sort=popular",
                popular_window.as_str()
            ),
            matches!(scope, ListScope::Popular),
        ),
    ];
    if matches!(scope, ListScope::Popular) {
        actions.push(window_actions(popular_window));
    }
    actions.join("")
}

fn window_actions(popular_window: PopularWindow) -> String {
    [
        PopularWindow::Days7,
        PopularWindow::Days30,
        PopularWindow::Days90,
    ]
    .into_iter()
    .map(|item| {
        scope_link(
            item.as_str(),
            &format!(
                "/search?scope=popular&popular_window={}&sort=popular",
                item.as_str()
            ),
            item == popular_window,
        )
    })
    .collect::<Vec<_>>()
    .join("")
}

fn scope_link(label: &str, href: &str, active: bool) -> String {
    format!(
        r#"<a href="{href}" class="btn{}">{label}</a>"#,
        if active { " btn-primary" } else { "" }
    )
}

fn sort_options(selected: &str, has_query: bool, scope: &ListScope) -> String {
    sort_catalog(has_query, scope)
        .into_iter()
        .map(|(value, label)| {
            format!(
                r#"<option value="{value}"{}>{label}</option>"#,
                if value == selected { " selected" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn sort_catalog(has_query: bool, scope: &ListScope) -> Vec<(&'static str, &'static str)> {
    let mut items = vec![
        ("updated_desc", "Recently updated"),
        ("updated_asc", "Oldest updates"),
        ("created_desc", "Newest created"),
        ("created_asc", "Oldest created"),
        ("title_asc", "Title A-Z"),
        ("title_desc", "Title Z-A"),
    ];
    if has_query {
        items.insert(0, ("relevance", "Relevance"));
    }
    match scope {
        ListScope::Favorites => items.insert(0, ("favorite_order", "Favorite order")),
        ListScope::Popular => items.insert(0, ("popular", "Popularity")),
        ListScope::All => {}
    }
    items
}
