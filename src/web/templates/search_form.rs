use super::layout::html_escape;
use super::sections::section;

pub fn search_section(
    query: &str,
    kind: &str,
    scope: &str,
    sort: &str,
    popular_window: &str,
    has_query: bool,
) -> String {
    let query_card = if has_query {
        format!(
            r#"<div class="search-state-card"><small>Query</small><strong>{}</strong></div>"#,
            html_escape(query)
        )
    } else {
        String::new()
    };
    section(
        "Search resources",
        &format!(
            r#"{}<form class="search-form" method="GET" action="/search">
<label for="search-page-input" class="visually-hidden">Search resources</label>
<div class="search-grid">
<input id="search-page-input" type="search" name="q" value="{}" placeholder="Search aliases, titles, bodies, and filenames">
<input type="hidden" name="scope" value="{}">
<label class="form-group search-sort" for="search-kind">
<span class="visually-hidden">Kind</span>
<select id="search-kind" name="kind" aria-label="Kind">{}</select>
</label>
<label class="form-group search-sort" for="search-sort">
<span class="visually-hidden">Sort</span>
<select id="search-sort" name="sort" aria-label="Sort">{}</select>
</label>
<label class="form-group search-sort" for="search-popular-window">
<span class="visually-hidden">Popular window</span>
<select id="search-popular-window" name="popular_window" aria-label="Popular window">{}</select>
</label>
<button type="submit" class="btn btn-primary">Search</button>
</div>
</form>"#,
            query_card,
            html_escape(query),
            scope,
            kind_options(kind),
            sort_options(sort, has_query, scope),
            popular_window_options(popular_window),
        ),
        "search-section",
    )
}

fn sort_options(selected: &str, has_query: bool, scope: &str) -> String {
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

fn kind_options(selected: &str) -> String {
    [
        ("all", "All resources"),
        ("note", "Notes"),
        ("media", "Media"),
    ]
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

fn popular_window_options(selected: &str) -> String {
    [
        ("1d", "1d"),
        ("7d", "7d"),
        ("30d", "30d"),
        ("90d", "90d"),
        ("all", "All time"),
    ]
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

fn sort_catalog(has_query: bool, scope: &str) -> Vec<(&'static str, &'static str)> {
    let mut items = vec![
        ("updated_desc", "Recently updated"),
        ("updated_asc", "Oldest updates"),
        ("created_desc", "Newest created"),
        ("created_asc", "Oldest created"),
        ("title_asc", "Title A-Z"),
        ("title_desc", "Title Z-A"),
        ("popular_desc", "Popular"),
        ("views_total_desc", "Most viewed"),
    ];
    if has_query {
        items.insert(0, ("relevance", "Relevance"));
    }
    if scope == "favorites" {
        items.push(("favorite_position_asc", "Favorite order"));
    }
    items
}
