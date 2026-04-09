//! Search HTML handler

use crate::error::AppError;
use crate::web::db::{
    self, DbPool, ListDirection, ListKind, ListRequest, ListScope, ListSort, PopularWindow,
};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub direction: Option<String>,
    pub kind: Option<String>,
    pub sort: Option<String>,
    pub scope: Option<String>,
    pub popular_window: Option<String>,
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
    let site = SiteContext::from_settings(&settings);
    let params = params.into_inner();
    let query = params
        .q
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let limit = params.limit.unwrap_or(settings.search_results_per_page);
    let direction = ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref());
    let kind = ListKind::resolve(params.kind.as_deref());
    let scope = ListScope::resolve(params.scope.as_deref());
    let popular_window = PopularWindow::resolve(params.popular_window.as_deref());
    let sort = ListSort::resolve(params.sort.as_deref(), query.is_some(), &scope);
    let page = db::list_resources(
        &pool,
        &ListRequest {
            include_private: is_admin,
            limit,
            query: query.clone(),
            direction,
            kind: kind.clone(),
            scope: scope.clone(),
            sort: sort.clone(),
            popular_window,
            cursor: params.cursor,
        },
    )
    .await?;
    Ok(html(templates::search_page(templates::SearchView {
        notes: &page
            .resources
            .iter()
            .map(|resource| view::index_item(resource, is_admin))
            .collect::<Vec<_>>(),
        previous_cursor: page.previous_cursor.as_deref(),
        next_cursor: page.next_cursor.as_deref(),
        kind: kind.as_str(),
        query: query.as_deref(),
        limit,
        scope: scope.as_str(),
        sort: sort.as_str(),
        popular_window: popular_window.as_str(),
        is_admin,
        site: &site,
    })))
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
