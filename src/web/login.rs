use actix_web::{http::header, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use super::guards::enforce_setup_completion;
use super::html::escape_html;
use super::password::verify_password;
use super::session::{
    clear_session_cookie, session_cookie, session_id_from_request, valid_session,
};
use super::state::AppState;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";
const INVALID_CREDENTIALS: &str = "invalid password";

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    password: Option<String>,
}

impl LoginForm {
    fn normalized_password(&self) -> Option<String> {
        let trimmed = self.password.as_deref().map(str::trim).unwrap_or_default();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    }
}

pub async fn get_login(request: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }
    let session = match valid_session(&request, &state).await {
        Ok(value) => value,
        Err(error) => {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    };
    if session.is_some() {
        return HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin"))
            .finish();
    }
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(render_login_page(&[]))
}

pub async fn post_login(
    request: HttpRequest,
    state: web::Data<AppState>,
    form: web::Form<LoginForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }
    let Some(password) = form.normalized_password() else {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_login_page(&["password is required"]));
    };

    let admin = match state.auth_store.load_admin().await {
        Ok(Some(value)) => value,
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .content_type(HTML_CONTENT_TYPE)
                .body(render_login_page(&[INVALID_CREDENTIALS]));
        }
        Err(error) => {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    };

    let verified = match verify_password(&password, &admin.password_hash) {
        Ok(value) => value,
        Err(error) => {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    };
    if !verified {
        return HttpResponse::Unauthorized()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_login_page(&[INVALID_CREDENTIALS]));
    }

    let session = match state
        .auth_store
        .create_session(admin.id, state.session_timeout_minutes)
        .await
    {
        Ok(value) => value,
        Err(error) => {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    };
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/admin"))
        .cookie(session_cookie(session.id, &request))
        .finish()
}

pub async fn post_logout(request: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }
    if let Some(session_id) = session_id_from_request(&request) {
        if let Err(error) = state.auth_store.delete_session(session_id).await {
            return HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(format!("{}: {}", error.code(), error));
        }
    }
    HttpResponse::NoContent()
        .cookie(clear_session_cookie(&request))
        .finish()
}

fn render_login_page(errors: &[&str]) -> String {
    let error_block = if errors.is_empty() {
        r#"<section id="login-errors" aria-live="polite"></section>"#.to_owned()
    } else {
        let items = errors
            .iter()
            .map(|error| format!("<li>{}</li>", escape_html(error)))
            .collect::<String>();
        format!(
            r#"<section id="login-errors" aria-live="polite"><p>Unable to sign in:</p><ul>{items}</ul></section>"#
        )
    };
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Admin login</title>
</head>
<body>
  <main id="login-page">
    <h1>Admin login</h1>
    <p>Username is fixed as <code>admin</code>. Enter password to continue.</p>
    {error_block}
    <form id="login-form" method="post" action="/login">
      <label for="password">Password</label>
      <input id="password" name="password" type="password" autocomplete="current-password" />
      <button type="submit">Sign in</button>
    </form>
  </main>
</body>
</html>"#
    )
}
