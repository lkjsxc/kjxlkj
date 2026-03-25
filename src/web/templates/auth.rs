//! Authentication pages

use super::layout::base;

pub fn setup_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>kjxlkj Setup</h1>
<p class="subtitle">Create your admin account</p>
{error_html}
<form method="POST" action="/setup">
<div class="form-group">
<label for="username">Username</label>
<input type="text" id="username" name="username" required minlength="3">
</div>
<div class="form-group">
<label for="password">Password</label>
<input type="password" id="password" name="password" required minlength="8">
</div>
<div class="form-group">
<label for="confirm_password">Confirm Password</label>
<input type="password" id="confirm_password" name="confirm_password" required>
</div>
<button type="submit" class="btn btn-primary">Create Account</button>
</form>
</div>
</div>"#
    );
    base("Setup", &content, "", "")
}

pub fn login_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>kjxlkj</h1>
<p class="subtitle">Sign in</p>
{error_html}
<form method="POST" action="/login">
<div class="form-group">
<label for="username">Username</label>
<input type="text" id="username" name="username" required>
</div>
<div class="form-group">
<label for="password">Password</label>
<input type="password" id="password" name="password" required>
</div>
<button type="submit" class="btn btn-primary">Sign In</button>
</form>
</div>
</div>"#
    );
    base("Login", &content, "", "")
}
