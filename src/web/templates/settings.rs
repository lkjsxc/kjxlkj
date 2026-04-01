//! Dedicated settings page template

use super::dashboard_favorites::favorite_order_section;
use super::index::list_rail;
use super::layout::{base, html_escape, shell_page};
use super::model::IndexItem;
use super::sections::{page_header, section};
use crate::web::db::AppSettings;

const ACTIONS_JS: &str = include_str!("note_actions.js");
const FAVORITE_ORDER_JS: &str = include_str!("favorite_order.js");

pub fn settings_page(settings: &AppSettings, favorites: &[IndexItem]) -> String {
    let content = format!(
        "{}<div class=\"dashboard-stack\">{}{}</div>",
        page_header("Settings", None, "settings-head"),
        settings_panel(settings),
        favorite_order_section(favorites),
    );
    base(
        "Settings",
        &shell_page(
            "Admin",
            &list_rail(
                "settings",
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

fn settings_panel(settings: &AppSettings) -> String {
    section(
        "Global settings",
        &format!(
            r#"<form class="settings-grid" method="POST" action="/settings">
<label class="form-group settings-wide"><span>Home intro Markdown</span><textarea name="home_intro_markdown" rows="6" placeholder="Optional homepage introduction">{}</textarea></label>
{}
<label class="form-group"><span>Search page size</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>
<label class="form-group"><span>New note visibility</span><select name="default_new_note_visibility"><option value="private"{}>Private by default</option><option value="public"{}>Public by default</option></select></label>
<button type="submit" class="btn btn-primary">Save settings</button>
</form>"#,
            html_escape(&settings.home_intro_markdown),
            home_sections_form(settings),
            settings.search_results_per_page,
            if settings.default_new_note_is_private {
                " selected"
            } else {
                ""
            },
            if settings.default_new_note_is_private {
                ""
            } else {
                " selected"
            },
        ),
        "settings-section",
    )
}

fn home_sections_form(settings: &AppSettings) -> String {
    [
        section_fields(
            "Popular notes",
            "popular",
            settings.home_popular_visible,
            settings.home_popular_position,
            settings.home_popular_limit,
        ),
        section_fields(
            "Recently updated",
            "recent",
            settings.home_recent_visible,
            settings.home_recent_position,
            settings.home_recent_limit,
        ),
        section_fields(
            "Favorites",
            "favorite",
            settings.home_favorite_visible,
            settings.home_favorite_position,
            settings.home_favorite_limit,
        ),
    ]
    .join("")
}

fn section_fields(label: &str, prefix: &str, visible: bool, position: i64, count: i64) -> String {
    format!(
        r#"<fieldset class="surface settings-card">
<legend>{label}</legend>
<label class="check-row"><input type="checkbox" name="home_{prefix}_visible"{}><span>Show on Home</span></label>
<label class="form-group"><span>Home position</span><select name="home_{prefix}_position">{}</select></label>
<label class="form-group"><span>Home count</span><input type="number" name="home_{prefix}_limit" min="1" max="24" value="{}"></label>
</fieldset>"#,
        if visible { " checked" } else { "" },
        position_options(position),
        count,
    )
}

fn position_options(selected: i64) -> String {
    (1..=3)
        .map(|value| {
            format!(
                r#"<option value="{value}"{}>{value}</option>"#,
                if value == selected { " selected" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("")
}
