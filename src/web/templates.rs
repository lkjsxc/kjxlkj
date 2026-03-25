//! HTML templates

use crate::web::db::Record;
use chrono::{DateTime, Utc};

const CSS: &str = include_str!("templates/style.css");
const EDITOR_JS: &str = include_str!("templates/editor.js");

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

pub fn setup_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>kjxlkj Setup</h1>
<p class="subtitle">Create your admin account</p>
{error_html}
<form method="POST" action="/setup">
<div class="form-group">
<label for="username">Username</label>
<input type="text" id="username" name="username" required minlength="3">
</div>
<div class="form-group">
<label for="password">Password</label>
<input type="password" id="password" name="password" required minlength="8">
</div>
<div class="form-group">
<label for="confirm_password">Confirm Password</label>
<input type="password" id="confirm_password" name="confirm_password" required>
</div>
<button type="submit" class="btn btn-primary">Create Account</button>
</form>
</div>
</div>"#
    );
    base("Setup", &content, "")
}

pub fn login_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>kjxlkj</h1>
<p class="subtitle">Sign in</p>
{error_html}
<form method="POST" action="/login">
<div class="form-group">
<label for="username">Username</label>
<input type="text" id="username" name="username" required>
</div>
<div class="form-group">
<label for="password">Password</label>
<input type="password" id="password" name="password" required>
</div>
<button type="submit" class="btn btn-primary">Sign In</button>
</form>
</div>
</div>"#
    );
    base("Login", &content, "")
}

pub fn home_page(notes: &[(String, String)], is_admin: bool) -> String {
    let sidebar = build_sidebar(notes, None, is_admin);
    let login_link = if is_admin {
        r#"<a href="/admin" class="btn btn-primary">Admin Dashboard</a>"#
    } else {
        r#"<a href="/login" class="btn btn-primary">Admin Login</a>"#
    };
    let content = format!(
        r#"<div class="app-layout">
{sidebar}
<main class="main-content">
<div class="welcome">
<h1>kjxlkj</h1>
<p class="subtitle">Notes</p>
{login_link}
</div>
</main>
</div>"#
    );
    base("Home", &content, "")
}

pub fn admin_page(notes: &[(String, String, bool, DateTime<Utc>)]) -> String {
    let sidebar_items: Vec<_> = notes
        .iter()
        .map(|(s, t, _, _)| (s.clone(), t.clone()))
        .collect();
    let sidebar = build_sidebar(&sidebar_items, None, true);
    let rows: String = notes
        .iter()
        .map(|(slug, title, is_private, updated)| {
            let icon = if *is_private { "🔒" } else { "🌐" };
            let date = format_date(updated);
            format!(
                r#"<a href="/{slug}" class="note-item">
<span class="note-title">{title}</span>
<span class="note-meta">{icon} {date}</span>
</a>"#
            )
        })
        .collect();
    let empty = if notes.is_empty() {
        r#"<p class="empty">No notes yet</p>"#
    } else {
        ""
    };
    let content = format!(
        r#"<div class="app-layout">
{sidebar}
<main class="main-content">
<header class="page-header">
<h1>All Notes</h1>
<button class="btn btn-primary" onclick="createNote()">New Note</button>
</header>
<div class="notes-list">{rows}{empty}</div>
</main>
</div>
<script>{EDITOR_JS}</script>"#
    );
    base("Admin", &content, "")
}

pub fn note_page(record: &Record, is_admin: bool) -> String {
    let title = crate::core::extract_title(&record.body).unwrap_or_else(|| record.slug.clone());
    let updated = format_date(&record.updated_at);
    let extra_head = if is_admin {
        r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/simplemde/latest/simplemde.min.css">
<script src="https://cdn.jsdelivr.net/simplemde/latest/simplemde.min.js"></script>"#
    } else {
        ""
    };
    let toggle = if is_admin {
        let checked = if record.is_private { "checked" } else { "" };
        format!(
            r#"<label class="toggle">
<input type="checkbox" id="private-toggle" {checked} onchange="togglePrivate()">
<span class="toggle-label">{}</span>
</label>"#,
            if record.is_private {
                "Private"
            } else {
                "Public"
            }
        )
    } else {
        String::new()
    };
    let editor = if is_admin {
        format!(
            r#"<textarea id="editor">{}</textarea>
<script>
var simplemde = new SimpleMDE({{ element: document.getElementById("editor"), spellChecker: false }});
simplemde.codemirror.on("blur", function() {{ saveNote(); }});
var currentSlug = "{}";
var isPrivate = {};
{EDITOR_JS}
</script>"#,
            html_escape(&record.body),
            record.slug,
            record.is_private
        )
    } else {
        format!(
            r#"<div class="rendered-content">{}</div>"#,
            render_markdown(&record.body)
        )
    };
    let content = format!(
        r#"<div class="app-layout">
<nav class="sidebar">
<div class="logo"><a href="/">kjxlkj</a></div>
<div class="sidebar-actions">
{back_link}
</div>
</nav>
<main class="main-content note-view">
<header class="note-header">
<h1>{title}</h1>
{toggle}
</header>
<div class="note-body">{editor}</div>
<footer class="note-footer">
<span class="updated">Last updated: {updated}</span>
<span id="save-status"></span>
</footer>
</main>
</div>"#,
        back_link = if is_admin {
            r#"<a href="/admin">← Back to Admin</a>"#
        } else {
            r#"<a href="/">← Back</a>"#
        }
    );
    base(&title, &content, extra_head)
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

fn build_sidebar(notes: &[(String, String)], active: Option<&str>, is_admin: bool) -> String {
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

fn format_date(dt: &DateTime<Utc>) -> String {
    dt.format("%B %d, %Y at %I:%M %p").to_string()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_markdown(body: &str) -> String {
    // Simple markdown rendering (headers, paragraphs, bold, italic)
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
