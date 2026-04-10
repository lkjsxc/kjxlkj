use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct ResetForm {
    pub token: String,
    pub password: String,
    pub confirm_password: String,
}

#[get("/reset-password")]
pub async fn reset_page(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    Ok(html(templates::password_reset_page(
        &site(&pool).await?,
        None,
    )))
}

#[post("/reset-password/request")]
pub async fn reset_request(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    if let Some(token) = db::issue_password_reset_token(&pool).await? {
        warn!(
            password_reset_token = %token,
            "password reset token issued; enter it on /reset-password within 15 minutes"
        );
    }
    Ok(html(templates::password_reset_page(
        &site(&pool).await?,
        Some("Reset token issued in the server console."),
    )))
}

#[post("/reset-password")]
pub async fn reset_submit(
    pool: web::Data<DbPool>,
    form: web::Form<ResetForm>,
) -> Result<HttpResponse, AppError> {
    if let Some(error) = password_error(&form) {
        return Ok(html_status(
            actix_web::http::StatusCode::BAD_REQUEST,
            templates::password_reset_page(&site(&pool).await?, Some(error)),
        ));
    }
    if db::reset_admin_password(&pool, &form.token, &form.password).await? {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish());
    }
    Ok(html_status(
        actix_web::http::StatusCode::BAD_REQUEST,
        templates::password_reset_page(&site(&pool).await?, Some("Reset token is invalid.")),
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

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

fn html_status(status: actix_web::http::StatusCode, body: String) -> HttpResponse {
    HttpResponse::build(status)
        .content_type("text/html; charset=utf-8")
        .body(body)
}
