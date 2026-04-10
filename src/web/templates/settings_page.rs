//! Dedicated admin settings template

use super::index::{admin_create_actions, list_rail};
use super::layout::{base, html_escape, shell_page};
use super::sections::{page_header, section};
use super::settings_home::{home_hero_section, home_sections_section};
use super::settings_panel::surface_panel;
use super::settings_security::security_section;
use crate::web::db::AppSettings;
use crate::web::site::SiteContext;

const ACTIONS_JS: &str = include_str!("resource_actions.js");
const SETTINGS_ORDER_JS: &str = include_str!("settings_order.js");

pub fn settings_page(settings: &AppSettings, site: &SiteContext) -> String {
    let admin_actions = admin_create_actions();
    let settings_form = format!(
        "<form class=\"settings-form settings-stack\" method=\"POST\" action=\"/admin/settings\">{}{}{}{}{}<div class=\"settings-submit-row\"><button type=\"submit\" class=\"btn btn-primary\">Save settings</button><a href=\"/admin\" class=\"btn\">Back to dashboard</a></div></form>",
        site_identity_section(settings),
        home_hero_section(settings),
        home_sections_section(settings),
        sessions_section(settings),
        defaults_section(settings),
    );
    let content = format!(
        "{}{}{}",
        page_header("Settings", None, "settings-head"),
        settings_form,
        security_section(settings)
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
        &format!(r#"<script>{ACTIONS_JS}</script><script>{SETTINGS_ORDER_JS}</script>"#),
    )
}

fn site_identity_section(settings: &AppSettings) -> String {
    section(
        "Site identity",
        &surface_panel(&format!(
            r#"<div class="settings-section-grid">
<label class="form-group"><span>Site name</span><input type="text" name="site_name" maxlength="80" value="{}"></label>
<label class="form-group"><span>Public base URL</span><input type="url" name="public_base_url" maxlength="255" placeholder="Leave blank to disable indexing" value="{}"></label>
<label class="form-group settings-wide"><span>Site description</span><textarea name="site_description" rows="4" maxlength="200">{}</textarea></label>
<p class="page-summary settings-wide">Leave Public base URL blank until the deployment has the final public origin for canonical URLs, robots, and the sitemap.</p>
</div>"#,
            html_escape(&settings.site_name),
            html_escape(&settings.public_base_url),
            html_escape(&settings.site_description),
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
<label class="form-group"><span>Media WebP quality</span><input type="number" name="media_webp_quality" min="1" max="100" value="{}"></label>
<label class="check-row check-row-field settings-wide"><input type="checkbox" name="default_new_resource_is_private" {}><span>New resources start private</span></label>
</div>"#,
            settings.search_results_per_page,
            settings.media_webp_quality,
            if settings.default_new_resource_is_private {
                "checked"
            } else {
                ""
            },
        )),
        "settings-section",
    )
}

fn sessions_section(settings: &AppSettings) -> String {
    section(
        "Sessions",
        &surface_panel(&format!(
            r#"<div class="settings-section-grid">
<label class="form-group"><span>Session timeout (minutes)</span><input type="number" name="session_timeout_minutes" min="5" max="10080" value="{}"></label>
<p class="page-summary settings-wide">Applies to future logins only. Active sessions keep their current expiry.</p>
</div>"#,
            settings.session_timeout_minutes,
        )),
        "settings-section",
    )
}
