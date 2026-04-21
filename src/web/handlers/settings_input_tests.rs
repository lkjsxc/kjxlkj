use super::settings_input::{validate_settings_form, SettingsForm};
use crate::web::db::AppSettings;

fn sample_form() -> SettingsForm {
    SettingsForm {
        site_name: "Launchpad".to_string(),
        site_description: "Search-friendly notes.".to_string(),
        public_base_url: "https://example.com".to_string(),
        nostr_names_json: "{}".to_string(),
        nostr_relays_json: "[]".to_string(),
        live_ice_servers_json: "[]".to_string(),
        live_default_source: "screen".to_string(),
        live_default_height: 1080,
        live_default_fps: 60,
        live_default_microphone_enabled: None,
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

#[test]
fn validate_accepts_live_defaults() {
    let mut form = sample_form();
    form.live_default_source = "camera".to_string();
    form.live_default_height = 2160;
    form.live_default_fps = 120;
    form.live_default_microphone_enabled = Some("on".to_string());
    let settings = validate_settings_form(&form, &AppSettings::default()).unwrap();
    assert_eq!(settings.live_default_source, "camera");
    assert_eq!(settings.live_default_height, 2160);
    assert_eq!(settings.live_default_fps, 120);
    assert!(settings.live_default_microphone_enabled);
}

#[test]
fn validate_rejects_invalid_live_defaults() {
    let mut form = sample_form();
    form.live_default_source = "window".to_string();
    assert!(validate_settings_form(&form, &AppSettings::default()).is_err());
    form = sample_form();
    form.live_default_height = 999;
    assert!(validate_settings_form(&form, &AppSettings::default()).is_err());
    form = sample_form();
    form.live_default_fps = 24;
    assert!(validate_settings_form(&form, &AppSettings::default()).is_err());
}
