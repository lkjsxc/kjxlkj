use super::page_html::{escape_html, wrap_page};

pub fn render_shell_page(
    site_title: &str,
    page_title: &str,
    page_main_html: &str,
    article_slugs: &[String],
    is_admin: bool,
) -> String {
    let full_title = format!("{page_title} · {site_title}");
    let body = format!(
        "<div id=\"app-shell\" data-nav-open=\"false\" data-role=\"{}\">{}{}<div id=\"app-main\">{}</div></div><script src=\"/static/app-shell.js\" defer></script>",
        if is_admin { "admin" } else { "public" },
        render_topbar(site_title),
        render_nav(article_slugs, is_admin),
        page_main_html,
    );
    wrap_page(&full_title, &body)
}

fn render_topbar(site_title: &str) -> String {
    format!(
        "<header id=\"app-topbar\"><button id=\"app-nav-toggle\" type=\"button\" aria-label=\"Toggle menu\" aria-expanded=\"false\">☰</button><a id=\"app-topbar-home\" href=\"/\">{}</a></header>",
        escape_html(site_title)
    )
}

fn render_nav(article_slugs: &[String], is_admin: bool) -> String {
    let article_rows = article_slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            format!("<li><a href=\"/article/{escaped}\" data-slug=\"{escaped}\">{escaped}</a></li>")
        })
        .collect::<String>();
    let mut action_rows = vec![
        "<li><a href=\"/\">Home</a></li>".to_owned(),
        "<li><a href=\"/search\">Search</a></li>".to_owned(),
    ];
    if is_admin {
        action_rows.push("<li><a href=\"/admin\">Admin</a></li>".to_owned());
        action_rows.push("<li><a href=\"/admin/settings\">Settings</a></li>".to_owned());
        action_rows.push("<li><a href=\"/admin/trash\">Trash</a></li>".to_owned());
    }
    format!(
        "<nav id=\"app-nav\"><section id=\"app-nav-actions\"><h2>Navigate</h2><ul>{}</ul></section><section id=\"app-nav-articles\"><h2>Articles</h2><ul>{}</ul></section></nav>",
        action_rows.join(""),
        article_rows
    )
}
