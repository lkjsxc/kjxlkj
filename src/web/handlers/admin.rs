//! Admin dashboard handlers

use crate::error::AppError;
use crate::web::db::{self, AppSettings, DbPool, ListRequest};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BrowseParams {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<BrowseParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params.into_inner()).await
}

#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<BrowseParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params.into_inner()).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: BrowseParams,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect("/login"));
    }
    let settings = db::get_settings(&pool).await?;
    let page = db::list_records(&pool, &list_request(params, &settings)).await?;
    let recent = db::list_recent_records(&pool, true, settings.home_recent_limit).await?;
    let favorites = db::list_favorite_records(&pool, true, settings.home_favorite_limit).await?;
    let stats = db::get_note_stats(&pool, true).await?;
    Ok(html(templates::admin_page(
        &stats,
        &settings,
        &recent.iter().map(|record| view::index_item(record, true)).collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|record| view::index_item(record, true))
            .collect::<Vec<_>>(),
        &page.records
            .iter()
            .map(|record| view::index_item(record, true))
            .collect::<Vec<_>>(),
        page.next_cursor.as_deref(),
    )))
}

fn list_request(params: BrowseParams, settings: &AppSettings) -> ListRequest {
    ListRequest {
        include_private: true,
        limit: params.limit.unwrap_or(settings.search_results_per_page),
        query: None,
        cursor: params.cursor,
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
