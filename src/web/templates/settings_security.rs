//! Security settings section

pub fn security_section() -> String {
    r#"<section class="section-block settings-section">
<div class="section-head"><h2>Security</h2></div>
<div class="surface settings-panel">
<form class="settings-section-grid" method="POST" action="/admin/password">
<label class="form-group" data-settings-item>
<span>Current password</span>
<input type="password" name="current_password" required>
</label>
<label class="form-group" data-settings-item>
<span>New password</span>
<input type="password" name="password" minlength="8" required>
</label>
<label class="form-group" data-settings-item>
<span>Confirm new password</span>
<input type="password" name="confirm_password" minlength="8" required>
</label>
<div class="settings-submit-row settings-wide" data-settings-item>
<button type="submit" class="btn">Change password</button>
</div>
</form>
</div>
 </section>"#
        .to_string()
}
