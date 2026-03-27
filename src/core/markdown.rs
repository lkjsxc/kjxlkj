//! Markdown rendering

use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(body: &str) -> String {
    let mut html_out = String::new();
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    html::push_html(&mut html_out, Parser::new_ext(body, options));
    html_out
}
