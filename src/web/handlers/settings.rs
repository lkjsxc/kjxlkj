//! Admin settings handler

use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::handlers::settings_input::{validate_settings_form, SettingsForm};
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use axum::extract::{Form, State};
use axum::http::{HeaderMap, Uri};
use axum::response::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PasswordForm {
    pub current_password: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn settings_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    if !session::check_session(&headers, pool).await? {
        return Ok(http::redirect(&session::login_url(&uri)));
    }
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(http::html(templates::settings_page(&settings, &site)))
}

pub async fn settings_submit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<SettingsForm>,
) -> Result<Response, AppError> {
    session::require_session(&headers, &state.pool).await?;
    let pool = &state.pool;
    let current = db::get_settings(pool).await?;
    db::update_settings(pool, &validate_settings_form(&form, &current)?).await?;
    Ok(http::see_other("/admin/settings"))
}

pub async fn password_submit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<PasswordForm>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let user_id = session::require_session(&headers, pool).await?;
    if form.password.len() < 8 || form.password != form.confirm_password {
        return Err(AppError::InvalidRequest(
            "password must be at least 8 characters and match confirmation".to_string(),
        ));
    }
    if !db::verify_admin_password(pool, user_id, &form.current_password).await? {
        return Err(AppError::Unauthorized(
            "current password is invalid".to_string(),
        ));
    }
    db::update_admin_password(pool, user_id, &form.password).await?;
    let mut response = http::see_other("/login");
    http::set_cookie(&mut response, http::clear_session_cookie());
    Ok(response)
}
