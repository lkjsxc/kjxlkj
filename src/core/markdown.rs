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
    builder.clean(&html_out).to_string()
}
