//! Favorite ordering handler

use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use axum::extract::{Json, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FavoriteOrderInput {
    pub ids: Vec<String>,
}

pub async fn reorder(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<FavoriteOrderInput>,
) -> Result<Response, AppError> {
    session::require_session(&headers, &state.pool).await?;
    db::reorder_favorites(&state.pool, &body.ids).await?;
    Ok(http::empty(StatusCode::NO_CONTENT))
}
