pub fn render_home_page(slugs: &[String], is_admin: bool) -> String {
    let article_rows = slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            let admin_badge = if is_admin {
                r#" <span class="admin-affordance">admin</span>"#
            } else {
                ""
            };
            format!(
                r#"<li><a href="/article/{escaped}" data-slug="{escaped}">{escaped}</a>{admin_badge}</li>"#
            )
        })
        .collect::<String>();

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Articles</title>
</head>
<body>
  <main id="home-page">
    <h1>Articles</h1>
    <section id="home-article-list">
      <ul>{article_rows}</ul>
    </section>
  </main>
</body>
</html>"#
    )
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
