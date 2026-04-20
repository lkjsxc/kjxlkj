use super::layout::html_escape;
use super::settings_panel::settings_row;
use crate::web::db::AppSettings;

pub fn site_icon_section(settings: &AppSettings) -> String {
    let status = settings
        .site_icon_content_type
        .as_deref()
        .map(|value| format!("Current icon: {}", html_escape(value)))
        .unwrap_or_else(|| "Current icon: bundled default".to_string());
    settings_row(
        "Site icon",
        &format!(
            r#"<div class="settings-icon-grid" data-settings-item>
<img src="/assets/site-icon" alt="" class="settings-icon-preview" data-site-icon-preview>
<div class="settings-icon-copy">
<p class="page-summary" data-site-icon-current>{status}</p>
<p class="page-summary" data-site-icon-status aria-live="polite"></p>
</div>
<div class="settings-submit-row">
<button type="button" class="btn" data-site-icon-upload>Upload icon</button>
<button type="button" class="btn" data-site-icon-reset{}>Reset icon</button>
</div>
<input type="file" accept="image/*,.ico" hidden data-site-icon-input>
</div>"#,
            if settings.site_icon_content_type.is_some() {
                ""
            } else {
                " hidden"
            },
        ),
        "settings-icon-row",
    )
}
