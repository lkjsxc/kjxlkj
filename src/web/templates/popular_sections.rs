//! Popular-note section rendering

use super::list_sections::{note_grid_body, popular_browse_card};
use super::model::IndexItem;
use super::sections::section_with_actions_attrs;
use crate::web::db::PopularWindow;

pub fn home_popular_section(notes: &[IndexItem], window: PopularWindow) -> String {
    popular_notes_section("home", notes, window)
}

pub fn admin_popular_section(notes: &[IndexItem], window: PopularWindow) -> String {
    popular_notes_section("admin", notes, window)
}

fn popular_notes_section(surface: &str, notes: &[IndexItem], window: PopularWindow) -> String {
    section_with_actions_attrs(
        "Popular notes",
        Some(&window_controls(window)),
        &note_grid_body(notes, "No popular notes yet.", Some(popular_browse_card(window))),
        "note-section",
        &format!(r#"data-popular-section data-popular-surface="{surface}""#),
    )
}

fn window_controls(window: PopularWindow) -> String {
    format!(
        r#"<div class="popular-window-switch" role="group" aria-label="Popular notes window">{}{}</div>
<p class="error" data-popular-error hidden></p>"#,
        window_button(PopularWindow::Days7, window),
        format!(
            "{}{}",
            window_button(PopularWindow::Days30, window),
            window_button(PopularWindow::Days90, window)
        ),
    )
}

fn window_button(item: PopularWindow, current: PopularWindow) -> String {
    format!(
        r#"<button type="button" class="btn{}" data-popular-window="{}" aria-pressed="{}">{}</button>"#,
        if item == current { " btn-primary" } else { "" },
        item.as_str(),
        if item == current { "true" } else { "false" },
        item.as_str(),
    )
}
