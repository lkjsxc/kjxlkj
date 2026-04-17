//! Markdown preview handler

use crate::error::AppError;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::markdown;
use crate::web::routes::AppState;
use axum::extract::{Json, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PreviewInput {
    pub body: String,
    pub current_resource_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PreviewOutput {
    pub html: String,
}

pub async fn render_markdown_preview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<PreviewInput>,
) -> Result<Response, AppError> {
    session::require_session(&headers, &state.pool).await?;
    Ok(http::json_status(
        StatusCode::OK,
        PreviewOutput {
            html: markdown::render_markdown_page(
                &state.pool,
                &body.body,
                body.current_resource_id.as_deref(),
                true,
            )
            .await?,
        },
    ))
}
