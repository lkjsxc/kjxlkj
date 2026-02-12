use actix_web::{web, HttpResponse};

use crate::{app_state::AppState, error::AppError};

pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

pub async fn readyz(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let _: i64 = sqlx::query_scalar("select 1::bigint")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ready"})))
}
