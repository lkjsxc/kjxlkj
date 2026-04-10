//! Security and icon settings sections

use super::layout::html_escape;
use crate::web::db::AppSettings;

pub fn security_section(settings: &AppSettings) -> String {
    format!(
        r#"{}<section class="section-block settings-section">
<div class="section-head"><h2>Security</h2></div>
<div class="surface settings-panel">
<form class="settings-section-grid" method="POST" action="/admin/password">
<label class="form-group">
<span>New password</span>
<input type="password" name="password" minlength="8" required>
</label>
<label class="form-group">
<span>Confirm new password</span>
<input type="password" name="confirm_password" minlength="8" required>
</label>
<div class="settings-submit-row settings-wide">
<button type="submit" class="btn">Change password</button>
</div>
</form>
</div>
</section>"#,
        site_icon_section(settings),
    )
}

fn site_icon_section(settings: &AppSettings) -> String {
    let status = settings
        .site_icon_content_type
        .as_deref()
        .map(|value| format!("Current icon: {}", html_escape(value)))
        .unwrap_or_else(|| "Current icon: bundled default".to_string());
    r#"<section class="section-block settings-section">
<div class="section-head"><h2>Site icon</h2></div>
<div class="surface settings-panel">
<form class="settings-section-grid" method="POST" action="/admin/site-icon" enctype="multipart/form-data">
<label class="form-group">
<span>Icon image</span>
<input type="file" name="icon" accept="image/*,.ico" required>
</label>
<p class="page-summary settings-wide">STATUS</p>
<div class="settings-submit-row settings-wide">
<button type="submit" class="btn">Upload icon</button>
</div>
</form>
</div>
</section>"#
        .replace("STATUS", &status)
        .to_string()
}
