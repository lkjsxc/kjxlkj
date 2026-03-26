//! Layout and helper functions

const BASE_CSS: &str = include_str!("base.css");
const CONTROLS_CSS: &str = include_str!("controls.css");
const SHELL_CSS: &str = include_str!("shell.css");
const SURFACES_CSS: &str = include_str!("surfaces.css");
const RESPONSIVE_CSS: &str = include_str!("responsive.css");
const EDITOR_CSS: &str = include_str!("editor.css");
const SHELL_JS: &str = include_str!("shell.js");
const MENU_ICON: &str = r#"<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 7h16M4 12h16M4 17h12"/></svg>"#;
const CLOSE_ICON: &str = r#"<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 7l10 10M17 7L7 17"/></svg>"#;

pub fn base(title: &str, content: &str, extra_head: &str, extra_script: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<script>document.documentElement.classList.add('js');</script>
<title>{title} - kjxlkj</title>
<style>{BASE_CSS}
{CONTROLS_CSS}
{SHELL_CSS}
{SURFACES_CSS}
{RESPONSIVE_CSS}
{EDITOR_CSS}</style>
{extra_head}
</head>
<body>{content}<script>{SHELL_JS}</script>{extra_script}</body>
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
    base("Not Found", content, "", "")
}

pub fn shell_page(mode_label: &str, rail: &str, main: &str, page_class: &str) -> String {
    format!(
        r#"<div class="app-shell">
<button type="button" class="menu-button icon-button" data-menu-toggle aria-expanded="false" aria-controls="shell-rail" aria-label="Open navigation">{MENU_ICON}<span class="visually-hidden">Open navigation</span></button>
<div class="drawer-backdrop" data-menu-close></div>
<aside id="shell-rail" class="shell-rail" aria-hidden="false">
<div class="rail-head">
<a href="/" class="brand">kjxlkj</a>
<span class="mode-pill">{mode_label}</span>
<button type="button" class="rail-close icon-button" data-menu-close aria-label="Close navigation">{CLOSE_ICON}<span class="visually-hidden">Close navigation</span></button>
</div>
<div class="rail-body">{rail}</div>
</aside>
<main class="shell-main {page_class}">{main}</main>
</div>"#
    )
}

pub fn render_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let iso = dt.to_rfc3339();
    format!(
        r#"<time class="local-time" datetime="{iso}" data-utc="{iso}">{}</time>"#,
        format_date(dt)
    )
}

pub fn format_date(dt: &chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
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
