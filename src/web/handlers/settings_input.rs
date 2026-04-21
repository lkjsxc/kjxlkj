//! Settings form parsing and validation

use crate::core::live_settings::normalize_ice_servers_json;
use crate::core::nostr::{normalize_names_json, normalize_relays_json};
use crate::error::AppError;
use crate::web::db::AppSettings;
use crate::web::site::normalize_public_base_url;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct SettingsForm {
    pub site_name: String,
    pub site_description: String,
    pub public_base_url: String,
    pub nostr_names_json: String,
    pub nostr_relays_json: String,
    pub live_ice_servers_json: String,
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub home_popular_limit: i64,
    pub home_intro_markdown: String,
    pub home_recent_visible: Option<String>,
    pub home_favorite_visible: Option<String>,
    pub home_popular_visible: Option<String>,
    pub home_recent_position: i64,
    pub home_favorite_position: i64,
    pub home_popular_position: i64,
    pub search_results_per_page: i64,
    pub session_timeout_minutes: i64,
    pub media_webp_quality: i64,
    pub default_new_resource_is_private: Option<String>,
}

pub fn validate_settings_form(
    form: &SettingsForm,
    current: &AppSettings,
) -> Result<AppSettings, AppError> {
    let site_name = form.site_name.trim();
    let site_description = form.site_description.trim();
    let public_base_url = validate_public_base_url(&form.public_base_url)?;
    let nostr_names = normalize_names_json(&form.nostr_names_json).map_err(|e| invalid(&e))?;
    let nostr_relays = normalize_relays_json(&form.nostr_relays_json).map_err(|e| invalid(&e))?;
    let live_ice_servers =
        normalize_ice_servers_json(&form.live_ice_servers_json).map_err(|e| invalid(&e))?;
    if site_name.is_empty() || site_name.len() > 80 {
        return Err(invalid("site name must be between 1 and 80 characters"));
    }
    if site_description.is_empty() || site_description.len() > 200 {
        return Err(invalid(
            "site description must be between 1 and 200 characters",
        ));
    }
    if !counts_are_valid(form) {
        return Err(invalid("section counts must be between 1 and 24"));
    }
    if !(5..=100).contains(&form.search_results_per_page) {
        return Err(invalid("search page size must be between 5 and 100"));
    }
    if !(5..=10080).contains(&form.session_timeout_minutes) {
        return Err(invalid(
            "session timeout must be between 5 and 10080 minutes",
        ));
    }
    if !(1..=100).contains(&form.media_webp_quality) {
        return Err(invalid("WebP quality must be between 1 and 100"));
    }
    if !positions_are_valid(form) {
        return Err(invalid("section order must use 1, 2, and 3 exactly once"));
    }
    Ok(AppSettings {
        site_name: site_name.to_string(),
        site_description: site_description.to_string(),
        public_base_url,
        nostr_names,
        nostr_relays,
        live_ice_servers,
        home_recent_limit: form.home_recent_limit,
        home_favorite_limit: form.home_favorite_limit,
        home_popular_limit: form.home_popular_limit,
        home_intro_markdown: form.home_intro_markdown.trim().to_string(),
        home_recent_visible: form.home_recent_visible.is_some(),
        home_favorite_visible: form.home_favorite_visible.is_some(),
        home_popular_visible: form.home_popular_visible.is_some(),
        home_recent_position: form.home_recent_position,
        home_favorite_position: form.home_favorite_position,
        home_popular_position: form.home_popular_position,
        search_results_per_page: form.search_results_per_page,
        session_timeout_minutes: form.session_timeout_minutes,
        media_webp_quality: form.media_webp_quality,
        default_new_resource_is_private: form.default_new_resource_is_private.is_some(),
        site_icon_key: current.site_icon_key.clone(),
        site_icon_content_type: current.site_icon_content_type.clone(),
    })
}

fn validate_public_base_url(value: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    normalize_public_base_url(trimmed)
        .filter(|normalized| normalized.len() <= 255)
        .ok_or_else(|| invalid("public base URL must be a bare http or https origin"))
}

fn counts_are_valid(form: &SettingsForm) -> bool {
    [
        form.home_recent_limit,
        form.home_favorite_limit,
        form.home_popular_limit,
    ]
    .into_iter()
    .all(|value| (1..=24).contains(&value))
}

fn positions_are_valid(form: &SettingsForm) -> bool {
    let positions = [
        form.home_popular_position,
        form.home_recent_position,
        form.home_favorite_position,
    ];
    positions.iter().all(|value| (1..=3).contains(value))
        && positions.into_iter().collect::<HashSet<_>>().len() == 3
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}

#[cfg(test)]
mod tests {
    use super::{validate_settings_form, SettingsForm};
    use crate::web::db::AppSettings;

    fn sample_form() -> SettingsForm {
        SettingsForm {
            site_name: "Launchpad".to_string(),
            site_description: "Search-friendly notes.".to_string(),
            public_base_url: "https://example.com".to_string(),
            nostr_names_json: "{}".to_string(),
            nostr_relays_json: "[]".to_string(),
            live_ice_servers_json: "[]".to_string(),
            home_recent_limit: 5,
            home_favorite_limit: 5,
            home_popular_limit: 5,
            home_intro_markdown: "# Home".to_string(),
            home_recent_visible: Some("on".to_string()),
            home_favorite_visible: Some("on".to_string()),
            home_popular_visible: Some("on".to_string()),
            home_recent_position: 1,
            home_favorite_position: 2,
            home_popular_position: 3,
            search_results_per_page: 20,
            session_timeout_minutes: 1440,
            media_webp_quality: 82,
            default_new_resource_is_private: None,
        }
    }

    #[test]
    fn validate_accepts_blank_public_origin() {
        let mut form = sample_form();
        form.public_base_url = "   ".to_string();
        assert_eq!(
            validate_settings_form(&form, &AppSettings::default())
                .unwrap()
                .public_base_url,
            ""
        );
    }

    #[test]
    fn validate_rejects_invalid_public_origin() {
        let mut form = sample_form();
        form.public_base_url = "https://example.com/path".to_string();
        assert!(validate_settings_form(&form, &AppSettings::default()).is_err());
    }

    #[test]
    fn validate_rejects_blank_site_name() {
        let mut form = sample_form();
        form.site_name = "   ".to_string();
        assert!(validate_settings_form(&form, &AppSettings::default()).is_err());
    }
}
