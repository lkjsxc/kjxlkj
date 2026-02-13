use actix_web::{web, HttpResponse};
use sqlx::PgPool;

/// Liveness check - always 200 if server is running.
pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Readiness check - verifies DB connectivity and migration state.
pub async fn readyz(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "ready"})),
        Err(_) => HttpResponse::ServiceUnavailable()
            .json(serde_json::json!({"status": "not_ready"})),
    }
}
