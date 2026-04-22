use super::{attr, segment, segments};
use crate::core::markdown_links::html_text;
use url::Url;

pub(super) fn render(url: &Url, host: &str) -> String {
    let (provider, title) = match host {
        "github.com" | "gist.github.com" => ("GitHub", github_title(url)),
        "pixiv.net" => ("Pixiv", pixiv_title(url)),
        "npmjs.com" => ("npm", last_segment_title(url, "Package")),
        "crates.io" => ("crates.io", last_segment_title(url, "Crate")),
        "docs.rs" => ("docs.rs", last_segment_title(url, "Docs")),
        "pypi.org" => ("PyPI", last_segment_title(url, "Project")),
        "hub.docker.com" => ("Docker Hub", last_segment_title(url, "Repository")),
        "developer.mozilla.org" => ("MDN", last_segment_title(url, "Document")),
        _ if is_mastodon_like(url) => ("Mastodon", social_title(url)),
        _ => ("External", host.to_string()),
    };
    static_card(provider, url.as_str(), &title)
}

fn static_card(provider: &str, href: &str, title: &str) -> String {
    format!(
        r#"<div class="external-embed external-embed-card"><a href="{}" target="_blank" rel="noopener noreferrer"><span class="external-embed-provider">{}</span><strong>{}</strong><small>{}</small></a></div>"#,
        attr(href),
        html_text(provider),
        html_text(title),
        html_text(href),
    )
}

fn github_title(url: &Url) -> String {
    let parts = segments(url);
    match parts.as_slice() {
        [user, gist, ..] if url.host_str() == Some("gist.github.com") => {
            format!("{user} gist {gist}")
        }
        [owner, repo, "issues", number, ..] => format!("{owner}/{repo} issue #{number}"),
        [owner, repo, "pull", number, ..] => format!("{owner}/{repo} pull #{number}"),
        [owner, repo, "commit", sha, ..] => format!("{owner}/{repo} commit {sha}"),
        [owner, repo, ..] => format!("{owner}/{repo}"),
        _ => "GitHub".to_string(),
    }
}

fn pixiv_title(url: &Url) -> String {
    if segment(url, 0) == Some("artworks") {
        return format!("Artwork {}", segment(url, 1).unwrap_or(""));
    }
    last_segment_title(url, "Pixiv")
}

fn social_title(url: &Url) -> String {
    segment(url, 0).map_or_else(|| "Profile".to_string(), |user| format!("@{user}"))
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
