//! Admin and home page handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

/// Admin dashboard handler
#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req).await
}

/// Admin dashboard with trailing slash
#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect("/login"));
    }
    let records = db::list_records(&pool, true, 100).await?;
    let entries: Vec<_> = records.iter().map(view::index_item).collect();
    Ok(html(templates::admin_page(&entries)))
}

/// Home/landing page handler
#[get("/")]
pub async fn home(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let is_admin = session::check_session(&req, &pool).await?;
    let records = db::list_records(&pool, false, 100).await?;
    let entries: Vec<_> = records.iter().map(view::index_item).collect();
    Ok(html(templates::home_page(&entries, is_admin)))
}

/// Note viewing page handler
#[get("/{slug}")]
pub async fn note_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let slug = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let record = match db::get_record(&pool, &slug).await? {
        Some(r) => r,
        None => return Ok(html(templates::not_found_page())),
    };
    if record.is_private && !is_admin {
        return Ok(html(templates::not_found_page()));
    }
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    Ok(html(templates::note_page(&record, &chrome, is_admin)))
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
