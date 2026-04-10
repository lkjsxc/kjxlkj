//! Authentication pages

use super::layout::{base, html_escape};
use crate::web::site::SiteContext;

pub fn setup_page(site: &SiteContext, error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>{} Setup</h1>
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
<div class="form-group">
<label for="setup_code">Setup code</label>
<input type="password" id="setup_code" name="setup_code" required>
</div>
<div class="auth-actions"><button type="submit" class="btn btn-primary">Create Account</button></div>
</form>
</div>
</div>"#,
        html_escape(&site.site_name),
    );
    base(
        &site.page_meta(
            "Setup",
            format!("Create the first admin account for {}.", site.site_name),
            false,
            None,
        ),
        &content,
        "",
        "",
    )
}

pub fn login_page(site: &SiteContext, error: Option<&str>, return_to: &str) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>{}</h1>
{error_html}
<form method="POST" action="/login">
<input type="hidden" name="return_to" value="{}">
<div class="form-group">
<label for="username">Username</label>
<input type="text" id="username" name="username" required>
</div>
<div class="form-group">
<label for="password">Password</label>
<input type="password" id="password" name="password" required>
</div>
<div class="auth-actions"><button type="submit" class="btn btn-primary">Sign In</button></div>
<p class="subtitle"><a href="/reset-password">Reset password</a></p>
</form>
</div>
</div>"#,
        html_escape(&site.site_name),
        html_escape(return_to),
    );
    base(
        &site.page_meta(
            "Login",
            format!("Sign in to manage {}.", site.site_name),
            false,
            None,
        ),
        &content,
        "",
        "",
    )
}

pub fn password_reset_page(site: &SiteContext, message: Option<&str>) -> String {
    let message_html = message
        .map(|value| format!(r#"<div class="error">{}</div>"#, html_escape(value)))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
<div class="auth-card">
<h1>Reset password</h1>
<p class="subtitle">Issue a console token, then enter it with the new password.</p>
{message_html}
<form method="POST" action="/reset-password/request">
<div class="auth-actions"><button type="submit" class="btn">Issue reset token</button></div>
</form>
<form method="POST" action="/reset-password">
<div class="form-group">
<label for="token">Reset token</label>
<input type="password" id="token" name="token" required>
</div>
<div class="form-group">
<label for="reset_password">New password</label>
<input type="password" id="reset_password" name="password" required minlength="8">
</div>
<div class="form-group">
<label for="reset_confirm_password">Confirm new password</label>
<input type="password" id="reset_confirm_password" name="confirm_password" required>
</div>
<div class="auth-actions"><button type="submit" class="btn btn-primary">Reset password</button></div>
</form>
<p class="subtitle"><a href="/login">Back to login</a></p>
</div>
</div>"#,
    );
    base(
        &site.page_meta(
            "Reset password",
            format!("Reset the admin password for {}.", site.site_name),
            false,
            None,
        ),
        &content,
        "",
        "",
    )
}
