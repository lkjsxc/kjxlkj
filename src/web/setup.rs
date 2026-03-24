use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;

use crate::core::auth::FIXED_ADMIN_USERNAME;

use super::guards::enforce_setup_pending;
use super::html::escape_html;
use super::password::hash_password;
use super::state::AppState;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Debug, Deserialize)]
pub struct SetupForm {
    password: Option<String>,
}

impl SetupForm {
    fn normalized_password(&self) -> Option<String> {
        let trimmed = self.password.as_deref().map(str::trim).unwrap_or_default();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    }
}

pub async fn get_setup(state: web::Data<AppState>) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(render_setup_page(&[]))
}

pub async fn post_setup(state: web::Data<AppState>, form: web::Form<SetupForm>) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }
    let Some(password) = form.normalized_password() else {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_setup_page(&["password is required"]));
    };

    let password_hash = match hash_password(&password) {
        Ok(value) => value,
        Err(error) => {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    };

    match state.auth_store.create_admin(&password_hash).await {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/login"))
            .finish(),
        Err(error) => HttpResponse::InternalServerError()
            .content_type("text/plain; charset=utf-8")
            .body(format!("{}: {}", error.code(), error)),
    }
}

fn render_setup_page(errors: &[&str]) -> String {
    let error_block = if errors.is_empty() {
        r#"<section id="setup-errors" aria-live="polite"></section>"#.to_owned()
    } else {
        let items = errors
            .iter()
            .map(|error| format!("<li>{}</li>", escape_html(error)))
            .collect::<String>();
        format!(
            r#"<section id="setup-errors" aria-live="polite"><p>Unable to complete setup:</p><ul>{items}</ul></section>"#
        )
    };

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
    <p>Create password for fixed admin account <code>{}</code>.</p>
    {error_block}
    <form id="setup-form" method="post" action="/setup">
      <label for="password">Password</label>
      <input id="password" name="password" type="password" autocomplete="new-password" />
      <button type="submit">Create admin account</button>
    </form>
  </main>
</body>
</html>"#,
        escape_html(FIXED_ADMIN_USERNAME)
    )
}
