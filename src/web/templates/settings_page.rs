//! Dedicated admin settings template

use super::dashboard_favorites::settings_favorite_order_section;
use super::index::{admin_create_actions, list_rail};
use super::layout::{base, html_escape, shell_page};
use super::sections::page_header;
use super::settings_home::{home_hero_section, home_sections_section};
use super::settings_icon::site_icon_section;
use super::settings_panel::settings_row;
use super::settings_security::security_section;
use super::IndexItem;
use crate::web::db::AppSettings;
use crate::web::site::SiteContext;

const ACTIONS_JS: &str = include_str!("resource_actions.js");
const FAVORITE_ORDER_JS: &str = include_str!("favorite_order.js");
const SETTINGS_ICON_JS: &str = include_str!("settings_icon.js");
const SETTINGS_ORDER_JS: &str = include_str!("settings_order.js");
const SETTINGS_SEARCH_JS: &str = include_str!("settings_search.js");

pub fn settings_page(
    settings: &AppSettings,
    favorites: &[IndexItem],
    site: &SiteContext,
) -> String {
    let admin_actions = admin_create_actions();
    let save_row = settings_row(
        "Save",
        r#"<div class="settings-submit-row">
<button type="submit" class="btn btn-primary">Save settings</button>
<a href="/admin" class="btn">Back to dashboard</a>
</div>"#,
        "settings-save-row",
    );
    let settings_form = format!(
        "<form class=\"settings-form settings-stack\" method=\"POST\" action=\"/admin/settings\">{}{}{}{}{}{}{}{}{}{}</form>",
        site_identity_section(settings),
        home_hero_section(settings),
        home_sections_section(settings),
        settings_favorite_order_section(favorites),
        sessions_section(settings),
        search_options_section(settings),
        media_section(settings),
        site_icon_section(settings),
        new_resources_section(settings),
        save_row,
    );
    let search_root = format!(
        r#"<div class="settings-stack" data-settings-search-root>{settings_form}{}</div>"#,
        security_section()
    );
    let content = format!(
        "{}{}{}",
        page_header("Settings", None, "settings-head"),
        settings_search_section(),
        search_root,
    );
    base(
        &site.page_meta(
            "Settings",
            format!("Admin settings for {}.", site.site_name),
            false,
            None,
        ),
        &shell_page(
            "Admin",
            &list_rail(
                "settings",
                &admin_actions,
                r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#,
                true,
            ),
            &content,
            "settings-page",
            &site.site_name,
        ),
        "",
        &format!(
            r#"<script>{ACTIONS_JS}</script><script>{FAVORITE_ORDER_JS}</script><script>{SETTINGS_ORDER_JS}</script><script>{SETTINGS_ICON_JS}</script><script>{SETTINGS_SEARCH_JS}</script>"#
        ),
    )
}

fn site_identity_section(settings: &AppSettings) -> String {
    settings_row(
        "Site identity",
        &format!(
            r#"<div class="settings-section-grid">
<label class="form-group" data-settings-item><span>Site name</span><input type="text" name="site_name" maxlength="80" value="{}"></label>
<label class="form-group" data-settings-item><span>Public base URL</span><input type="url" name="public_base_url" maxlength="255" placeholder="Leave blank to disable indexing" value="{}"></label>
<label class="form-group settings-wide" data-settings-item><span>Site description</span><textarea name="site_description" rows="4" maxlength="200">{}</textarea></label>
<p class="page-summary settings-wide" data-settings-item>Leave Public base URL blank until the deployment has the final public origin for canonical URLs, robots, and the sitemap.</p>
</div>"#,
            html_escape(&settings.site_name),
            html_escape(&settings.public_base_url),
            html_escape(&settings.site_description),
        ),
        "settings-identity-row",
    )
}

fn settings_search_section() -> String {
    r#"<div class="settings-search-row">
<label class="form-group settings-search-card">
<span>Search settings</span>
<input type="search" placeholder="Search labels, rows, and helper text" data-settings-search-input>
</label>
<p class="surface-empty settings-search-empty" data-settings-search-empty hidden>No matching settings.</p>
</div>"#
        .to_string()
}

fn search_options_section(settings: &AppSettings) -> String {
    settings_row(
        "Search",
        &format!(
            r#"<div class="settings-section-grid">
<label class="form-group" data-settings-item><span>Search page size</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>
</div>"#,
            settings.search_results_per_page,
        ),
        "settings-search-options-row",
    )
}

fn media_section(settings: &AppSettings) -> String {
    settings_row(
        "Media",
        &format!(
            r#"<div class="settings-section-grid">
<label class="form-group" data-settings-item><span>Media WebP quality</span><input type="number" name="media_webp_quality" min="1" max="100" value="{}"></label>
<p class="page-summary settings-wide" data-settings-item>Applies to future uploads only. Existing media keeps its current original file and stored derivatives.</p>
</div>"#,
            settings.media_webp_quality,
        ),
        "settings-media-row",
    )
}

fn new_resources_section(settings: &AppSettings) -> String {
    settings_row(
        "New resources",
        &format!(
            r#"<div class="settings-section-grid">
<label class="check-row check-row-field settings-wide" data-settings-item><input type="checkbox" name="default_new_resource_is_private" {}><span>New resources start private</span></label>
<p class="page-summary settings-wide" data-settings-item>Applies to future notes and media only.</p>
</div>"#,
            if settings.default_new_resource_is_private {
                "checked"
            } else {
                ""
            },
        ),
        "settings-new-resources-row",
    )
}

fn sessions_section(settings: &AppSettings) -> String {
    settings_row(
        "Sessions",
        &format!(
            r#"<div class="settings-section-grid">
<label class="form-group" data-settings-item><span>Session timeout (minutes)</span><input type="number" name="session_timeout_minutes" min="5" max="10080" value="{}"></label>
<p class="page-summary settings-wide" data-settings-item>Applies to future logins only. Active sessions keep their current expiry.</p>
</div>"#,
            settings.session_timeout_minutes,
        ),
        "settings-sessions-row",
    )
}
