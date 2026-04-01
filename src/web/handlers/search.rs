//! Search HTML handler

use crate::error::AppError;
use crate::web::db::{self, DbPool, ListDirection, ListRequest, ListSort};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub direction: Option<String>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[get("/search")]
pub async fn search_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    params: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let is_admin = session::check_session(&req, &pool).await?;
    let settings = db::get_settings(&pool).await?;
    let params = params.into_inner();
    let query = params
        .q
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let limit = params.limit.unwrap_or(settings.search_results_per_page);
    let direction = ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref());
    let sort = ListSort::resolve(params.sort.as_deref(), query.is_some());
    let page = db::list_records(
        &pool,
        &ListRequest {
            include_private: is_admin,
            limit,
            query: query.clone(),
            direction,
            sort: sort.clone(),
            cursor: params.cursor,
        },
    )
    .await?;
    Ok(html(templates::search_page(
        &page
            .records
            .iter()
            .map(|record| view::index_item(record, is_admin))
            .collect::<Vec<_>>(),
        page.previous_cursor.as_deref(),
        page.next_cursor.as_deref(),
        query.as_deref(),
        limit,
        sort.as_str(),
        is_admin,
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
