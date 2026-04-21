//! Dedicated admin settings template

use super::dashboard_favorites::settings_favorite_order_section;
use super::index::{admin_create_actions, list_rail};
use super::layout::{base, shell_page};
use super::sections::page_header;
use super::settings_core::{
    live_ice_servers_row, media_quality_row, new_resources_private_row, nostr_names_row,
    nostr_relays_row, public_base_url_row, search_page_size_row, session_timeout_row,
    site_description_row, site_name_row,
};
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
        "<form class=\"settings-form settings-stack\" method=\"POST\" action=\"/admin/settings\">{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}</form>",
        site_name_row(settings),
        site_description_row(settings),
        public_base_url_row(settings),
        home_hero_section(settings),
        home_sections_section(settings),
        settings_favorite_order_section(favorites),
        session_timeout_row(settings),
        search_page_size_row(settings),
        media_quality_row(settings),
        live_ice_servers_row(settings),
        nostr_names_row(settings),
        nostr_relays_row(settings),
        site_icon_section(settings),
        new_resources_private_row(settings),
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
