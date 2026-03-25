//! Layout and helper functions

const CSS: &str = include_str!("style.css");

pub fn base(title: &str, content: &str, extra_head: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} - kjxlkj</title>
<style>{CSS}</style>
{extra_head}
</head>
<body>{content}</body>
</html>"#
    )
}

pub fn not_found_page() -> String {
    let content = r#"<div class="auth-container">
<div class="auth-card">
<h1>404</h1>
<p class="subtitle">Note not found</p>
<a href="/" class="btn btn-primary">Go Home</a>
</div>
</div>"#;
    base("Not Found", content, "")
}

pub fn build_sidebar(notes: &[(String, String)], active: Option<&str>, is_admin: bool) -> String {
    let items: String = notes
        .iter()
        .map(|(slug, title)| {
            let class = if active == Some(slug.as_str()) {
                " active"
            } else {
                ""
            };
            format!(r#"<a href="/{slug}" class="sidebar-item{class}">{title}</a>"#)
        })
        .collect();
    let logout = if is_admin {
        r#"<form method="POST" action="/logout" class="logout-form">
<button type="submit" class="btn btn-sm">Logout</button>
</form>"#
    } else {
        ""
    };
    format!(
        r#"<nav class="sidebar">
<div class="logo"><a href="/">kjxlkj</a></div>
<div class="sidebar-items">{items}</div>
{logout}
</nav>"#
    )
}

pub fn format_date(dt: &chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%B %d, %Y at %I:%M %p").to_string()
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn render_markdown(body: &str) -> String {
    let mut html = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(h) = trimmed.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>", html_escape(h)));
        } else if let Some(h) = trimmed.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>", html_escape(h)));
        } else if let Some(h) = trimmed.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>", html_escape(h)));
        } else if trimmed.is_empty() {
            html.push_str("<br>");
        } else {
            html.push_str(&format!("<p>{}</p>", html_escape(trimmed)));
        }
    }
    html
}
