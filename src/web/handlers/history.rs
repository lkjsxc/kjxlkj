//! Note history HTML handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/{id}/history")]
pub async fn history_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let id = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let Some(record) = accessible_record(&pool, &id, is_admin).await? else {
        return Ok(not_found());
    };
    let revisions = db::get_record_revisions(&pool, &id).await?;
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    let history = view::visible_history(&record, &revisions, is_admin);
    Ok(html(templates::history_page(
        &record,
        &chrome.with_history(history),
        is_admin,
    )))
}

#[get("/{id}/history/{revision_number}")]
pub async fn revision_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<(String, i32)>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let (id, revision_number) = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let Some(record) = accessible_record(&pool, &id, is_admin).await? else {
        return Ok(not_found());
    };
    let Some(revision) = db::get_record_revision(&pool, &id, revision_number).await? else {
        return Ok(not_found());
    };
    if revision.is_private && !is_admin {
        return Ok(not_found());
    }
    let revisions = db::get_record_revisions(&pool, &id).await?;
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    let history = view::visible_history(&record, &revisions, is_admin);
    Ok(html(templates::revision_page(
        &record,
        &chrome.with_history(history),
        &revision,
        is_admin,
    )))
}

async fn accessible_record(
    pool: &DbPool,
    id: &str,
    is_admin: bool,
) -> Result<Option<db::Record>, AppError> {
    let record = db::get_record(pool, id).await?;
    Ok(record.filter(|record| is_admin || !record.is_private))
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
