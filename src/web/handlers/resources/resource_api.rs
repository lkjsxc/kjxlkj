use crate::error::AppError;
use crate::web::db::{
    self, ListDirection, ListKind, ListRequest, ListScope, ListSort, ListedResource, PopularWindow,
};
use crate::web::handlers::http;
use crate::web::handlers::resource_payload::ResourcePayload;
use crate::web::handlers::search::SearchParams;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Serialize;

#[derive(Serialize)]
struct SearchResponse {
    resources: Vec<SearchItem>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize)]
struct SearchItem {
    resource: ResourcePayload,
    preview: String,
    popular_views: Option<i64>,
}

pub async fn search(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<SearchParams>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let is_admin = session::check_session(&headers, pool).await?;
    let settings = db::get_settings(pool).await?;
    let query = params
        .q
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let direction = ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref());
    let kind = ListKind::resolve(params.kind.as_deref());
    let scope = ListScope::resolve(params.scope.as_deref());
    let sort = ListSort::resolve(params.sort.as_deref(), query.is_some(), &scope);
    let page = db::list_resources(
        pool,
        &ListRequest {
            include_private: is_admin,
            limit: params.limit.unwrap_or(settings.search_results_per_page),
            query,
            direction,
            kind,
            scope,
            sort: sort.clone(),
            popular_window: sort.popular_window().unwrap_or(PopularWindow::Days30),
            cursor: params.cursor,
        },
    )
    .await?;
    Ok(http::json_status(
        StatusCode::OK,
        SearchResponse {
            resources: page
                .resources
                .into_iter()
                .map(SearchItem::from_row)
                .collect(),
            previous_cursor: page.previous_cursor,
            next_cursor: page.next_cursor,
        },
    ))
}

pub async fn fetch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(reference): Path<String>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let is_admin = session::check_session(&headers, pool).await?;
    match db::get_resource_by_ref(pool, &reference).await? {
        Some(resource) if is_admin || !resource.is_private => Ok(http::json_status(
            StatusCode::OK,
            ResourcePayload::from_resource(resource),
        )),
        _ => Err(AppError::NotFound(format!(
            "resource '{reference}' not found"
        ))),
    }
}

impl SearchItem {
    fn from_row(row: ListedResource) -> Self {
        Self {
            resource: ResourcePayload::from_resource(row.resource),
            preview: row.preview,
            popular_views: row.popular_views,
        }
    }
}
