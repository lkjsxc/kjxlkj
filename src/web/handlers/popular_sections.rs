//! Popular-note fragment handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, PopularWindow};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PopularSectionPath {
    surface: String,
    window: String,
}

#[get("/_/popular-resources/{surface}/{window}")]
pub async fn popular_resources_section(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<PopularSectionPath>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(not_found());
    }
    let path = path.into_inner();
    let Some(window) = PopularWindow::parse(&path.window) else {
        return Ok(not_found());
    };
    match path.surface.as_str() {
        "home" => render_home_fragment(pool, req, window).await,
        "admin" => render_admin_fragment(pool, req, window).await,
        _ => Ok(not_found()),
    }
}

async fn render_home_fragment(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    window: PopularWindow,
) -> Result<HttpResponse, AppError> {
    let is_admin = session::check_session(&req, &pool).await?;
    let settings = db::get_settings(&pool).await?;
    if !settings.home_popular_visible {
        return Ok(not_found());
    }
    let items = popular_items(
        pool.get_ref(),
        is_admin,
        settings.home_popular_limit,
        window,
    )
    .await?;
    Ok(html(templates::home_popular_section(&items, window)))
}

async fn render_admin_fragment(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    window: PopularWindow,
) -> Result<HttpResponse, AppError> {
    if !session::check_session(&req, &pool).await? {
        return Ok(unauthorized());
    }
    let limit = db::get_settings(&pool).await?.home_popular_limit;
    let items = popular_items(pool.get_ref(), true, limit, window).await?;
    Ok(html(templates::admin_popular_section(&items, window)))
}

async fn popular_items(
    pool: &DbPool,
    is_admin: bool,
    limit: i64,
    window: PopularWindow,
) -> Result<Vec<templates::IndexItem>, AppError> {
    Ok(db::list_popular_resources(pool, is_admin, limit, window)
        .await?
        .iter()
        .map(|resource| view::popular_index_item(resource, is_admin, window))
        .collect())
}

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

fn unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().finish()
}
