use super::attr;
use crate::core::markdown_links::html_text;
use url::Url;

pub(super) fn render(url: &Url, host: &str) -> Option<String> {
    let provider = match host {
        "x.com" | "twitter.com" => "X",
        "instagram.com" => "Instagram",
        "bsky.app" => "Bluesky",
        _ => return None,
    };
    Some(format!(
        r#"<div class="external-embed external-embed-social" data-embed-provider="{}" data-embed-url="{}"><a href="{}" target="_blank" rel="noopener noreferrer"><span class="external-embed-provider">{}</span><strong>{}</strong><small>{}</small></a></div>"#,
        html_text(&provider.to_ascii_lowercase()),
        attr(url.as_str()),
        attr(url.as_str()),
        html_text(provider),
        html_text(&social_title(url)),
        html_text(url.as_str()),
    ))
}

fn social_title(url: &Url) -> String {
    let parts = url
        .path_segments()
        .map(|items| items.filter(|item| !item.is_empty()).collect::<Vec<_>>())
        .unwrap_or_default();
    match parts.as_slice() {
        [user, "status", id, ..] | [user, "statuses", id, ..] => format!("@{user} post {id}"),
        ["profile", user, "post", id, ..] => format!("@{user} post {id}"),
        [user, ..] => format!("@{user}"),
        _ => "Social post".to_string(),
    }
}
