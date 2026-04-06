//! Login handlers

use crate::config::Config;
use crate::error::AppError;
use crate::web::db::DbPool;
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

/// Login form data
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Login page GET handler
#[get("/login")]
pub async fn login_page(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if session::check_session(&req, &pool).await? {
        return Ok(see_other("/admin"));
    }
    let settings = crate::web::db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&config, &settings);
    Ok(html(templates::login_page(&site, None)))
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
            let settings = crate::web::db::get_settings(&pool).await?;
            let timeout = i32::try_from(settings.session_timeout_minutes)
                .map_err(|_| AppError::StorageError("invalid session timeout".to_string()))?;
            let session_id = crate::web::db::create_session(&pool, id, timeout).await?;

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
            templates::login_page(
                &SiteContext::from_settings(&config, &crate::web::db::get_settings(&pool).await?),
                Some("Invalid username or password"),
            ),
        )),
    }
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
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
