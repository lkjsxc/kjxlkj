use super::render_markdown;

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
