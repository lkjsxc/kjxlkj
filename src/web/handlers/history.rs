//! Note history HTML handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, ListDirection};
use crate::web::handlers::record_history::HistoryParams;
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/{id}/history")]
pub async fn history_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    params: web::Query<HistoryParams>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let reference = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let Some(record) = accessible_record(&pool, &reference, is_admin).await? else {
        return Ok(not_found());
    };
    if record
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == record.id
    {
        return Ok(redirect(&with_query(
            &view::history_href(&record),
            req.query_string(),
        )));
    }
    let settings = db::get_settings(&pool).await?;
    let page = db::list_record_snapshots(
        &pool,
        &record.id,
        is_admin,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    Ok(html(templates::history_page(
        &record,
        &chrome,
        &view::history_links(&page.snapshots, params.cursor.is_none()),
        page.previous_cursor.as_deref(),
        page.next_cursor.as_deref(),
        params.limit.unwrap_or(settings.search_results_per_page),
        is_admin,
    )))
}

async fn accessible_record(
    pool: &DbPool,
    reference: &str,
    is_admin: bool,
) -> Result<Option<db::Record>, AppError> {
    let record = db::get_record_by_ref(pool, reference).await?;
    Ok(record.filter(|record| is_admin || !record.is_private))
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn with_query(path: &str, query: &str) -> String {
    if query.is_empty() {
        path.to_string()
    } else {
        format!("{path}?{query}")
    }
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
