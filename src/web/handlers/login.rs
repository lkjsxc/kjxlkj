//! Login handlers

use crate::config::Config;
use crate::error::AppError;
use crate::web::db::DbPool;
use crate::web::templates;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

/// Login form data
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Login page GET handler
#[get("/login")]
pub async fn login_page(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    if !crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    Ok(html(templates::login_page(None)))
}

/// Login form POST handler
#[post("/login")]
pub async fn login_submit(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    form: web::Form<LoginForm>,
) -> Result<HttpResponse, AppError> {
    if !crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }

    let user_id = crate::web::db::verify_credentials(&pool, &form.username, &form.password).await?;

    match user_id {
        Some(id) => {
            let session_id =
                crate::web::db::create_session(&pool, id, config.session_timeout_minutes).await?;

            let cookie = Cookie::build("session_id", session_id.to_string())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish();

            Ok(HttpResponse::SeeOther()
                .cookie(cookie)
                .append_header(("Location", "/admin"))
                .finish())
        }
        None => Ok(html_status(
            actix_web::http::StatusCode::UNAUTHORIZED,
            templates::login_page(Some("Invalid username or password")),
        )),
    }
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

fn html_status(status: actix_web::http::StatusCode, body: String) -> HttpResponse {
    HttpResponse::build(status)
        .content_type("text/html; charset=utf-8")
        .body(body)
}
