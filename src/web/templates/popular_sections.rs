//! Popular-note section rendering

use super::list_sections::{note_grid_body, popular_browse_card};
use super::model::IndexItem;
use super::sections::section_with_actions_attrs;
use crate::web::db::PopularWindow;

pub fn home_popular_section(notes: &[IndexItem], window: PopularWindow) -> String {
    popular_resources_section("home", notes, window)
}

pub fn admin_popular_section(notes: &[IndexItem], window: PopularWindow) -> String {
    popular_resources_section("admin", notes, window)
}

fn popular_resources_section(surface: &str, notes: &[IndexItem], window: PopularWindow) -> String {
    section_with_actions_attrs(
        "Popular",
        Some(&window_controls(window)),
        &note_grid_body(
            notes,
            "No popular entries yet.",
            Some(popular_browse_card(window)),
        ),
        "resource-section",
        &format!(r#"data-popular-section data-popular-surface="{surface}""#),
    )
}

fn window_controls(window: PopularWindow) -> String {
    let buttons = [
        PopularWindow::Days1,
        PopularWindow::Days7,
        PopularWindow::Days30,
        PopularWindow::Days90,
        PopularWindow::All,
    ]
    .into_iter()
    .map(|item| window_button(item, window))
    .collect::<Vec<_>>()
    .join("");
    format!(
        r#"<div class="popular-window-switch" role="group" aria-label="Popular window">{buttons}</div>
<p class="error" data-popular-error hidden></p>"#,
    )
}

fn window_button(item: PopularWindow, current: PopularWindow) -> String {
    format!(
        r#"<button type="button" class="btn{}" data-popular-window="{}" aria-pressed="{}">{}</button>"#,
        if item == current { " btn-primary" } else { "" },
        item.as_str(),
        if item == current { "true" } else { "false" },
        item.button_label(),
    )
}
