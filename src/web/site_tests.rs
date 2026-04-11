use super::db::AppSettings;
use super::site::SiteContext;

#[test]
fn page_meta_uses_safe_noindex_without_public_origin() {
    let meta = SiteContext::from_settings(&settings("")).page_meta("Home", "", true, Some("/"));
    assert!(meta.head_tags().contains("noindex,nofollow"));
    assert!(!meta.head_tags().contains("rel=\"canonical\""));
}

#[test]
fn page_meta_uses_page_then_site_titles() {
    let meta = SiteContext::from_settings(&settings("https://example.com")).page_meta(
        "Home",
        "",
        true,
        Some("/"),
    );
    assert_eq!(meta.full_title(), "Home | Launchpad");
    assert!(meta.head_tags().contains("https://example.com/"));
}

#[test]
fn invalid_persisted_public_origin_falls_back_to_safe_mode() {
    assert_eq!(
        SiteContext::from_settings(&settings("https://example.com/path")).public_base_url,
        None
    );
}

#[test]
fn social_cards_require_canonical_url() {
    let meta = SiteContext::from_settings(&settings(""))
        .page_meta("Home", "", true, Some("/"))
        .with_social_card(Some("https://example.com/demo.webp".to_string()));
    assert!(!meta.head_tags().contains("og:title"));
}

#[test]
fn social_cards_emit_large_image_when_present() {
    let tags = SiteContext::from_settings(&settings("https://example.com"))
        .page_meta("Orbital Chart", "Preview", true, Some("/orbital-chart"))
        .with_social_card(Some(
            "https://example.com/orbital-chart/file?variant=display".to_string(),
        ))
        .head_tags();
    assert!(tags.contains("og:image"));
    assert!(tags.contains("summary_large_image"));
}

fn settings(public_base_url: &str) -> AppSettings {
    AppSettings {
        site_name: "Launchpad".to_string(),
        site_description: "Search-friendly notes.".to_string(),
        public_base_url: public_base_url.to_string(),
        ..AppSettings::default()
    }
}
