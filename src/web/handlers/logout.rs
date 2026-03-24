//! Logout handler

use crate::error::AppError;
use crate::web::db::DbPool;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpRequest, HttpResponse};
use uuid::Uuid;

/// Logout POST handler
#[post("/logout")]
pub async fn logout(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    if let Some(cookie) = req.cookie("session_id") {
        if let Ok(session_id) = Uuid::parse_str(cookie.value()) {
            let _ = crate::web::db::delete_session(&pool, session_id).await;
        }
    }

    let clear_cookie = Cookie::build("session_id", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    Ok(HttpResponse::NoContent().cookie(clear_cookie).finish())
}
