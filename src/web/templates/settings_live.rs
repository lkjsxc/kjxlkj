//! Live settings rows

use super::layout::html_escape;
use super::settings_panel::settings_row;
use crate::core::live_settings::{LIVE_FPS_VALUES, LIVE_HEIGHTS};
use crate::web::db::AppSettings;

pub(super) fn live_default_source_row(settings: &AppSettings) -> String {
    settings_row(
        "Live/Default_source",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Live/Default_source</span><select name="live_default_source">{}</select></label>"#,
            source_options(&settings.live_default_source),
        ),
        "settings-live-source-row",
    )
}

pub(super) fn live_default_quality_row(settings: &AppSettings) -> String {
    settings_row(
        "Live/Default_quality",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Live/Default_quality</span><select name="live_default_height">{}</select></label>"#,
            number_options(LIVE_HEIGHTS, settings.live_default_height, "p"),
        ),
        "settings-live-quality-row",
    )
}

pub(super) fn live_default_fps_row(settings: &AppSettings) -> String {
    settings_row(
        "Live/Default_fps",
        &format!(
            r#"<label class="form-group" data-settings-item><span>Live/Default_fps</span><select name="live_default_fps">{}</select></label>"#,
            number_options(LIVE_FPS_VALUES, settings.live_default_fps, " fps"),
        ),
        "settings-live-fps-row",
    )
}

pub(super) fn live_default_microphone_row(settings: &AppSettings) -> String {
    settings_row(
        "Live/Microphone_default",
        &format!(
            r#"<label class="check-row check-row-field" data-settings-item><input type="checkbox" name="live_default_microphone_enabled" {}><span>Live/Microphone_default</span></label>
<p class="page-summary" data-settings-item>Controls whether new broadcasts request microphone audio by default.</p>"#,
            if settings.live_default_microphone_enabled {
                "checked"
            } else {
                ""
            },
        ),
        "settings-live-mic-row",
    )
}

fn source_options(current: &str) -> String {
    [("screen", "Screen"), ("camera", "Camera")]
        .into_iter()
        .map(|(value, label)| option(value, label, value == current))
        .collect::<Vec<_>>()
        .join("")
}

fn number_options(values: &[i64], current: i64, suffix: &str) -> String {
    values
        .iter()
        .map(|value| {
            option(
                &value.to_string(),
                &format!("{value}{suffix}"),
                *value == current,
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn option(value: &str, label: &str, selected: bool) -> String {
    format!(
        r#"<option value="{}"{}>{}</option>"#,
        html_escape(value),
        if selected { " selected" } else { "" },
        html_escape(label),
    )
}
