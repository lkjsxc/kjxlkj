use std::{path::PathBuf, process::Stdio};

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use tokio::{fs, process::Command};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{auth_session, enforce_csrf},
    error::AppError,
    models::JobStatus,
};

pub async fn export_markdown(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let job = create_job(&state, "markdown_export").await?;

    let pool = state.pool.clone();
    let export_dir = state.config.export_dir.clone();
    tokio::spawn(async move {
        let _ = run_export_job(pool, job.id, export_dir).await;
    });

    Ok(HttpResponse::Accepted().json(job))
}

pub async fn export_job_status(
    state: web::Data<AppState>,
    job_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let row = sqlx::query_as::<_, JobStatus>(
        "select id, kind, status, artifact_path, error, created_at, updated_at from jobs where id = $1",
    )
    .bind(job_id.into_inner())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("job not found".to_string()))?;
    Ok(HttpResponse::Ok().json(row))
}

pub async fn backup_sql(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let job = create_job(&state, "sql_backup").await?;

    let pool = state.pool.clone();
    let backup_dir = state.config.backup_dir.clone();
    let db_url = state.config.database_url.clone();
    tokio::spawn(async move {
        let _ = run_backup_job(pool, job.id, backup_dir, db_url).await;
    });

    Ok(HttpResponse::Accepted().json(job))
}

async fn create_job(state: &AppState, kind: &str) -> Result<JobStatus, AppError> {
    let id = Uuid::now_v7();
    sqlx::query("insert into jobs (id, kind, status) values ($1, $2, 'queued')")
        .bind(id)
        .bind(kind)
        .execute(&state.pool)
        .await?;
    let row = sqlx::query_as::<_, JobStatus>(
        "select id, kind, status, artifact_path, error, created_at, updated_at from jobs where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(row)
}

async fn run_export_job(
    pool: sqlx::PgPool,
    job_id: Uuid,
    export_dir: String,
) -> Result<(), AppError> {
    set_job_status(&pool, job_id, "running", None, None).await?;
    let result: Result<String, anyhow::Error> = async {
        fs::create_dir_all(&export_dir).await?;
        let stamp = Utc::now().format("%Y%m%d%H%M%S");
        let dir = PathBuf::from(export_dir).join(format!("export-{job_id}-{stamp}"));
        fs::create_dir_all(&dir).await?;

        let rows = sqlx::query_as::<_, (Uuid, String, String)>(
            "select note_id, title, markdown from note_projections order by updated_at desc",
        )
        .fetch_all(&pool)
        .await?;
        for (note_id, title, markdown) in rows {
            let safe = title.replace('/', "-").replace(' ', "_");
            let path = dir.join(format!("{}-{}.md", safe, note_id));
            fs::write(path, markdown).await?;
        }
        Ok(dir.to_string_lossy().to_string())
    }
    .await;

    match result {
        Ok(path) => set_job_status(&pool, job_id, "succeeded", Some(path), None).await,
        Err(err) => set_job_status(&pool, job_id, "failed", None, Some(err.to_string())).await,
    }
}

async fn run_backup_job(
    pool: sqlx::PgPool,
    job_id: Uuid,
    backup_dir: String,
    db_url: String,
) -> Result<(), AppError> {
    set_job_status(&pool, job_id, "running", None, None).await?;
    let result: Result<String, anyhow::Error> = async {
        fs::create_dir_all(&backup_dir).await?;
        let path = PathBuf::from(backup_dir).join(format!("backup-{job_id}.sql"));
        let status = Command::new("pg_dump")
            .arg(db_url)
            .arg("-f")
            .arg(path.as_os_str())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .status()
            .await?;
        if !status.success() {
            anyhow::bail!("pg_dump failed with status {status}");
        }
        Ok(path.to_string_lossy().to_string())
    }
    .await;

    match result {
        Ok(path) => set_job_status(&pool, job_id, "succeeded", Some(path), None).await,
        Err(err) => set_job_status(&pool, job_id, "failed", None, Some(err.to_string())).await,
    }
}

async fn set_job_status(
    pool: &sqlx::PgPool,
    id: Uuid,
    status: &str,
    artifact_path: Option<String>,
    error: Option<String>,
) -> Result<(), AppError> {
    sqlx::query("update jobs set status = $2, artifact_path = $3, error = $4, updated_at = now() where id = $1")
        .bind(id)
        .bind(status)
        .bind(artifact_path)
        .bind(error)
        .execute(pool)
        .await?;
    Ok(())
}
