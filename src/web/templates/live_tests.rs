use super::live::live_page;
use crate::web::db::AppSettings;
use crate::web::site::SiteContext;
use serde_json::Value;

fn site() -> SiteContext {
    SiteContext {
        site_name: "Launchpad".to_string(),
        site_description: "Search-friendly notes.".to_string(),
        public_base_url: None,
    }
}

#[test]
fn admin_live_page_renders_capture_controls() {
    let html = live_page(&site(), true, "/login", &AppSettings::default());
    assert!(html.contains("data-live-source"));
    assert!(html.contains("data-live-camera"));
    assert!(html.contains("data-live-height"));
    assert!(html.contains("value=\"1080\" selected"));
    assert!(html.contains("data-live-fps"));
    assert!(html.contains("value=\"60\" selected"));
    assert!(html.contains("data-live-mic"));
    assert!(html.contains("data-live-viewer-count"));
    assert!(html.contains("<video class=\"live-video\" autoplay playsinline controls muted"));
}

#[test]
fn guest_live_page_uses_native_controls_without_admin_ui() {
    let html = live_page(
        &site(),
        false,
        "/login?return_to=%2Flive",
        &AppSettings::default(),
    );
    assert!(html.contains("<video class=\"live-video\" autoplay playsinline controls"));
    assert!(!html.contains("<select data-live-source"));
    assert!(!html.contains("class=\"status-pill\" data-live-viewer-count"));
    assert!(html.contains("Admin sign in"));
}

#[test]
fn live_config_is_parseable_script_json_without_ice_servers() {
    let settings = AppSettings {
        live_default_source: "camera".to_string(),
        ..AppSettings::default()
    };
    let html = live_page(&site(), false, "/login", &settings);
    let config = extract_live_config(&html);
    assert!(!config.contains("&quot;"));
    let value: Value = serde_json::from_str(config).unwrap();
    assert_eq!(value["source"], "camera");
    assert_eq!(value["height"], 1080);
    assert_eq!(value["fps"], 60);
    assert!(value.get("iceServers").is_none());
}

fn extract_live_config(html: &str) -> &str {
    let start_tag = r#"<script type="application/json" id="live-config">"#;
    let start = html.find(start_tag).unwrap() + start_tag.len();
    let end = html[start..].find("</script>").unwrap() + start;
    &html[start..end]
}
