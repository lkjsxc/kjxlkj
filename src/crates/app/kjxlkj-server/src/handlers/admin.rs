use crate::app_state::AppState;
use crate::authn::require_identity;
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repos;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{ensure_automation_manage, ensure_workspace_member_read};
use serde::Deserialize;
use serde_json::json;
use std::fmt::Write as _;
use std::str::FromStr;
use std::time::SystemTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ExportMarkdownRequest {
    workspace_id: Uuid,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/admin/export/markdown", web::post().to(export_markdown))
        .route("/admin/export/{job_id}", web::get().to(get_export_job))
        .route("/admin/backup/sql", web::post().to(backup_sql));
}

async fn export_markdown(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<ExportMarkdownRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;

    let workspace_role = actor_workspace_role(&state, body.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let queued = repos::admin_jobs::create_job(
        &state.pool,
        identity.user_id,
        Some(body.workspace_id),
        "export_markdown",
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    let _ = repos::admin_jobs::mark_running(&state.pool, queued.id).await;
    let _ = repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        Some(body.workspace_id),
        "export_markdown_job_started",
        json!({ "job_id": queued.id }),
    )
    .await;

    let result = build_markdown_export_artifact(&state, queued.id, body.workspace_id).await;
    let final_job = match result {
        Ok(path) => {
            let succeeded = repos::admin_jobs::mark_succeeded(&state.pool, queued.id, &path)
                .await
                .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
                .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "JOB_NOT_FOUND", "job not found"))?;

            let _ = repos::audit::emit_security_event(
                &state.pool,
                &request_id,
                Some(identity.user_id),
                Some(body.workspace_id),
                "export_markdown_job_succeeded",
                json!({ "job_id": queued.id, "artifact_path": path }),
            )
            .await;

            succeeded
        }
        Err(error_message) => {
            let failed = repos::admin_jobs::mark_failed(
                &state.pool,
                queued.id,
                "EXPORT_FAILED",
                &error_message,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
            .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "JOB_NOT_FOUND", "job not found"))?;

            let _ = repos::audit::emit_security_event(
                &state.pool,
                &request_id,
                Some(identity.user_id),
                Some(body.workspace_id),
                "export_markdown_job_failed",
                json!({ "job_id": queued.id, "error": error_message }),
            )
            .await;

            failed
        }
    };

    Ok(HttpResponse::Accepted().json(json!({
        "job": job_json(final_job),
        "request_id": request_id,
    })))
}

async fn get_export_job(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let job_id = path.into_inner();

    let job = repos::admin_jobs::get_job(&state.pool, job_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "JOB_NOT_FOUND", "job not found"))?;

    if let Some(workspace_id) = job.workspace_id {
        let workspace_role = actor_workspace_role(&state, workspace_id, identity.user_id).await?;
        ensure_workspace_member_read(workspace_role)
            .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;
    } else if !matches!(identity.role, Role::Owner | Role::Admin) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN",
            "forbidden",
        ));
    }

    Ok(HttpResponse::Ok().json(json!({
        "job": job_json(job),
        "request_id": request_id,
    })))
}

async fn backup_sql(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;

    if !matches!(identity.role, Role::Owner | Role::Admin) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN",
            "forbidden",
        ));
    }

    let queued = repos::admin_jobs::create_job(&state.pool, identity.user_id, None, "backup_sql")
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    let _ = repos::admin_jobs::mark_running(&state.pool, queued.id).await;
    let _ = repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        None,
        "backup_sql_job_started",
        json!({ "job_id": queued.id }),
    )
    .await;

    let result = build_sql_backup_artifact(&state, queued.id).await;
    let final_job = match result {
        Ok(path) => {
            let succeeded = repos::admin_jobs::mark_succeeded(&state.pool, queued.id, &path)
                .await
                .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
                .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "JOB_NOT_FOUND", "job not found"))?;

            let _ = repos::audit::emit_security_event(
                &state.pool,
                &request_id,
                Some(identity.user_id),
                None,
                "backup_sql_job_succeeded",
                json!({ "job_id": queued.id, "artifact_path": path }),
            )
            .await;

            succeeded
        }
        Err(error_message) => {
            let failed = repos::admin_jobs::mark_failed(
                &state.pool,
                queued.id,
                "BACKUP_FAILED",
                &error_message,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
            .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "JOB_NOT_FOUND", "job not found"))?;

            let _ = repos::audit::emit_security_event(
                &state.pool,
                &request_id,
                Some(identity.user_id),
                None,
                "backup_sql_job_failed",
                json!({ "job_id": queued.id, "error": error_message }),
            )
            .await;

            failed
        }
    };

    Ok(HttpResponse::Accepted().json(json!({
        "job": job_json(final_job),
        "request_id": request_id,
    })))
}

async fn build_markdown_export_artifact(
    state: &AppState,
    job_id: Uuid,
    workspace_id: Uuid,
) -> Result<String, String> {
    let notes = repos::notes::list_notes(&state.pool, workspace_id, false)
        .await
        .map_err(|_| "failed to list workspace notes".to_owned())?;

    let mut content = String::new();
    writeln!(&mut content, "# Workspace Export").map_err(|_| "failed to build markdown".to_owned())?;
    writeln!(&mut content, "").map_err(|_| "failed to build markdown".to_owned())?;

    for stream in notes {
        let projection = repos::notes::get_note(&state.pool, stream.id)
            .await
            .map_err(|_| "failed to fetch note projection".to_owned())?
            .map(|(_, projection)| projection)
            .ok_or_else(|| "note projection missing".to_owned())?;

        writeln!(&mut content, "## {}", projection.title).map_err(|_| "failed to build markdown".to_owned())?;
        writeln!(&mut content, "").map_err(|_| "failed to build markdown".to_owned())?;
        writeln!(&mut content, "{}", projection.markdown).map_err(|_| "failed to build markdown".to_owned())?;
        writeln!(&mut content, "").map_err(|_| "failed to build markdown".to_owned())?;
    }

    std::fs::create_dir_all("/tmp/kjxlkj/jobs")
        .map_err(|_| "failed to create jobs directory".to_owned())?;

    let path = format!("/tmp/kjxlkj/jobs/export-{}.md", job_id);
    std::fs::write(&path, content).map_err(|_| "failed to write markdown export".to_owned())?;
    Ok(path)
}

async fn build_sql_backup_artifact(state: &AppState, job_id: Uuid) -> Result<String, String> {
    let database_name = sqlx::query_scalar::<_, String>("SELECT current_database()")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| "failed to read database name".to_owned())?;

    let table_names = sqlx::query_scalar::<_, String>(
        "SELECT table_name
         FROM information_schema.tables
         WHERE table_schema = 'public'
         ORDER BY table_name ASC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| "failed to enumerate tables".to_owned())?;

    let mut content = String::new();
    writeln!(&mut content, "-- kjxlkj deterministic SQL backup summary")
        .map_err(|_| "failed to build backup content".to_owned())?;
    writeln!(&mut content, "-- generated_at_utc={:?}", SystemTime::now())
        .map_err(|_| "failed to build backup content".to_owned())?;
    writeln!(&mut content, "-- database={}", database_name)
        .map_err(|_| "failed to build backup content".to_owned())?;
    writeln!(&mut content, "").map_err(|_| "failed to build backup content".to_owned())?;

    for table_name in table_names {
        let count_query = format!("SELECT COUNT(*)::BIGINT FROM \"{}\"", table_name);
        let row_count: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| format!("failed to count table rows for {}", table_name))?;
        writeln!(
            &mut content,
            "INSERT INTO backup_table_counts (table_name, row_count) VALUES ('{}', {});",
            table_name,
            row_count
        )
        .map_err(|_| "failed to build backup content".to_owned())?;
    }

    std::fs::create_dir_all("/tmp/kjxlkj/jobs")
        .map_err(|_| "failed to create jobs directory".to_owned())?;

    let path = format!("/tmp/kjxlkj/jobs/backup-{}.sql", job_id);
    std::fs::write(&path, content).map_err(|_| "failed to write sql backup artifact".to_owned())?;
    Ok(path)
}

fn job_json(job: kjxlkj_db::models::DbAdminJob) -> serde_json::Value {
    json!({
        "id": job.id,
        "requested_by": job.requested_by,
        "workspace_id": job.workspace_id,
        "job_type": job.job_type,
        "status": job.status,
        "artifact_path": job.artifact_path,
        "error_code": job.error_code,
        "error_detail": job.error_detail,
        "started_at": job.started_at.and_then(|value| value.format(&Rfc3339).ok()),
        "finished_at": job.finished_at.and_then(|value| value.format(&Rfc3339).ok()),
        "created_at": job.created_at.format(&Rfc3339).unwrap_or_else(|_| job.created_at.to_string()),
    })
}

async fn actor_workspace_role(
    state: &AppState,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Role, ApiError> {
    let role_text = repos::workspaces::actor_workspace_role(&state.pool, workspace_id, user_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    Role::from_str(&role_text)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "invalid role data"))
}
