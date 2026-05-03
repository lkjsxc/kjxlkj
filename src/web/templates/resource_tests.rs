use super::{resource::resource_page, ResourceAnalytics, ResourceChrome};
use crate::core::render_markdown;
use crate::web::db::{MediaFamily, Resource, ResourceKind};
use crate::web::site::SiteContext;
use chrono::Utc;

fn sample_resource() -> Resource {
    Resource {
        id: "abcdefghijklmnopqrstuvwx26".to_string(),
        space_slug: "alice".to_string(),
        kind: ResourceKind::Note,
        alias: Some("demo-note".to_string()),
        title: "Demo".to_string(),
        summary: "Body".to_string(),
        body: "# Demo\n\nBody".to_string(),
        media_family: None,
        file_key: None,
        content_type: None,
        byte_size: None,
        sha256_hex: None,
        original_filename: None,
        width: None,
        height: None,
        duration_ms: None,
        media_variants: None,
        owner_note_id: None,
        is_favorite: true,
        favorite_position: Some(1),
        is_private: false,
        view_count_total: 3,
        last_viewed_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn sample_chrome() -> ResourceChrome {
    ResourceChrome {
        id: "abcdefghijklmnopqrstuvwx26".to_string(),
        kind: ResourceKind::Note,
        alias: Some("demo-note".to_string()),
        title: "Demo".to_string(),
        summary: "Body".to_string(),
        current_href: "/demo-note".to_string(),
        created_at: "2026-03-26 08:34 UTC".to_string(),
        updated_at: "2026-03-26 08:35 UTC".to_string(),
        is_favorite: true,
        visibility: "Public",
        previous: None,
        next: None,
        history_href: "/demo-note/history".to_string(),
    }
}
#[rustfmt::skip]
fn sample_site() -> SiteContext { SiteContext { site_name: "Launchpad".to_string(), site_description: "Search-friendly notes.".to_string(), public_base_url: Some("https://example.com".to_string()) } }
fn sample_media_resource() -> Resource {
    Resource {
        id: "bcdefghijklmnopqrstuvwxy27".to_string(),
        space_slug: "alice".to_string(),
        kind: ResourceKind::Media,
        alias: Some("demo-image".to_string()),
        title: "Demo file".to_string(),
        summary: "File body".to_string(),
        body: "# Demo file\n\nBody".to_string(),
        media_family: Some(MediaFamily::File),
        file_key: Some("media/demo/original.heic".to_string()),
        content_type: Some("image/heic".to_string()),
        byte_size: Some(1234),
        sha256_hex: Some("abc".to_string()),
        original_filename: Some("demo.heic".to_string()),
        width: None,
        height: None,
        duration_ms: None,
        media_variants: None,
        owner_note_id: None,
        is_favorite: false,
        favorite_position: None,
        is_private: false,
        view_count_total: 0,
        last_viewed_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
#[rustfmt::skip]
fn guest_resource_page_hides_editor() {
    let html = resource_page(
        &sample_resource(),
        &sample_chrome(),
        None,
        &render_markdown(&sample_resource().body),
        false,
        &sample_site(),
    );
    assert!(html.contains("shell-rail"));
    assert!(!html.contains("id=\"editor-body\""));
    assert_eq!(html.match_indices("<h1>").count(), 1);
    assert!(html.contains("<title>Demo | Launchpad</title>"));
    assert!(html.contains(r#"class="resource-nav-strip resource-nav-strip-dual""#));
    assert!(html.contains("rel=\"canonical\" href=\"https://example.com/demo-note\"") && html.contains("content=\"index,follow\""));
}

#[test]
fn admin_resource_page_renders_alias_controls_without_markdown_body_label() {
    let html = resource_page(
        &sample_resource(),
        &sample_chrome(),
        Some(&ResourceAnalytics {
            total: 12,
            views_1d: 2,
            views_7d: 4,
            views_30d: 7,
            views_90d: 9,
            last_viewed_at: Some("2026-03-26 08:35 UTC".to_string()),
        }),
        &render_markdown(&sample_resource().body),
        true,
        &sample_site(),
    );
    assert!(html.contains("favorite-toggle"));
    assert!(html.contains("alias-input"));
    assert!(html.contains("id=\"editor-body\""));
    assert!(html.contains("preview-toggle"));
    assert!(html.contains("upload-media-trigger"));
    assert!(html.contains("resource-nav-strip"));
    assert!(html.contains("editor-field-card"));
    assert!(html.contains("Views total"));
    assert!(html.contains("Views 1d"));
    assert!(html.contains("2026-03-26 08:35 UTC"));
    assert!(html.contains("Open GitHub"));
    assert!(html.contains("status-pill"));
    assert!(!html.contains(r#"class="summary-card current-resource-card"#));
    assert!(!html.contains("<strong>Alias</strong>"));
    assert!(!html.contains("Markdown body"));
    assert!(!html.contains("<div class=\"page-title-stack\"><h1"));
    assert!(!html.contains("Open saved snapshots."));
    assert!(!html.contains(r#"<p class="page-summary" data-live-summary>"#));
    assert!(!html.contains("toastui"));
    assert!(html.contains("content=\"noindex,nofollow\""));
    assert!(!html.contains("rel=\"canonical\""));
}

#[test]
fn guest_media_page_exposes_original_download_and_display_route() {
    let html = resource_page(
        &sample_media_resource(),
        &ResourceChrome {
            current_href: "/demo-image".to_string(),
            history_href: "/demo-image/history".to_string(),
            kind: ResourceKind::Media,
            ..sample_chrome()
        },
        None,
        &render_markdown(&sample_media_resource().body),
        false,
        &sample_site(),
    );
    assert!(html.contains("Download original"));
    assert!(html.contains("href=\"/alice/demo-image/file\""));
    assert!(html.contains("download=\"demo.heic\""));
    assert!(html.contains("Open raw file"));
    assert!(html.contains(r#"class="resource-nav-strip resource-nav-strip-dual""#));
    assert!(!html.contains(r#"class="summary-card current-resource-card"#));
    assert!(!html.contains("data-history-link"));
    assert!(!html.contains("variant=display"));
}

#[test]
fn admin_media_page_uses_live_resource_shell() {
    let html = resource_page(
        &sample_media_resource(),
        &ResourceChrome {
            current_href: "/demo-image".to_string(),
            history_href: "/demo-image/history".to_string(),
            kind: ResourceKind::Media,
            ..sample_chrome()
        },
        Some(&ResourceAnalytics {
            total: 5,
            views_1d: 1,
            views_7d: 2,
            views_30d: 3,
            views_90d: 4,
            last_viewed_at: Some("2026-03-26 08:35 UTC".to_string()),
        }),
        &render_markdown(&sample_media_resource().body),
        true,
        &sample_site(),
    );
    assert!(html.contains("resource-nav-strip"));
    assert!(html.contains("File URL"));
    assert!(html.contains("File metadata"));
    assert!(html.contains("Delete media"));
    assert!(html.contains("Download original"));
    assert!(html.contains("id=\"editor-body\""));
    assert!(!html.contains(">Upload media<"));
    assert!(!html.contains(r#"class="summary-card current-resource-card"#));
}
