use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;

use crate::web::handlers::common::{enforce_setup_pending, internal_error};
use crate::web::handlers::page_html::escape_html;
use crate::web::password::hash_password;
use crate::web::state::WebState;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Debug, Deserialize)]
pub struct SetupForm {
    password: Option<String>,
}

impl SetupForm {
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
        .body(render_setup_page(&[]))
}

pub async fn handle_post_setup(
    state: web::Data<WebState>,
    form: web::Form<SetupForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }

    let password = form.normalized_password();
    let errors = setup_validation_errors(password.is_some());
    if !errors.is_empty() {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_setup_page(&errors));
    }
    let Some(password) = password else {
        return HttpResponse::BadRequest().finish();
    };

    let password_hash = match hash_password(&password) {
        Ok(value) => value,
        Err(error) => return internal_error(error),
    };

    match state
        .admin_store
        .create_admin("admin", &password_hash)
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

fn setup_validation_errors(has_password: bool) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if !has_password {
        errors.push("password is required");
    }
    errors
}

fn render_setup_page(errors: &[&str]) -> String {
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
    <p>Create password for fixed admin account <code>admin</code>.</p>
    {error_block}
    <form id="setup-form" method="post" action="/setup">
      <input type="hidden" name="username" value="admin" />
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
