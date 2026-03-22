use actix_web::{http::header, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::core::settings::SiteSettings;
use crate::web::handlers::common::{
    clear_session_cookie, enforce_setup_completion, internal_error, session_cookie,
};
use crate::web::handlers::login_page::render_login_page;
use crate::web::password::verify_password;
use crate::web::session::{session_id_from_request, valid_session};
use crate::web::state::WebState;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";
const INVALID_CREDENTIALS_ERROR: &str = "invalid username or password";

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: Option<String>,
    password: Option<String>,
}

impl LoginForm {
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

pub async fn handle_get_login(request: HttpRequest, state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    let session = match valid_session(&request, &state).await {
        Ok(session) => session,
        Err(error) => return internal_error(error),
    };
    if session.is_some() {
        return HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin"))
            .finish();
    }

    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(render_login_page("", &[]))
}

pub async fn handle_post_login(
    state: web::Data<WebState>,
    form: web::Form<LoginForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    let username_for_display = form.username_for_display();
    let Some(username) = form.normalized_username() else {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_login_page(
                &username_for_display,
                &["username is required"],
            ));
    };
    let Some(password) = form.normalized_password() else {
        return HttpResponse::BadRequest()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_login_page(
                &username_for_display,
                &["password is required"],
            ));
    };

    let admin = match state.admin_store.find_admin_by_username(&username).await {
        Ok(Some(admin)) => admin,
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .content_type(HTML_CONTENT_TYPE)
                .body(render_login_page(&username, &[INVALID_CREDENTIALS_ERROR]));
        }
        Err(error) => return internal_error(error),
    };

    let verified = match verify_password(&password, &admin.password_hash) {
        Ok(value) => value,
        Err(error) => return internal_error(error),
    };
    if !verified {
        return HttpResponse::Unauthorized()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_login_page(&username, &[INVALID_CREDENTIALS_ERROR]));
    }

    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let timeout_minutes =
        SiteSettings::normalized_timeout_minutes(settings.session_timeout_minutes);
    let session = match state
        .session_store
        .create_session(admin.id, timeout_minutes)
        .await
    {
        Ok(session) => session,
        Err(error) => return internal_error(error),
    };

    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/admin"))
        .cookie(session_cookie(session.id))
        .finish()
}

pub async fn handle_post_logout(request: HttpRequest, state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    if let Some(session_id) = session_id_from_request(&request) {
        let _ = state.session_store.delete_session(session_id).await;
    }

    HttpResponse::NoContent()
        .cookie(clear_session_cookie())
        .finish()
}

fn normalize_required(value: Option<&str>) -> Option<String> {
    let trimmed = value.map(str::trim).unwrap_or_default();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
