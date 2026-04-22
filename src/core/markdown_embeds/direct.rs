use super::{attr, frame_card};
use crate::core::markdown_links::html_text;
use url::Url;

pub(super) fn render(url: &Url, host: &str) -> Option<String> {
    let href = url.as_str();
    if has_ext(
        url.path(),
        &["png", "jpg", "jpeg", "gif", "webp", "svg", "avif"],
    ) {
        return Some(format!(
            r#"<figure class="external-embed external-embed-image"><img src="{}" alt="" loading="lazy"><figcaption>{}</figcaption></figure>"#,
            attr(href),
            html_text(host),
        ));
    }
    if has_ext(url.path(), &["mp4", "webm", "mov", "m4v", "ogv"]) {
        return Some(format!(
            r#"<div class="external-embed external-embed-native"><video controls preload="metadata" src="{}"></video><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></div>"#,
            attr(href),
            attr(href),
            html_text(host),
        ));
    }
    if has_ext(url.path(), &["mp3", "m4a", "ogg", "opus", "wav", "flac"]) {
        return Some(format!(
            r#"<div class="external-embed external-embed-native"><audio controls preload="metadata" src="{}"></audio><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></div>"#,
            attr(href),
            attr(href),
            html_text(host),
        ));
    }
    has_ext(url.path(), &["pdf"]).then(|| frame_card("Document", href, href))
}

fn has_ext(path: &str, extensions: &[&str]) -> bool {
    path.rsplit('.')
        .next()
        .map(str::to_ascii_lowercase)
        .is_some_and(|ext| extensions.contains(&ext.as_str()))
}
