//! Note display pages

use super::layout::{base, build_sidebar, format_date, html_escape, render_markdown};
use crate::web::db::Record;
use chrono::{DateTime, Utc};

const EDITOR_JS: &str = include_str!("editor.js");

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
    let back_link = if is_admin {
        r#"<a href="/admin">← Back to Admin</a>"#
    } else {
        r#"<a href="/">← Back</a>"#
    };
    let content = format!(
        r#"<div class="app-layout">
<nav class="sidebar">
<div class="logo"><a href="/">kjxlkj</a></div>
<div class="sidebar-actions">{back_link}</div>
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
</div>"#
    );
    base(&title, &content, extra_head)
}
