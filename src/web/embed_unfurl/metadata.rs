use crate::web::db::ExternalEmbed;
use url::Url;

pub(super) fn parse_html(url: &str, provider: &str, html: &str) -> ExternalEmbed {
    let meta = collect_meta(html);
    ExternalEmbed {
        url: url.to_string(),
        provider: provider.to_string(),
        title: pick(&meta, &["og:title", "twitter:title"]).or_else(|| title_tag(html)),
        description: pick(
            &meta,
            &["og:description", "twitter:description", "description"],
        ),
        site_name: pick(&meta, &["og:site_name", "application-name"]),
        author_name: pick(&meta, &["author", "article:author"]),
        thumbnail_url: pick(&meta, &["og:image", "twitter:image"])
            .filter(|url| safe_image_url(url)),
    }
}

pub(super) fn should_unfurl(url: &str) -> bool {
    let Ok(url) = Url::parse(url) else {
        return false;
    };
    let host = url.host_str().unwrap_or("").trim_start_matches("www.");
    !matches!(
        host,
        "youtube.com" | "youtu.be" | "open.spotify.com" | "tiktok.com" | "x.com" | "twitter.com"
    ) && !has_ext(
        url.path(),
        &["png", "jpg", "jpeg", "gif", "webp", "mp4", "webm", "mp3"],
    )
}

pub(super) fn provider_label(url: &str) -> String {
    Url::parse(url)
        .ok()
        .and_then(|url| {
            url.host_str()
                .map(|host| host.trim_start_matches("www.").to_string())
        })
        .unwrap_or_else(|| "External".to_string())
}

fn collect_meta(html: &str) -> Vec<(String, String)> {
    html.split('<')
        .filter_map(|part| {
            part.strip_prefix("meta")
                .or_else(|| part.strip_prefix("META"))
        })
        .filter_map(|tag| {
            let name = attr(tag, "property").or_else(|| attr(tag, "name"))?;
            Some((
                name.to_ascii_lowercase(),
                html_decode(&attr(tag, "content")?),
            ))
        })
        .collect()
}

fn attr(tag: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let marker = format!("{name}={quote}");
        if let Some(start) = tag.find(&marker).map(|index| index + marker.len()) {
            let end = tag[start..].find(quote)?;
            return Some(tag[start..start + end].to_string());
        }
    }
    None
}

fn pick(meta: &[(String, String)], keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        meta.iter()
            .find(|(name, value)| name == key && !value.trim().is_empty())
            .map(|(_, value)| value.trim().to_string())
    })
}

fn title_tag(html: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let start = lower.find("<title>")? + "<title>".len();
    let end = lower[start..].find("</title>")?;
    Some(html_decode(html[start..start + end].trim()))
}

fn html_decode(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

fn has_ext(path: &str, extensions: &[&str]) -> bool {
    path.rsplit('.')
        .next()
        .map(str::to_ascii_lowercase)
        .is_some_and(|ext| extensions.contains(&ext.as_str()))
}

fn safe_image_url(value: &str) -> bool {
    Url::parse(value)
        .ok()
        .is_some_and(|url| matches!(url.scheme(), "http" | "https"))
}

#[cfg(test)]
mod tests {
    use super::{parse_html, provider_label, should_unfurl};

    #[test]
    fn parse_html_prefers_open_graph_metadata() {
        let html = r#"<html><head>
<meta property="og:title" content="Article title">
<meta property="og:description" content="Article summary">
<meta property="og:site_name" content="Example">
<meta property="og:image" content="https://example.com/card.jpg">
</head></html>"#;
        let embed = parse_html("https://example.com/post", "example.com", html);

        assert_eq!(embed.title.as_deref(), Some("Article title"));
        assert_eq!(embed.description.as_deref(), Some("Article summary"));
        assert_eq!(embed.site_name.as_deref(), Some("Example"));
        assert_eq!(
            embed.thumbnail_url.as_deref(),
            Some("https://example.com/card.jpg")
        );
    }

    #[test]
    fn unfurl_skips_player_and_direct_media_urls() {
        assert!(!should_unfurl("https://youtu.be/dQw4w9WgXcQ"));
        assert!(!should_unfurl("https://example.com/video.mp4"));
        assert!(should_unfurl("https://example.com/article"));
    }

    #[test]
    fn provider_label_uses_normalized_host() {
        assert_eq!(provider_label("https://www.example.com/a"), "example.com");
    }
}
