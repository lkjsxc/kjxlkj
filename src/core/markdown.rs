//! Markdown rendering

use ammonia::Builder;
use pulldown_cmark::{html, Options, Parser};

use super::markdown_links::{
    escape_attr, is_local_file_href, poster_href, replace_local_resource_cards, variant_href,
};

pub fn render_markdown(body: &str) -> String {
    let mut html_out = String::new();
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    html::push_html(&mut html_out, Parser::new_ext(body, options));
    let mut builder = Builder::default();
    builder.add_tags(["input", "video", "source"]);
    builder.add_tag_attributes("input", ["checked", "disabled", "type"]);
    builder.add_tag_attributes("video", ["controls", "src", "poster", "preload"]);
    builder.add_tag_attributes("video", ["muted", "loop", "autoplay", "playsinline"]);
    builder.add_tag_attributes("source", ["src", "type"]);
    post_process_html(&builder.clean(&html_out).to_string())
}

fn post_process_html(html: &str) -> String {
    replace_local_resource_cards(&decorate_local_images(&decorate_local_videos(html)))
}

fn decorate_local_videos(html: &str) -> String {
    let mut rest = html;
    let mut output = String::new();
    let marker = "<video";
    while let Some(start) = rest.find(marker) {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start..];
        let Some(end) = after_marker.find('>') else {
            output.push_str(after_marker);
            return output;
        };
        output.push_str(&decorate_video_tag(&after_marker[..=end]));
        rest = &after_marker[end + 1..];
    }
    output.push_str(rest);
    output
}

fn decorate_local_images(html: &str) -> String {
    let mut rest = html;
    let mut output = String::new();
    let marker = "<img";
    while let Some(start) = rest.find(marker) {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start..];
        let Some(end) = after_marker.find('>') else {
            output.push_str(after_marker);
            return output;
        };
        output.push_str(&decorate_image_tag(&after_marker[..=end]));
        rest = &after_marker[end + 1..];
    }
    output.push_str(rest);
    output
}

fn decorate_video_tag(tag: &str) -> String {
    if tag.contains(" poster=") {
        return tag.to_string();
    }
    let Some(src) = attribute_value(tag, "src") else {
        return tag.to_string();
    };
    if !is_local_file_href(src) {
        return tag.to_string();
    }
    format!(
        "{} poster=\"{}\">",
        &tag[..tag.len().saturating_sub(1)],
        escape_attr(&poster_href(src))
    )
}

fn decorate_image_tag(tag: &str) -> String {
    let Some(src) = attribute_value(tag, "src") else {
        return tag.to_string();
    };
    if !is_local_file_href(src) || src.contains("variant=") {
        return tag.to_string();
    }
    replace_attribute(tag, "src", &variant_href(src, "display"))
}

fn attribute_value<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!(r#"{name}=""#);
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

fn replace_attribute(tag: &str, name: &str, value: &str) -> String {
    let marker = format!(r#"{name}=""#);
    let Some(start) = tag.find(&marker).map(|index| index + marker.len()) else {
        return tag.to_string();
    };
    let Some(end) = tag[start..].find('"') else {
        return tag.to_string();
    };
    format!(
        "{}{}{}",
        &tag[..start],
        escape_attr(value),
        &tag[start + end..]
    )
}
