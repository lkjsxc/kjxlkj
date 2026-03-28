//! Layout and helper functions

const BASE_CSS: &str = include_str!("base.css");
const CONTENT_CSS: &str = include_str!("content.css");
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
{extra_head}
<style>{BASE_CSS}
{CONTENT_CSS}
{CONTROLS_CSS}
{SHELL_CSS}
{SURFACES_CSS}
{RESPONSIVE_CSS}
{EDITOR_CSS}</style>
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
        r#"<div class="shell-frame">
<header class="mobile-bar">
<div class="mobile-branding">
<a href="/" class="brand">kjxlkj</a>
<span class="mode-pill">{mode_label}</span>
</div>
<button type="button" class="btn rail-toggle" data-menu-toggle aria-controls="shell-rail" aria-expanded="false">Menu</button>
</header>
<div class="shell-backdrop" data-menu-backdrop hidden></div>
<div class="app-shell">
<aside id="shell-rail" class="shell-rail" data-menu-panel>
<div class="rail-head">
<div class="rail-branding">
<a href="/" class="brand">kjxlkj</a>
</div>
<span class="mode-pill">{mode_label}</span>
</div>
<div class="rail-body">{rail}</div>
</aside>
<main class="shell-main {page_class}">{main}</main>
</div>
</div>"#
    )
}

pub fn rail_section(title: &str, body: &str) -> String {
    format!(r#"<section class="rail-section" aria-label="{title}">{body}</section>"#)
}

pub fn primary_nav(active: &str, is_admin: bool) -> String {
    let mut links = vec![
        nav_link("/", "Public notes", active == "home"),
        nav_link("/search", "Search", active == "search"),
    ];
    if is_admin {
        links.push(nav_link("/admin", "Admin notes", active == "admin"));
    }
    format!(r#"<div class="rail-list">{}</div>"#, links.join(""))
}

fn nav_link(href: &str, label: &str, active: bool) -> String {
    format!(
        r#"<a href="{href}" class="rail-link{}"><span>{label}</span></a>"#,
        if active { " active" } else { "" }
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
