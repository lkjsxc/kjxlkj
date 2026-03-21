use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut rendered = String::new();
    html::push_html(&mut rendered, parser);
    ammonia::clean(&rendered)
}
