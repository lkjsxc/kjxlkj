//! Scalar and JSON settings rows

use super::layout::html_escape;
use super::settings_panel::settings_row;
use crate::web::db::AppSettings;

pub(super) fn site_name_row(settings: &AppSettings) -> String {
    settings_row(
        "Site_identity/Site_name",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Site_identity/Site_name</span><input type="text" name="site_name" maxlength="80" value="{}"></label>"#,
            html_escape(&settings.site_name),
        ),
        "settings-site-name-row",
    )
}

pub(super) fn site_description_row(settings: &AppSettings) -> String {
    settings_row(
        "Site_identity/Site_description",
        &format!(
            r#"<label class="form-group settings-wide" data-settings-item><span>Site_identity/Site_description</span><textarea name="site_description" rows="4" maxlength="200">{}</textarea></label>"#,
            html_escape(&settings.site_description),
        ),
        "settings-site-description-row",
    )
}

pub(super) fn public_base_url_row(settings: &AppSettings) -> String {
    settings_row(
        "Site_identity/Public_base_URL",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Site_identity/Public_base_URL</span><input type="url" name="public_base_url" maxlength="255" placeholder="Blank disables indexing" value="{}"></label>
<p class="page-summary" data-settings-item>Leave blank until the deployment has the final public origin for canonical URLs, robots, and the sitemap.</p>"#,
            html_escape(&settings.public_base_url),
        ),
        "settings-public-base-url-row",
    )
}

pub(super) fn search_page_size_row(settings: &AppSettings) -> String {
    settings_row(
        "Search/Results_per_page",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Search/Results_per_page</span><input type="number" name="search_results_per_page" min="5" max="100" value="{}"></label>"#,
            settings.search_results_per_page,
        ),
        "settings-search-options-row",
    )
}

pub(super) fn media_quality_row(settings: &AppSettings) -> String {
    settings_row(
        "Media/WebP_quality",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Media/WebP_quality</span><input type="number" name="media_webp_quality" min="1" max="100" value="{}"></label>
<p class="page-summary" data-settings-item>Applies to future uploads only. Existing media keeps its current original file and stored derivatives.</p>"#,
            settings.media_webp_quality,
        ),
        "settings-media-row",
    )
}

pub(super) fn google_maps_key_row(settings: &AppSettings) -> String {
    settings_row(
        "Embeds/Google_Maps_API_key",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Embeds/Google_Maps_API_key</span><input type="text" name="google_maps_embed_api_key" maxlength="255" value="{}"></label>
<p class="page-summary" data-settings-item>Leave blank to render Google Maps URLs as static cards instead of generated map iframes.</p>"#,
            html_escape(&settings.google_maps_embed_api_key),
        ),
        "settings-embeds-row",
    )
}

pub(super) fn new_resources_private_row(settings: &AppSettings) -> String {
    settings_row(
        "Resources/New_resources_start_private",
        &format!(
            r#"<label class="check-row check-row-field" data-settings-item><input type="checkbox" name="default_new_resource_is_private" {}><span>Resources/New_resources_start_private</span></label>
<p class="page-summary" data-settings-item>Applies to future notes and media only.</p>"#,
            if settings.default_new_resource_is_private {
                "checked"
            } else {
                ""
            },
        ),
        "settings-new-resources-row",
    )
}

pub(super) fn session_timeout_row(settings: &AppSettings) -> String {
    settings_row(
        "Session/Timeout_minutes",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Session/Timeout_minutes</span><input type="number" name="session_timeout_minutes" min="5" max="10080" value="{}"></label>
<p class="page-summary" data-settings-item>Applies to future logins only. Active sessions keep their current expiry.</p>"#,
            settings.session_timeout_minutes,
        ),
        "settings-sessions-row",
    )
}

pub(super) fn nostr_names_row(settings: &AppSettings) -> String {
    json_textarea_row(
        "Nostr/Names_JSON",
        "nostr_names_json",
        &settings.nostr_names,
        "settings-nostr-names-row",
    )
}

pub(super) fn nostr_relays_row(settings: &AppSettings) -> String {
    json_textarea_row(
        "Nostr/Relays_JSON",
        "nostr_relays_json",
        &settings.nostr_relays,
        "settings-nostr-relays-row",
    )
}

pub(super) fn json_textarea_row(
    label: &str,
    name: &str,
    value: &serde_json::Value,
    class_name: &str,
) -> String {
    settings_row(
        label,
        &format!(
            r#"<label class="form-group settings-wide" data-settings-item><span>{}</span><textarea name="{name}" rows="5">{}</textarea></label>"#,
            html_escape(label),
            html_escape(&pretty_json(value)),
        ),
        class_name,
    )
}

fn pretty_json(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}
