//! Favorite sections for dashboard and settings

use super::layout::html_escape;
use super::list_sections::{favorite_browse_card, note_grid_section};
use super::model::IndexItem;
use super::settings_panel::settings_row;

pub fn dashboard_favorites_section(favorites: &[IndexItem]) -> String {
    note_grid_section(
        "Favorites",
        favorites,
        "No favorites yet.",
        "favorites-section",
        Some(r#"<a href="/admin/settings#favorites-settings" class="btn">Manage order</a>"#),
        Some(favorite_browse_card()),
    )
}

pub fn settings_favorite_order_section(favorites: &[IndexItem]) -> String {
    let body = if favorites.is_empty() {
        r#"<p class="surface-empty favorite-order-empty" data-settings-item>No favorites yet.</p>"#
            .to_string()
    } else {
        format!(
            r#"<div class="favorite-order-panel" data-settings-item>
<p class="page-summary">Drag to reorder favorites. Changes save immediately.</p>
<p class="favorite-order-error" data-favorite-order-error aria-live="polite"></p>
<ol class="favorite-order-list" data-favorite-order>{}</ol>
</div>"#,
            favorites
                .iter()
                .map(favorite_item)
                .collect::<Vec<_>>()
                .join("")
        )
    };
    let row = settings_row("Favorites", &body, "favorites-section");
    format!(r#"<div id="favorites-settings">{row}</div>"#)
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
