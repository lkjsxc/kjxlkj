pub fn markdown_to_html(md: &str) -> String {
    let rendered = markdown::to_html(md);
    ammonia::clean(&rendered)
}
