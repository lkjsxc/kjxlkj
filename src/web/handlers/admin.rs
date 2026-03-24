//! Admin page handler

use crate::error::AppError;
use crate::storage::{FilesystemStorage, Storage};
use crate::web::db::DbPool;
use crate::web::templates;
use actix_web::{get, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;

/// Admin page GET handler
#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    storage: web::Data<Arc<FilesystemStorage>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, storage, req).await
}

/// Admin page with trailing slash
#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    storage: web::Data<Arc<FilesystemStorage>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, storage, req).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    storage: web::Data<Arc<FilesystemStorage>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }

    let session_valid = check_session(&req, &pool).await?;
    if !session_valid {
        return Ok(redirect("/login"));
    }

    let records = storage.list().await?;
    Ok(html(templates::admin_page(&records)))
}

/// Home page handler
#[get("/")]
pub async fn home(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    if !crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }

    let session_valid = check_session(&req, &pool).await?;
    if session_valid {
        return Ok(redirect("/admin"));
    }

    Ok(html(templates::home_page()))
}

async fn check_session(req: &HttpRequest, pool: &DbPool) -> Result<bool, AppError> {
    let cookie = match req.cookie("session_id") {
        Some(c) => c,
        None => return Ok(false),
    };

    let session_id = match Uuid::parse_str(cookie.value()) {
        Ok(id) => id,
        Err(_) => return Ok(false),
    };

    let user_id = crate::web::db::validate_session(pool, session_id).await?;
    Ok(user_id.is_some())
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
