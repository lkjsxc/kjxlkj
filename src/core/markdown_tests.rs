use super::render_markdown;

#[test]
fn render_markdown_keeps_safe_media_embeds() {
    let html = render_markdown("![](/demo/file)\n\n<video controls src=\"/clip/file\"></video>");
    assert!(html.contains("<img"));
    assert!(html.contains("src=\"/demo/file\""));
    assert!(html.contains("<video"));
    assert!(html.contains("src=\"/clip/file\""));
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
