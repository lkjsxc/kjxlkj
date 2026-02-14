// Admin operations per /docs/spec/api/http.md
// POST /admin/export/markdown, GET /admin/export/{job_id}, POST /admin/backup/sql
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::{require_role, AuthSession};
use kjxlkj_domain::types::Role;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::ErrorBody;

/// POST /api/admin/export/markdown — launch markdown export job
pub async fn launch_export(
    pool: web::Data<PgPool>,
    auth: AuthSession,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(resp) = require_role(&auth, Role::Admin) {
        return resp.into();
    }

    let job_id = Uuid::now_v7();
    match sqlx::query(
        "INSERT INTO jobs (id, job_type, status, created_by) VALUES ($1, 'markdown_export', 'queued', $2)",
    )
    .bind(job_id)
    .bind(auth.user.id)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Accepted().json(serde_json::json!({
            "job_id": job_id,
            "status": "queued"
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        }),
    }
}

/// GET /api/admin/export/{job_id} — export job status/artifact
pub async fn export_status(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _auth: AuthSession,
) -> HttpResponse {
    let job_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();

    let row: Option<(String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT status, artifact_path, error_detail FROM jobs WHERE id = $1",
    )
    .bind(job_id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    match row {
        Some((status, artifact, error)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "job_id": job_id,
                "status": status,
                "artifact_path": artifact,
                "error_detail": error,
            }))
        }
        None => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(),
            message: "Job not found".into(),
            details: None,
            request_id: rid,
        }),
    }
}

/// POST /api/admin/backup/sql — launch SQL backup job
pub async fn launch_backup(
    pool: web::Data<PgPool>,
    auth: AuthSession,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(resp) = require_role(&auth, Role::Admin) {
        return resp.into();
    }

    let job_id = Uuid::now_v7();
    match sqlx::query(
        "INSERT INTO jobs (id, job_type, status, created_by) VALUES ($1, 'sql_backup', 'queued', $2)",
    )
    .bind(job_id)
    .bind(auth.user.id)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Accepted().json(serde_json::json!({
            "job_id": job_id,
            "status": "queued"
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        }),
    }
}
