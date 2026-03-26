//! Layout and helper functions

const BASE_CSS: &str = include_str!("base.css");
const CONTROLS_CSS: &str = include_str!("controls.css");
const SHELL_CSS: &str = include_str!("shell.css");
const SURFACES_CSS: &str = include_str!("surfaces.css");
const RESPONSIVE_CSS: &str = include_str!("responsive.css");
const EDITOR_CSS: &str = include_str!("editor.css");
const SHELL_JS: &str = include_str!("shell.js");

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
<aside id="shell-rail" class="shell-rail">
<div class="rail-head">
<a href="/" class="brand">kjxlkj</a>
<span class="mode-pill">{mode_label}</span>
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
