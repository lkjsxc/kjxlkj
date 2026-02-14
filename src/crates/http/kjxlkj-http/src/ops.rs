// Ops handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

/// GET /api/healthz — liveness check
pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// GET /api/readyz — readiness check (DB + migrations)
pub async fn readyz(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "ready"})),
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "reason": e.to_string()
        })),
    }
}
