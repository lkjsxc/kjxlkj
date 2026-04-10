//! Markdown rendering

use ammonia::Builder;
use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(body: &str) -> String {
    let mut html_out = String::new();
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    html::push_html(&mut html_out, Parser::new_ext(body, options));
    let mut builder = Builder::default();
    builder.add_tags(["video", "source"]);
    builder.add_tag_attributes("video", ["controls", "src", "poster", "preload"]);
    builder.add_tag_attributes("video", ["muted", "loop", "autoplay", "playsinline"]);
    builder.add_tag_attributes("source", ["src", "type"]);
    local_file_cards(&builder.clean(&html_out).to_string())
}

fn local_file_cards(html: &str) -> String {
    let mut rest = html;
    let mut output = String::new();
    let marker = "<p><a href=\"";
    while let Some(start) = rest.find(marker) {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start + marker.len()..];
        let Some(href_end) = after_marker.find('"') else {
            output.push_str(&rest[start..]);
            return output;
        };
        let href = &after_marker[..href_end];
        let after_href = &after_marker[href_end..];
        let Some(label_end) = after_href.find("</a></p>") else {
            output.push_str(&rest[start..]);
            return output;
        };
        let block_len = marker.len() + href_end + label_end + "</a></p>".len();
        if is_local_file_href(href) {
            output.push_str(&local_file_card(href));
        } else {
            output.push_str(&rest[start..start + block_len]);
        }
        rest = &rest[start + block_len..];
    }
    output.push_str(rest);
    output
}

fn local_file_card(href: &str) -> String {
    format!(
        r#"<div class="local-url-card"><a href="{0}"><img src="{1}" alt="" loading="lazy"><span>{0}</span></a></div>"#,
        escape_attr(href),
        escape_attr(&thumbnail_href(href)),
    )
}

fn thumbnail_href(href: &str) -> String {
    if href.contains('?') {
        format!("{href}&variant=card")
    } else {
        format!("{href}?variant=card")
    }
}

fn is_local_file_href(href: &str) -> bool {
    href.starts_with('/')
        && !href.starts_with("//")
        && href.split('?').next().unwrap_or("").ends_with("/file")
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
