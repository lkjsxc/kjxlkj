use super::page_html::escape_html;

pub fn render_article_main(slug: &str, html: &str) -> String {
    format!(
        "<main id=\"article-page\"><h1>{}</h1><article id=\"article-content\">{}</article></main>",
        escape_html(slug),
        html
    )
}
