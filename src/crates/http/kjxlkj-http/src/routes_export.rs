//! Export and backup job routes.
//! Per /docs/spec/api/http.md and /docs/spec/domain/export.md.
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_export_job;
use kjxlkj_domain::error::DomainError;
use uuid::Uuid;

use crate::dto::{ExportJobResponse, LaunchExportRequest};
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

fn job_to_response(j: &repo_export_job::ExportJobRow) -> ExportJobResponse {
    ExportJobResponse {
        id: j.id,
        job_type: j.job_type.clone(),
        status: j.status.clone(),
        artifact_path: j.artifact_path.clone(),
    }
}

/// POST /api/admin/export/markdown
pub async fn launch_markdown_export(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    _body: web::Json<LaunchExportRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let id = Uuid::now_v7();
    match repo_export_job::create_job(
        pool.get_ref(), id, "markdown", identity.user_id.0,
    ).await {
        Ok(j) => HttpResponse::Accepted().json(job_to_response(&j)),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// POST /api/admin/backup/sql
pub async fn launch_sql_backup(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let id = Uuid::now_v7();
    match repo_export_job::create_job(
        pool.get_ref(), id, "sql_backup", identity.user_id.0,
    ).await {
        Ok(j) => HttpResponse::Accepted().json(job_to_response(&j)),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /api/admin/export/{job_id}
pub async fn get_export_job(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let id = path.into_inner();
    match repo_export_job::find_job(pool.get_ref(), id).await {
        Ok(Some(j)) => HttpResponse::Ok().json(job_to_response(&j)),
        Ok(None) => domain_error_response(
            DomainError::NotFound("job".into()), &rid,
        ),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
