//! Dedicated admin settings template

use super::index::list_rail;
use super::layout::{base, html_escape, shell_page};
use super::sections::{page_header, section};
use crate::web::db::AppSettings;

const ACTIONS_JS: &str = include_str!("note_actions.js");
const SETTINGS_ORDER_JS: &str = include_str!("settings_order.js");

pub fn settings_page(settings: &AppSettings) -> String {
    let content = format!(
        "{}<form class=\"settings-form settings-stack\" method=\"POST\" action=\"/admin/settings\">{}{}{}<div class=\"settings-submit-row\"><button type=\"submit\" class=\"btn btn-primary\">Save settings</button><a href=\"/admin\" class=\"btn\">Back to dashboard</a></div></form>",
        page_header("Settings", None, "settings-head"),
        home_hero_section(settings),
        home_sections_section(settings),
        defaults_section(settings),
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
            "settings-page",
        ),
        "",
        &format!(r#"<script>{ACTIONS_JS}</script><script>{SETTINGS_ORDER_JS}</script>"#),
    )
}

fn home_hero_section(settings: &AppSettings) -> String {
    section(
        "Home hero",
        &surface_panel(&format!(
            r#"<div class="settings-section-grid">
<label class="form-group settings-wide"><span>Home intro Markdown</span><textarea name="home_intro_markdown" rows="7" placeholder="Optional homepage introduction">{}</textarea></label>
</div>"#,
            html_escape(&settings.home_intro_markdown),
        )),
        "settings-section",
    )
}

fn home_sections_section(settings: &AppSettings) -> String {
    let mut rows = vec![
        (
            "Popular notes",
            "home_popular_visible",
            settings.home_popular_visible,
            "home_popular_position",
            settings.home_popular_position,
            "home_popular_limit",
            settings.home_popular_limit,
        ),
        (
            "Recently updated",
            "home_recent_visible",
            settings.home_recent_visible,
            "home_recent_position",
            settings.home_recent_position,
            "home_recent_limit",
            settings.home_recent_limit,
        ),
        (
            "Favorites",
            "home_favorite_visible",
            settings.home_favorite_visible,
            "home_favorite_position",
            settings.home_favorite_position,
            "home_favorite_limit",
            settings.home_favorite_limit,
        ),
    ];
    rows.sort_by_key(|row| row.4);
    section(
        "Home sections",
        &surface_panel(&format!(
            r#"<div class="settings-table">
<div class="settings-row settings-row-head"><span>Section</span><span>Visible</span><span>Items</span></div>
<div class="settings-table-body" data-settings-order-list>{}</div>
</div>"#,
            rows.into_iter()
                .map(|row| section_row(row.0, row.1, row.2, row.3, row.4, row.5, row.6))
                .collect::<Vec<_>>()
                .join("")
        )),
        "settings-section",
    )
}

fn defaults_section(settings: &AppSettings) -> String {
    section(
        "Defaults",
        &surface_panel(&format!(
            r#"<div class="settings-section-grid">
<label class="form-group"><span>Search page size</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>
<label class="check-row check-row-field settings-wide"><input type="checkbox" name="default_new_note_is_private" {}><span>New notes start private</span></label>
</div>"#,
            settings.search_results_per_page,
            if settings.default_new_note_is_private {
                "checked"
            } else {
                ""
            },
        )),
        "settings-section",
    )
}

fn section_row(
    label: &str,
    visible_name: &str,
    visible: bool,
    position_name: &str,
    position: i64,
    limit_name: &str,
    limit: i64,
) -> String {
    format!(
        r#"<div class="settings-row settings-order-item" data-settings-order-item draggable="true">
<input type="hidden" name="{position_name}" value="{position}">
<div class="settings-row-label-group">
<button type="button" class="settings-drag-handle" aria-label="Reorder home sections">Drag</button>
<span class="settings-row-label">{label}</span>
</div>
<span class="settings-row-field settings-row-check"><input type="checkbox" name="{visible_name}" {}></span>
<span class="settings-row-field"><input type="number" name="{limit_name}" min="1" max="24" value="{limit}"></span>
</div>"#,
        if visible { "checked" } else { "" },
    )
}

fn surface_panel(body: &str) -> String {
    format!(r#"<div class="surface settings-panel">{body}</div>"#)
}
