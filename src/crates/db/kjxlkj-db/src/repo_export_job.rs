use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Per /docs/spec/domain/export.md: export job lifecycle.
/// States: queued → running → succeeded | failed.
#[derive(Debug, FromRow)]
pub struct ExportJobRow {
    pub id: Uuid,
    pub job_type: String,
    pub status: String,
    pub artifact_path: Option<String>,
    pub actor_id: Option<Uuid>,
}

pub async fn create_job(
    pool: &PgPool,
    id: Uuid,
    job_type: &str,
    actor_id: Uuid,
) -> Result<ExportJobRow, sqlx::Error> {
    sqlx::query_as::<_, ExportJobRow>(
        "INSERT INTO export_jobs (id, job_type, status, actor_id)
         VALUES ($1, $2, 'queued', $3)
         RETURNING id, job_type, status, artifact_path, actor_id",
    )
    .bind(id)
    .bind(job_type)
    .bind(actor_id)
    .fetch_one(pool)
    .await
}

pub async fn find_job(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ExportJobRow>, sqlx::Error> {
    sqlx::query_as::<_, ExportJobRow>(
        "SELECT id, job_type, status, artifact_path, actor_id
         FROM export_jobs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn start_job(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE export_jobs SET status = 'running', started_at = now() WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn complete_job(
    pool: &PgPool,
    id: Uuid,
    artifact_path: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE export_jobs
         SET status = 'succeeded', finished_at = now(), artifact_path = $2
         WHERE id = $1",
    )
    .bind(id)
    .bind(artifact_path)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fail_job(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE export_jobs SET status = 'failed', finished_at = now() WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
