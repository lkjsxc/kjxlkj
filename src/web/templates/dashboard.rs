//! Admin dashboard template

use super::layout::{base, shell_page};
use super::model::IndexItem;

const EDITOR_JS: &str = include_str!("editor.js");

pub fn admin_page(notes: &[IndexItem]) -> String {
    let rail_notes: String = notes
        .iter()
        .map(|note| {
            format!(
                r#"<a href="{}" class="rail-link"><span>{}</span><small>{} · {}</small><small class="rail-summary">{}</small></a>"#,
                note.href, note.title, note.status, note.slug, note.summary
            )
        })
        .collect();
    let rail = format!(
        r#"<section class="rail-section">
<h2>All notes</h2>
<div class="rail-list">{}</div>
</section>
<section class="rail-section">
<h2>Actions</h2>
<div class="rail-actions">
<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>
<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>
</div>
</section>"#,
        if rail_notes.is_empty() {
            r#"<p class="rail-empty">No notes yet.</p>"#.to_string()
        } else {
            rail_notes
        }
    );
    let rows: String = notes
        .iter()
        .map(|note| {
            format!(
                r#"<a href="{}" class="index-card">
<div class="card-body">
<p class="card-title">{}</p>
<p class="card-summary">{}</p>
<p class="card-slug">{}</p>
</div>
<div class="card-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</a>"#,
                note.href, note.title, note.summary, note.slug, note.status, note.meta
            )
        })
        .collect();
    let content = format!(
        r#"<header class="page-head">
<div>
<p class="eyebrow">Knowledge console</p>
<h1>All notes</h1>
<p class="page-summary">Dense note operations, full visibility context, and direct paths back into editing.</p>
</div>
<button type="button" class="btn btn-primary" onclick="createNote()">New note</button>
</header>
<section class="stack">{}</section>
<script>{}</script>"#,
        if rows.is_empty() {
            r#"<p class="surface-empty">No notes yet.</p>"#.to_string()
        } else {
            rows
        },
        EDITOR_JS
    );
    base(
        "Admin",
        &shell_page("Admin", &rail, &content, "dashboard-page"),
        "",
        "",
    )
}
