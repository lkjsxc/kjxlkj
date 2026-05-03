//! Resource history JSON and resource navigation handlers

use crate::core::validate_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool, ListDirection};
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryParams {
    pub direction: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct NavResponse {
    pub id: Option<String>,
}

pub async fn history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    if db::get_resource(pool, &id).await?.is_none() {
        return Err(AppError::NotFound(format!("resource '{id}' not found")));
    }
    let settings = db::get_settings(pool).await?;
    let page = db::list_resource_snapshots(
        pool,
        &id,
        true,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    Ok(http::json_status(StatusCode::OK, page))
}

pub async fn history_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, id)): Path<(String, String)>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    history(State(state), headers, Path(id), Query(params)).await
}

pub async fn api_history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(reference): Path<String>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    let resource = db::get_resource_by_ref(pool, &reference)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("resource '{reference}' not found")))?;
    let settings = db::get_settings(pool).await?;
    let page = db::list_resource_snapshots(
        pool,
        &resource.id,
        true,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    Ok(http::json_status(StatusCode::OK, page))
}

pub async fn api_history_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, reference)): Path<(String, String)>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    api_history(State(state), headers, Path(reference), Query(params)).await
}

pub async fn previous(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    nav_response(&state.pool, &headers, id, true).await
}

pub async fn previous_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    previous(State(state), headers, Path(id)).await
}

pub async fn next(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    nav_response(&state.pool, &headers, id, false).await
}

pub async fn next_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    next(State(state), headers, Path(id)).await
}

async fn nav_response(
    pool: &DbPool,
    headers: &HeaderMap,
    id: String,
    older: bool,
) -> Result<Response, AppError> {
    validate_id(&id)?;
    let is_admin = session::check_session(headers, pool).await?;
    let resource = db::get_resource(pool, &id).await?;
    match resource {
        Some(resource) if is_admin || !resource.is_private => {
            let neighbor = if older {
                db::get_previous_resource(pool, &id, is_admin).await?
            } else {
                db::get_next_resource(pool, &id, is_admin).await?
            };
            Ok(http::json_status(
                StatusCode::OK,
                NavResponse {
                    id: neighbor.map(|note| note.id),
                },
            ))
        }
        _ => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}
