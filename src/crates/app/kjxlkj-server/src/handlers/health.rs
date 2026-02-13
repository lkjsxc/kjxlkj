use crate::app_state::AppState;
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(healthz_api))
        .route("/readyz", web::get().to(readyz_api));
}

pub fn configure_root(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(healthz_root))
        .route("/readyz", web::get().to(readyz_root));
}

async fn healthz_api() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

async fn healthz_root() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

async fn readyz_api(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    readyz(state).await
}

async fn readyz_root(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    readyz(state).await
}

async fn readyz(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();

    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DB_UNAVAILABLE",
                "database unavailable",
            )
            .with_request_id(request_id.clone())
        })?;

    let migrations_present = kjxlkj_db::migrations::migration_table_exists(&state.pool)
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "MIGRATION_UNAVAILABLE",
                "migration table unavailable",
            )
            .with_request_id(request_id.clone())
        })?;

    if !migrations_present {
        return Err(ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "MIGRATION_UNAVAILABLE",
            "migration table unavailable",
        )
        .with_request_id(request_id));
    }

    Ok(HttpResponse::Ok().json(json!({ "status": "ready" })))
}
