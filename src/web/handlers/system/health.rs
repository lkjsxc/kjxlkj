//! Health endpoint handler

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Health check endpoint
pub async fn healthz() -> Response {
    (StatusCode::OK, [("Content-Type", "text/plain")], "ok").into_response()
}
