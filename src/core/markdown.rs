//! Markdown rendering

use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(body: &str) -> String {
    let mut html_out = String::new();
    html::push_html(&mut html_out, Parser::new_ext(body, Options::empty()));
    html_out
}
