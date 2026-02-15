//! Admin export/backup handlers per /docs/spec/api/http.md.

use crate::dto::ApiError;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// POST /api/admin/export/markdown — launch markdown export job.
pub async fn export_markdown(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let role = kjxlkj_rbac::parse_role(&ctx.role)
        .unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_manage_workspace(role) {
        return middleware::forbidden();
    }
    let job_id = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::automation::create_job(
        pool.get_ref(),
        job_id,
        "export_markdown",
        None,
    )
    .await
    {
        Ok(()) => HttpResponse::Accepted()
            .json(serde_json::json!({"job_id": job_id, "status": "queued"})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/admin/export/{job_id} — export job status.
pub async fn export_status(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    let job_id = path.into_inner();
    match kjxlkj_db::repo::automation::get_job(pool.get_ref(), job_id).await {
        Ok(Some(j)) => HttpResponse::Ok().json(serde_json::json!({
            "job_id": j.id, "job_type": j.job_type, "status": j.status,
            "result_json": j.result_json
        })),
        Ok(None) => middleware::not_found("job"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/admin/backup/sql — launch SQL backup job.
pub async fn backup_sql(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let role = kjxlkj_rbac::parse_role(&ctx.role)
        .unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_manage_workspace(role) {
        return middleware::forbidden();
    }
    let job_id = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::automation::create_job(
        pool.get_ref(),
        job_id,
        "backup_sql",
        None,
    )
    .await
    {
        Ok(()) => HttpResponse::Accepted()
            .json(serde_json::json!({"job_id": job_id, "status": "queued"})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
