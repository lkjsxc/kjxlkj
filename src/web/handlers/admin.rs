//! Public index, admin dashboard, and note handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, ListRequest};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ListParams {
    pub q: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<ListParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params.into_inner()).await
}

#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<ListParams>,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req, params.into_inner()).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: ListParams,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect("/login"));
    }
    let page = db::list_records(&pool, &list_request(params.clone(), true)).await?;
    let entries: Vec<_> = page
        .records
        .iter()
        .map(|record| view::index_item(record, true))
        .collect();
    Ok(html(templates::admin_page(
        &entries,
        page.next_cursor.as_deref(),
        params.q.as_deref(),
    )))
}

#[get("/")]
pub async fn home(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<ListParams>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let is_admin = session::check_session(&req, &pool).await?;
    let params = params.into_inner();
    let page = db::list_records(&pool, &list_request(params.clone(), false)).await?;
    let entries: Vec<_> = page
        .records
        .iter()
        .map(|record| view::index_item(record, false))
        .collect();
    Ok(html(templates::home_page(
        &entries,
        page.next_cursor.as_deref(),
        params.q.as_deref(),
        is_admin,
    )))
}

#[get("/{id}")]
pub async fn note_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let id = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let record = match db::get_record(&pool, &id).await? {
        Some(record) => record,
        None => return Ok(not_found()),
    };
    if record.is_private && !is_admin {
        return Ok(not_found());
    }
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    Ok(html(templates::note_page(&record, &chrome, is_admin)))
}

fn list_request(params: ListParams, include_private: bool) -> ListRequest {
    ListRequest {
        include_private,
        limit: params.limit.unwrap_or(50),
        query: params.q,
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

fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(templates::not_found_page())
}
