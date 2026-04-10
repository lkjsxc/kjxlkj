//! Admin settings handler

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::handlers::settings_input::{validate_settings_form, SettingsForm};
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PasswordForm {
    pub password: String,
    pub confirm_password: String,
}

#[get("/admin/settings")]
pub async fn settings_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect(&session::login_url(&req)));
    }
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(html(templates::settings_page(&settings, &site)))
}

#[post("/admin/settings")]
pub async fn settings_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<SettingsForm>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let current = db::get_settings(&pool).await?;
    db::update_settings(&pool, &validate_settings_form(&form, &current)?).await?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/settings"))
        .finish())
}

#[post("/admin/password")]
pub async fn password_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<PasswordForm>,
) -> Result<HttpResponse, AppError> {
    let user_id = session::require_session(&req, &pool).await?;
    if form.password.len() < 8 || form.password != form.confirm_password {
        return Err(AppError::InvalidRequest(
            "password must be at least 8 characters and match confirmation".to_string(),
        ));
    }
    db::update_admin_password(&pool, user_id, &form.password).await?;
    let clear_cookie = Cookie::build("session_id", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();
    Ok(HttpResponse::SeeOther()
        .cookie(clear_cookie)
        .append_header(("Location", "/login"))
        .finish())
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
