use crate::models::DbAdminJob;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_job(
    pool: &PgPool,
    requested_by: Uuid,
    workspace_id: Option<Uuid>,
    job_type: &str,
) -> Result<DbAdminJob, sqlx::Error> {
    sqlx::query_as::<_, DbAdminJob>(
        "INSERT INTO admin_jobs
         (id, requested_by, workspace_id, job_type, status)
         VALUES ($1, $2, $3, $4, 'queued')
         RETURNING id, requested_by, workspace_id, job_type, status,
                   artifact_path, error_code, error_detail, started_at,
                   finished_at, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(requested_by)
    .bind(workspace_id)
    .bind(job_type)
    .fetch_one(pool)
    .await
}

pub async fn get_job(pool: &PgPool, job_id: Uuid) -> Result<Option<DbAdminJob>, sqlx::Error> {
    sqlx::query_as::<_, DbAdminJob>(
        "SELECT id, requested_by, workspace_id, job_type, status,
                artifact_path, error_code, error_detail, started_at,
                finished_at, created_at
         FROM admin_jobs
         WHERE id = $1",
    )
    .bind(job_id)
    .fetch_optional(pool)
    .await
}

pub async fn mark_running(pool: &PgPool, job_id: Uuid) -> Result<Option<DbAdminJob>, sqlx::Error> {
    sqlx::query_as::<_, DbAdminJob>(
        "UPDATE admin_jobs
         SET status = 'running', started_at = NOW()
         WHERE id = $1 AND status = 'queued'
         RETURNING id, requested_by, workspace_id, job_type, status,
                   artifact_path, error_code, error_detail, started_at,
                   finished_at, created_at",
    )
    .bind(job_id)
    .fetch_optional(pool)
    .await
}

pub async fn mark_succeeded(
    pool: &PgPool,
    job_id: Uuid,
    artifact_path: &str,
) -> Result<Option<DbAdminJob>, sqlx::Error> {
    sqlx::query_as::<_, DbAdminJob>(
        "UPDATE admin_jobs
         SET status = 'succeeded', artifact_path = $2,
             error_code = NULL, error_detail = NULL, finished_at = NOW()
         WHERE id = $1 AND status IN ('queued', 'running')
         RETURNING id, requested_by, workspace_id, job_type, status,
                   artifact_path, error_code, error_detail, started_at,
                   finished_at, created_at",
    )
    .bind(job_id)
    .bind(artifact_path)
    .fetch_optional(pool)
    .await
}

pub async fn mark_failed(
    pool: &PgPool,
    job_id: Uuid,
    error_code: &str,
    error_detail: &str,
) -> Result<Option<DbAdminJob>, sqlx::Error> {
    sqlx::query_as::<_, DbAdminJob>(
        "UPDATE admin_jobs
         SET status = 'failed', error_code = $2, error_detail = $3, finished_at = NOW()
         WHERE id = $1 AND status IN ('queued', 'running')
         RETURNING id, requested_by, workspace_id, job_type, status,
                   artifact_path, error_code, error_detail, started_at,
                   finished_at, created_at",
    )
    .bind(job_id)
    .bind(error_code)
    .bind(error_detail)
    .fetch_optional(pool)
    .await
}
