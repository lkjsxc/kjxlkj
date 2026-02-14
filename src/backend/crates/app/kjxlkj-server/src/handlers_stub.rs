use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::auth::request_id;

pub async fn stub_ok(headers: HeaderMap) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "status": "ok", "request_id": request_id(&headers) })),
    )
}

pub async fn stub_accepted(headers: HeaderMap) -> impl IntoResponse {
    (
        StatusCode::ACCEPTED,
        Json(json!({ "status": "accepted", "request_id": request_id(&headers) })),
    )
}

pub async fn stub_no_content(headers: HeaderMap) -> impl IntoResponse {
    (StatusCode::NO_CONTENT, Json(json!({ "request_id": request_id(&headers) })))
}
