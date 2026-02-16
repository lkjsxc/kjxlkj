/// Health endpoint handlers per /docs/spec/architecture/deployment.md
///
/// GET /api/healthz — liveness
/// GET /api/readyz  — readiness
use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn healthz() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

pub async fn readyz() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}
