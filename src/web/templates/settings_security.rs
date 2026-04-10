//! Security settings form section

pub fn security_section() -> String {
    r#"<section class="section-block settings-section">
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
</section>"#
        .to_string()
}
