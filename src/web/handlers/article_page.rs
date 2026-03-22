use super::page_html::escape_html;

pub struct ArticleMainModel<'a> {
    pub slug: &'a str,
    pub html: &'a str,
    pub updated_at: &'a str,
    pub previous_slug: Option<&'a str>,
    pub next_slug: Option<&'a str>,
    pub is_admin: bool,
    pub inline_editor: Option<&'a str>,
    pub autosave_script: Option<&'a str>,
}

pub fn render_article_main(model: &ArticleMainModel<'_>) -> String {
    let nav = render_article_nav(model.previous_slug, model.next_slug);
    let history_link = if model.is_admin {
        format!(
            "<p><a href=\"/article/{}/history\">View history</a></p>",
            escape_html(model.slug)
        )
    } else {
        String::new()
    };
    let inline = model.inline_editor.unwrap_or_default();
    let script = model.autosave_script.unwrap_or_default();
    format!(
        "<main id=\"article-page\"><h1>{}</h1><p id=\"article-updated\">Last updated: {}</p>{}<article id=\"article-content\" data-markdown-editable=\"{}\">{}</article>{}{}{}</main>",
        escape_html(model.slug),
        escape_html(model.updated_at),
        nav,
        if model.is_admin { "true" } else { "false" },
        model.html,
        history_link,
        inline,
        script
    )
}

fn render_article_nav(previous_slug: Option<&str>, next_slug: Option<&str>) -> String {
    let previous = previous_slug.map_or_else(String::new, |slug| {
        format!(
            "<a id=\"article-prev\" href=\"/article/{}\">Previous</a>",
            escape_html(slug)
        )
    });
    let next = next_slug.map_or_else(String::new, |slug| {
        format!(
            "<a id=\"article-next\" href=\"/article/{}\">Next</a>",
            escape_html(slug)
        )
    });
    format!("<nav id=\"article-nav\">{}{}</nav>", previous, next)
}
