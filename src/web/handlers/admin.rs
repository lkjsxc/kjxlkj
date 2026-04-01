//! Admin dashboard handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, PopularWindow};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AdminParams {
    pub popular_window: Option<String>,
}

#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<AdminParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params).await
}

#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<AdminParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<AdminParams>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect("/login"));
    }
    let settings = db::get_settings(&pool).await?;
    let window = PopularWindow::resolve(params.popular_window.as_deref());
    let popular =
        db::list_popular_records(&pool, true, settings.home_popular_limit, window).await?;
    let recent = db::list_recent_records(&pool, true, settings.home_recent_limit).await?;
    let favorites = db::list_favorite_records(&pool, true, settings.home_favorite_limit).await?;
    let stats = db::get_note_stats(&pool, true).await?;
    Ok(html(templates::admin_page(
        &stats,
        &settings,
        &popular
            .iter()
            .map(|record| view::popular_index_item(record, true, window))
            .collect::<Vec<_>>(),
        &recent
            .iter()
            .map(|record| view::index_item(record, true))
            .collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|record| view::index_item(record, true))
            .collect::<Vec<_>>(),
        window,
    )))
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
