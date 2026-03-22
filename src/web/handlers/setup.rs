use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;

use crate::web::handlers::common::{enforce_setup_pending, internal_error};
use crate::web::password::hash_password;
use crate::web::state::WebState;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Debug, Deserialize)]
pub struct SetupForm {
    username: Option<String>,
    password: Option<String>,
}

impl SetupForm {
    fn username_for_display(&self) -> String {
        self.username
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .to_owned()
    }

    fn normalized_username(&self) -> Option<String> {
        normalize_required(self.username.as_deref())
    }

    fn normalized_password(&self) -> Option<String> {
        normalize_required(self.password.as_deref())
    }
}

pub async fn handle_get_setup(state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(render_setup_page("", &[]))
}

pub async fn handle_post_setup(
    state: web::Data<WebState>,
    form: web::Form<SetupForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }

    let username = form.normalized_username();
    let password = form.normalized_password();
    let errors = setup_validation_errors(username.is_some(), password.is_some());
    if !errors.is_empty() {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_setup_page(&form.username_for_display(), &errors));
    }
    let Some(username) = username else {
        return HttpResponse::BadRequest().finish();
    };
    let Some(password) = password else {
        return HttpResponse::BadRequest().finish();
    };

    let password_hash = match hash_password(&password) {
        Ok(value) => value,
        Err(error) => return internal_error(error),
    };

    match state
        .admin_store
        .create_admin(&username, &password_hash)
        .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/login"))
            .finish(),
        Err(error) => internal_error(error),
    }
}

fn normalize_required(value: Option<&str>) -> Option<String> {
    let trimmed = value.map(str::trim).unwrap_or_default();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn setup_validation_errors(has_username: bool, has_password: bool) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if !has_username {
        errors.push("username is required");
    }
    if !has_password {
        errors.push("password is required");
    }
    errors
}

fn render_setup_page(username: &str, errors: &[&str]) -> String {
    let escaped_username = escape_html(username);
    let error_block = render_error_block(errors);
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Initial setup</title>
</head>
<body>
  <main id="setup-page">
    <h1>Set up first admin account</h1>
    <p>Create the initial administrator credentials to unlock login.</p>
    {error_block}
    <form id="setup-form" method="post" action="/setup">
      <label for="username">Username</label>
      <input id="username" name="username" type="text" autocomplete="username" value="{escaped_username}" />
      <label for="password">Password</label>
      <input id="password" name="password" type="password" autocomplete="new-password" />
      <button type="submit">Create admin account</button>
    </form>
  </main>
</body>
</html>"#
    )
}

fn render_error_block(errors: &[&str]) -> String {
    if errors.is_empty() {
        return r#"<section id="setup-errors" aria-live="polite"></section>"#.to_owned();
    }

    let items = errors
        .iter()
        .map(|error| format!("<li>{}</li>", escape_html(error)))
        .collect::<String>();
    format!(
        r#"<section id="setup-errors" aria-live="polite">
      <p>Unable to complete setup:</p>
      <ul>{items}</ul>
    </section>"#
    )
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
