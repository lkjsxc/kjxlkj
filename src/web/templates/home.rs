//! Landing page template

use super::layout::{base, shell_page};
use super::model::IndexItem;

pub fn home_page(notes: &[IndexItem], is_admin: bool) -> String {
    let rail_items: String = notes
        .iter()
        .map(|note| {
            format!(
                r#"<a href="{}" class="rail-link"><span>{}</span><small>{}</small></a>"#,
                note.href, note.title, note.slug
            )
        })
        .collect();
    let rail = format!(
        r#"<section class="rail-section">
<h2>Public notes</h2>
<div class="rail-list">{}</div>
</section>
<section class="rail-section">
<h2>Session</h2>
<div class="rail-actions">{}</div>
</section>"#,
        if rail_items.is_empty() {
            r#"<p class="rail-empty">No public notes yet.</p>"#.to_string()
        } else {
            rail_items
        },
        if is_admin {
            r#"<a href="/admin" class="btn btn-primary">Open dashboard</a>"#.to_string()
        } else {
            r#"<a href="/login" class="btn btn-primary">Admin sign in</a>"#.to_string()
        }
    );
    let content = format!(
        r#"<section class="hero">
<p class="eyebrow">Markdown note system</p>
<h1>Notes stay readable first.</h1>
<p class="hero-copy">Server-rendered pages, revision history, and a compact shell built for AI-assisted editing.</p>
<div class="hero-actions">{}</div>
</section>"#,
        if is_admin {
            r#"<a href="/admin" class="btn btn-primary">Manage notes</a>"#
        } else {
            r#"<a href="/login" class="btn btn-primary">Admin sign in</a>"#
        }
    );
    base(
        "Home",
        &shell_page("Guest", &rail, &content, "home-page"),
        "",
        "",
    )
}
