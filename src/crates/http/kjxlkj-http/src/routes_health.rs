use actix_web::{web, HttpResponse};
use sqlx::PgPool;

/// GET /api/healthz - liveness per /docs/spec/api/http.md.
pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// GET /api/readyz - readiness (DB + migrations) per /docs/spec/api/http.md.
pub async fn readyz(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "ready"})),
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unavailable",
            "error": e.to_string()
        })),
    }
}
