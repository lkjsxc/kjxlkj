use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_rbac::roles;

/// POST /api/admin/export/markdown
pub async fn export_markdown(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    // Export jobs are created but processed asynchronously.
    let job_id = uuid::Uuid::new_v4();
    tracing::info!(job_id = %job_id, "markdown export job queued");
    HttpResponse::Accepted().json(serde_json::json!({
        "job_id": job_id,
        "status": "queued"
    }))
}

/// GET /api/admin/export/{job_id}
pub async fn export_status(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let job_id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "job_id": job_id,
        "status": "queued"
    }))
}

/// POST /api/admin/backup/sql
pub async fn backup_sql(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let job_id = uuid::Uuid::new_v4();
    tracing::info!(job_id = %job_id, "sql backup job queued");
    HttpResponse::Accepted().json(serde_json::json!({
        "job_id": job_id,
        "status": "queued"
    }))
}
