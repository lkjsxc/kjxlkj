use super::page_html::escape_html;

use crate::web::state::ArticleSummary;

use super::time_format::format_utc_timestamp;

pub fn render_home_main(articles: &[ArticleSummary], is_admin: bool) -> String {
    let article_rows = articles
        .iter()
        .map(|article| {
            let escaped = escape_html(&article.slug);
            let admin_badge = if is_admin {
                " <span class=\"admin-affordance\">admin</span>"
            } else {
                ""
            };
            let private_badge = if article.private {
                " <span class=\"article-private\">private</span>"
            } else {
                ""
            };
            let updated = format_utc_timestamp(article.updated_at);
            format!(
                "<li><a href=\"/article/{escaped}\" data-slug=\"{escaped}\">{escaped}</a>{admin_badge}{private_badge}<time>{}</time></li>",
                escape_html(&updated)
            )
        })
        .collect::<String>();
    format!(
        "<main id=\"home-page\"><h1>Articles</h1><section id=\"home-article-list\"><ul>{article_rows}</ul></section></main>"
    )
}
