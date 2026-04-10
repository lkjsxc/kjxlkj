use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::http;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct ResetForm {
    pub token: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn reset_page(State(state): State<AppState>) -> Result<Response, AppError> {
    Ok(http::html(templates::password_reset_page(
        &site(&state.pool).await?,
        None,
    )))
}

pub async fn reset_request(State(state): State<AppState>) -> Result<Response, AppError> {
    if let Some(token) = db::issue_password_reset_token(&state.pool).await? {
        warn!(
            password_reset_token = %token,
            "password reset token issued; enter it on /reset-password within 15 minutes"
        );
    }
    Ok(http::html(templates::password_reset_page(
        &site(&state.pool).await?,
        Some("Reset token issued in the server console."),
    )))
}

pub async fn reset_submit(
    State(state): State<AppState>,
    Form(form): Form<ResetForm>,
) -> Result<Response, AppError> {
    if let Some(error) = password_error(&form) {
        return Ok(http::html_status(
            StatusCode::BAD_REQUEST,
            templates::password_reset_page(&site(&state.pool).await?, Some(error)),
        ));
    }
    if db::reset_admin_password(&state.pool, &form.token, &form.password).await? {
        return Ok(http::see_other("/login"));
    }
    Ok(http::html_status(
        StatusCode::BAD_REQUEST,
        templates::password_reset_page(&site(&state.pool).await?, Some("Reset token is invalid.")),
    ))
}

fn password_error(form: &ResetForm) -> Option<&'static str> {
    if form.password.len() < 8 {
        Some("Password must be at least 8 characters.")
    } else if form.password != form.confirm_password {
        Some("Passwords do not match.")
    } else {
        None
    }
}

async fn site(pool: &DbPool) -> Result<SiteContext, AppError> {
    db::get_settings(pool)
        .await
        .map(|settings| SiteContext::from_settings(&settings))
}
