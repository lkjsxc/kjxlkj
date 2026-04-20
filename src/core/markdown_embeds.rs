use super::markdown_links::{escape_attr, html_text, local_url_card};
use url::Url;

pub fn render_url_embed(value: &str) -> Option<String> {
    if value.starts_with('/') && !value.starts_with("//") {
        return local_url_card(value, value);
    }
    let url = Url::parse(value).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    let host = url
        .host_str()?
        .trim_start_matches("www.")
        .to_ascii_lowercase();
    if is_image_url(url.path()) {
        return Some(image_embed(value, &host));
    }
    match host.as_str() {
        "youtube.com" | "m.youtube.com" | "youtube-nocookie.com" | "youtu.be" => youtube(&url),
        "vimeo.com" | "player.vimeo.com" => vimeo(&url),
        "soundcloud.com" => Some(frame_card("SoundCloud", value, &soundcloud_src(value))),
        "github.com" => Some(static_card("GitHub", value, &github_title(&url))),
        "x.com" | "twitter.com" => Some(static_card("X", value, &social_title(&url))),
        "pixiv.net" => Some(static_card("Pixiv", value, &pixiv_title(&url))),
        "bsky.app" => Some(static_card("Bluesky", value, &bluesky_title(&url))),
        "npmjs.com" => Some(static_card(
            "npm",
            value,
            &last_segment_title(&url, "Package"),
        )),
        "crates.io" => Some(static_card(
            "crates.io",
            value,
            &last_segment_title(&url, "Crate"),
        )),
        "docs.rs" => Some(static_card(
            "docs.rs",
            value,
            &last_segment_title(&url, "Docs"),
        )),
        _ if is_mastodon_like(&url) => Some(static_card("Mastodon", value, &social_title(&url))),
        _ => Some(static_card(
            "External",
            value,
            url.host_str().unwrap_or(value),
        )),
    }
}

fn youtube(url: &Url) -> Option<String> {
    let host = url.host_str()?.trim_start_matches("www.");
    let id = if host == "youtu.be" {
        segment(url, 0)
    } else if url.path() == "/watch" {
        url.query_pairs()
            .find(|(key, _)| key == "v")
            .map(|(_, value)| value.to_string())
    } else if matches!(segment(url, 0).as_deref(), Some("shorts" | "embed")) {
        segment(url, 1)
    } else {
        None
    };
    id.map(|value| {
        frame_card(
            "YouTube",
            url.as_str(),
            &format!(
                "https://www.youtube-nocookie.com/embed/{}",
                escape_attr(&value)
            ),
        )
    })
    .or_else(|| Some(static_card("YouTube", url.as_str(), "YouTube")))
}

fn vimeo(url: &Url) -> Option<String> {
    segment(url, 0)
        .filter(|value| value.chars().all(|ch| ch.is_ascii_digit()))
        .map(|id| {
            frame_card(
                "Vimeo",
                url.as_str(),
                &format!("https://player.vimeo.com/video/{id}"),
            )
        })
        .or_else(|| Some(static_card("Vimeo", url.as_str(), "Vimeo")))
}

fn static_card(provider: &str, href: &str, title: &str) -> String {
    format!(
        r#"<div class="external-embed external-embed-card"><a href="{}" target="_blank" rel="noopener noreferrer"><span class="external-embed-provider">{}</span><strong>{}</strong><small>{}</small></a></div>"#,
        escape_attr(href),
        html_text(provider),
        html_text(title),
        html_text(href),
    )
}

fn frame_card(provider: &str, href: &str, src: &str) -> String {
    format!(
        r#"<div class="external-embed external-embed-frame"><iframe title="{} embed" src="{}" loading="lazy" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></div>"#,
        html_text(provider),
        escape_attr(src),
        escape_attr(href),
        html_text(provider),
    )
}

fn image_embed(href: &str, host: &str) -> String {
    format!(
        r#"<figure class="external-embed external-embed-image"><img src="{}" alt="" loading="lazy"><figcaption>{}</figcaption></figure>"#,
        escape_attr(href),
        html_text(host),
    )
}

fn soundcloud_src(href: &str) -> String {
    let encoded = url::form_urlencoded::byte_serialize(href.as_bytes()).collect::<String>();
    format!("https://w.soundcloud.com/player/?url={encoded}")
}

fn github_title(url: &Url) -> String {
    let parts = segments(url);
    match parts.as_slice() {
        [owner, repo, "issues", number, ..] => format!("{owner}/{repo} issue #{number}"),
        [owner, repo, "pull", number, ..] => format!("{owner}/{repo} pull #{number}"),
        [owner, repo, ..] => format!("{owner}/{repo}"),
        _ => "GitHub".to_string(),
    }
}

fn social_title(url: &Url) -> String {
    segment(url, 0).map_or_else(|| "Profile".to_string(), |user| format!("@{user}"))
}

fn pixiv_title(url: &Url) -> String {
    let parts = segments(url);
    if parts.first().is_some_and(|value| *value == "artworks") {
        return format!("Artwork {}", parts.get(1).unwrap_or(&""));
    }
    last_segment_title(url, "Pixiv")
}

fn bluesky_title(url: &Url) -> String {
    let parts = segments(url);
    parts
        .get(1)
        .map_or_else(|| "Bluesky".to_string(), |user| format!("@{user}"))
}

fn last_segment_title(url: &Url, fallback: &str) -> String {
    segments(url)
        .last()
        .map_or_else(|| fallback.to_string(), |value| value.replace('-', " "))
}

fn is_mastodon_like(url: &Url) -> bool {
    segments(url)
        .iter()
        .any(|part| part.starts_with('@') || part.chars().all(|ch| ch.is_ascii_digit()))
}

fn is_image_url(path: &str) -> bool {
    matches!(
        path.rsplit('.')
            .next()
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "avif")
    )
}

fn segment(url: &Url, index: usize) -> Option<String> {
    url.path_segments()?
        .nth(index)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn segments(url: &Url) -> Vec<&str> {
    url.path_segments()
        .map(|items| items.filter(|item| !item.is_empty()).collect())
        .unwrap_or_default()
}
