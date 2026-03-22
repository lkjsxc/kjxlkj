use super::page_html::escape_html;

pub fn render_home_main(slugs: &[String], is_admin: bool) -> String {
    let article_rows = slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            let admin_badge = if is_admin {
                " <span class=\"admin-affordance\">admin</span>"
            } else {
                ""
            };
            format!(
                "<li><a href=\"/article/{escaped}\" data-slug=\"{escaped}\">{escaped}</a>{admin_badge}</li>"
            )
        })
        .collect::<String>();
    format!(
        "<main id=\"home-page\"><h1>Articles</h1><section id=\"home-article-list\"><ul>{article_rows}</ul></section></main>"
    )
}
