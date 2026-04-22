use super::markdown_links::{html_text, local_url_card};
use super::MarkdownOptions;
use url::Url;

mod direct;
mod frames;
mod social;
mod static_cards;

pub fn render_url_embed(value: &str, options: MarkdownOptions<'_>) -> Option<String> {
    if value.starts_with('/') && !value.starts_with("//") {
        return local_url_card(value, value);
    }
    let url = Url::parse(value).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    let host = normalized_host(&url)?;
    direct::render(&url, &host)
        .or_else(|| frames::render(&url, &host, options))
        .or_else(|| social::render(&url, &host))
        .or_else(|| Some(static_cards::render(&url, &host)))
}

fn normalized_host(url: &Url) -> Option<String> {
    Some(
        url.host_str()?
            .trim_start_matches("www.")
            .to_ascii_lowercase(),
    )
}

pub(super) fn frame_card(provider: &str, href: &str, src: &str) -> String {
    format!(
        r#"<div class="external-embed external-embed-frame"><iframe title="{} embed" src="{}" loading="lazy" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></div>"#,
        html_text(provider),
        attr(src),
        attr(href),
        html_text(provider),
    )
}

pub(super) fn attr(value: &str) -> String {
    super::markdown_links::escape_attr(value)
}

pub(super) fn segments(url: &Url) -> Vec<&str> {
    url.path_segments()
        .map(|items| items.filter(|item| !item.is_empty()).collect())
        .unwrap_or_default()
}

pub(super) fn segment(url: &Url, index: usize) -> Option<&str> {
    url.path_segments()?
        .filter(|value| !value.is_empty())
        .nth(index)
}

pub(super) fn encoded(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}
