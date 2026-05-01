use super::{render_markdown, render_markdown_with_options, EmbedMetadata, MarkdownOptions};
use std::collections::HashMap;

#[test]
fn render_markdown_keeps_safe_media_embeds() {
    let html = render_markdown("![](/demo/file)\n\n<video controls src=\"/clip/file\"></video>");
    assert!(html.contains("<img"));
    assert!(html.contains("src=\"/demo/file?variant=display\""));
    assert!(html.contains("<video"));
    assert!(html.contains("src=\"/clip/file\""));
    assert!(html.contains("poster=\"/clip/file?variant=poster\""));
}

#[test]
fn render_markdown_strips_unsafe_html() {
    let html = render_markdown(
        "<script>alert(1)</script><video onclick=\"evil()\" controls src=\"/clip/file\"></video>",
    );
    assert!(!html.contains("<script"));
    assert!(!html.contains("onclick="));
    assert!(html.contains("<video"));
}

#[test]
fn render_markdown_cards_local_file_links() {
    let html = render_markdown("[/demo/file](/demo/file)\n\n[external](https://example.com/file)");

    assert!(html.contains("class=\"local-url-card\""));
    assert!(html.contains("src=\"/demo/file?variant=card\""));
    assert!(html.contains("https://example.com/file"));
}

#[test]
fn render_markdown_keeps_task_list_checkboxes() {
    let html = render_markdown("- [x] Done\n- [ ] Todo");

    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("checked"));
    assert!(html.contains("disabled"));
}

#[test]
fn render_markdown_cards_local_resource_pages() {
    let html = render_markdown("[Orbit Ledger](/orbit-ledger)");

    assert!(html.contains("local-url-card-page"));
    assert!(html.contains("href=\"/orbit-ledger\""));
    assert!(html.contains(">Orbit Ledger<"));
}

#[test]
fn render_markdown_embeds_standalone_external_urls() {
    let html = render_markdown("Before\n\nhttps://github.com/lkjsxc/kjxlkj/pull/12\n\nAfter");

    assert!(html.contains("external-embed-card"));
    assert!(html.contains("GitHub"));
    assert!(html.contains("lkjsxc/kjxlkj pull #12"));
}

#[test]
fn render_markdown_keeps_non_standalone_urls_plain() {
    let html = render_markdown(
        "Inline https://github.com/lkjsxc/kjxlkj stays text.\n\n    https://x.com/lkjsxc\n\n```txt\nhttps://pixiv.net/artworks/123\n```",
    );

    assert!(!html.contains("external-embed"));
    assert!(html.contains("https://github.com/lkjsxc/kjxlkj"));
    assert!(html.contains("https://pixiv.net/artworks/123"));
}

#[test]
fn render_markdown_generates_safe_provider_frames() {
    let html = render_markdown(
        "https://youtu.be/dQw4w9WgXcQ\n\n<iframe src=\"https://evil.example\"></iframe>",
    );

    assert!(html.contains("external-embed-frame"));
    assert!(html.contains("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ"));
    assert!(!html.contains("evil.example"));
}

#[test]
fn render_markdown_embeds_direct_images_and_local_urls() {
    let html = render_markdown("https://example.com/chart.webp\n\n/demo/file");

    assert!(html.contains("external-embed-image"));
    assert!(html.contains("src=\"https://example.com/chart.webp\""));
    assert!(html.contains("local-url-card"));
    assert!(html.contains("/demo/file?variant=card"));
}

#[test]
fn render_markdown_embeds_broad_provider_frames() {
    let html = render_markdown_with_options(
        "https://open.spotify.com/track/abc123\n\nhttps://www.tiktok.com/@demo/video/1234567890",
        MarkdownOptions::default(),
    );

    assert!(html.contains("https://open.spotify.com/embed/track/abc123"));
    assert!(html.contains("https://www.tiktok.com/player/v1/1234567890"));
}

#[test]
fn render_markdown_embeds_maps_only_with_key() {
    let without_key = render_markdown("https://www.google.com/maps/place/Tokyo");
    let with_key = render_markdown_with_options(
        "https://www.google.com/maps/place/Tokyo",
        MarkdownOptions {
            public_base_url: None,
            google_maps_embed_api_key: Some("maps-key"),
            external_embed_cache: None,
        },
    );

    assert!(without_key.contains("external-embed-card"));
    assert!(!without_key.contains("maps/embed/v1"));
    assert!(with_key.contains("https://www.google.com/maps/embed/v1/search"));
    assert!(with_key.contains("key=maps-key"));
}

#[test]
fn render_markdown_embeds_native_audio_and_social_placeholders() {
    let html =
        render_markdown("https://example.com/interview.mp3\n\nhttps://x.com/user/status/123");

    assert!(html.contains("<audio controls"));
    assert!(html.contains("data-embed-provider=\"x\""));
    assert!(html.contains("data-embed-url=\"https://x.com/user/status/123\""));
}

#[test]
fn render_markdown_uses_cached_external_metadata_cards() {
    let mut cache = HashMap::new();
    cache.insert(
        "https://example.com/article".to_string(),
        EmbedMetadata {
            provider: "example.com".to_string(),
            title: Some("Cached article".to_string()),
            description: Some("Cached summary".to_string()),
            site_name: Some("Example".to_string()),
            author_name: Some("Author".to_string()),
            thumbnail_url: Some("https://example.com/card.jpg".to_string()),
        },
    );
    let html = render_markdown_with_options(
        "https://example.com/article",
        MarkdownOptions {
            public_base_url: None,
            google_maps_embed_api_key: None,
            external_embed_cache: Some(&cache),
        },
    );

    assert!(html.contains("external-embed-bookmark"));
    assert!(html.contains("Cached article"));
    assert!(html.contains("Cached summary"));
    assert!(html.contains("https://example.com/card.jpg"));
}
