//! Dashboard favorite ordering section

use super::layout::html_escape;
use super::model::IndexItem;
use super::sections::section;

pub fn favorite_order_section(favorites: &[IndexItem], extra_card: &str) -> String {
    let body = if favorites.is_empty() {
        format!(
            r#"<p class="surface-empty favorite-order-empty">No favorites yet.</p>{extra_card}"#
        )
    } else {
        format!(
            r#"<div class="favorite-order-panel">
<p class="favorite-order-error" data-favorite-order-error aria-live="polite"></p>
<ol class="favorite-order-list" data-favorite-order>{}</ol>
</div>{extra_card}"#,
            favorites
                .iter()
                .map(favorite_item)
                .collect::<Vec<_>>()
                .join("")
        )
    };
    section("Favorites", &body, "favorites-section")
}

fn favorite_item(note: &IndexItem) -> String {
    let visibility = note
        .visibility
        .map(|value| format!(r#"<span class="status-pill">{value}</span>"#))
        .unwrap_or_default();
    format!(
        r#"<li class="favorite-order-item" data-favorite-id="{}" draggable="true">
<button type="button" class="favorite-order-handle" aria-label="Reorder favorites">Drag</button>
<a href="{}" class="favorite-order-link">
<span class="favorite-order-title">{}</span>
<span class="favorite-order-summary">{}</span>
</a>
<div class="favorite-order-meta">{}<small><span>Updated</span>{}</small></div>
</li>"#,
        note.id,
        note.href,
        html_escape(&note.title),
        html_escape(&note.summary),
        visibility,
        note.updated_at
    )
}
