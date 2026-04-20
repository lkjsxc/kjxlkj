use super::layout::html_escape;
use super::settings_panel::settings_row;
use crate::web::db::AppSettings;

pub(super) fn home_hero_section(settings: &AppSettings) -> String {
    settings_row(
        "Home hero",
        &format!(
            r#"<div class="settings-section-grid">
<label class="form-group settings-wide" data-settings-item><span>Home intro Markdown</span><textarea name="home_intro_markdown" rows="7" placeholder="Optional homepage introduction">{}</textarea></label>
</div>"#,
            html_escape(&settings.home_intro_markdown),
        ),
        "settings-home-hero-row",
    )
}

pub(super) fn home_sections_section(settings: &AppSettings) -> String {
    let mut rows = vec![
        section_item(
            "Popular",
            "home_popular",
            settings.home_popular_visible,
            settings.home_popular_position,
            settings.home_popular_limit,
        ),
        section_item(
            "Recently updated",
            "home_recent",
            settings.home_recent_visible,
            settings.home_recent_position,
            settings.home_recent_limit,
        ),
        section_item(
            "Favorites",
            "home_favorite",
            settings.home_favorite_visible,
            settings.home_favorite_position,
            settings.home_favorite_limit,
        ),
    ];
    rows.sort_by_key(|row| row.position);
    settings_row(
        "Home sections",
        &format!(
            r#"<div class="settings-table">
<div class="settings-row settings-row-head" data-settings-item><span>Section</span><span>Visible</span><span>Items</span></div>
<div class="settings-table-body" data-settings-order-list>{}</div>
</div>"#,
            rows.into_iter()
                .map(section_row)
                .collect::<Vec<_>>()
                .join("")
        ),
        "settings-home-sections-row",
    )
}

fn section_item(
    label: &'static str,
    prefix: &'static str,
    visible: bool,
    position: i64,
    limit: i64,
) -> HomeSectionItem {
    HomeSectionItem {
        label,
        prefix,
        visible,
        position,
        limit,
    }
}

fn section_row(item: HomeSectionItem) -> String {
    let visible_name = format!("{}_visible", item.prefix);
    let position_name = format!("{}_position", item.prefix);
    let limit_name = format!("{}_limit", item.prefix);
    format!(
        r#"<div class="settings-row settings-order-item" data-settings-item data-settings-order-item draggable="true">
<input type="hidden" name="{position_name}" value="{}">
<div class="settings-row-label-group">
<button type="button" class="settings-drag-handle" aria-label="Reorder home sections">Drag</button>
<span class="settings-row-label">{}</span>
</div>
<span class="settings-row-field settings-row-check"><input type="checkbox" name="{visible_name}" {}></span>
<span class="settings-row-field"><input type="number" name="{limit_name}" min="1" max="24" value="{}"></span>
</div>"#,
        item.position,
        item.label,
        if item.visible { "checked" } else { "" },
        item.limit,
    )
}

struct HomeSectionItem {
    label: &'static str,
    prefix: &'static str,
    visible: bool,
    position: i64,
    limit: i64,
}
