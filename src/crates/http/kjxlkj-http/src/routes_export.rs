/// Export and backup API handlers per /docs/spec/domain/export.md
///
/// POST /admin/export               — create export job
/// GET  /admin/export/:job_id       — get job status
/// GET  /admin/export               — list jobs for workspace
///
/// Jobs are created as "queued" and simulate async processing.
/// In production, a background task would pick up queued jobs.
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use kjxlkj_domain::export::{ExportJob, ExportJobKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;
use kjxlkj_db::mem_export_repo::ExportRepo;

/// Request body for POST /admin/export
#[derive(Debug, Deserialize)]
pub struct CreateExportRequest {
    pub workspace_id: Uuid,
    pub kind: ExportJobKind,
}

/// Query params for GET /admin/export
#[derive(Debug, Deserialize)]
pub struct ListExportQuery {
    pub workspace_id: Uuid,
}

/// Response envelope for export job.
#[derive(Debug, Serialize)]
pub struct ExportJobResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub kind: ExportJobKind,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub artifact_url: Option<String>,
    pub error_message: Option<String>,
}

impl From<ExportJob> for ExportJobResponse {
    fn from(j: ExportJob) -> Self {
        Self {
            id: j.id,
            workspace_id: j.workspace_id,
            kind: j.kind,
            status: j.status.as_str().to_string(),
            created_at: j.created_at.to_rfc3339(),
            updated_at: j.updated_at.to_rfc3339(),
            artifact_url: j.artifact_url,
            error_message: j.error_message,
        }
    }
}

/// POST /admin/export — create new export job
pub async fn create_export(
    State(state): State<AppState>,
    Json(req): Json<CreateExportRequest>,
) -> Result<(StatusCode, Json<ExportJobResponse>), StatusCode> {
    let job = ExportJob::new(req.workspace_id, req.kind);
    state
        .export_repo
        .create_job(&job)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(ExportJobResponse::from(job))))
}

/// GET /admin/export/:job_id — get export job status
pub async fn get_export(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<ExportJobResponse>, StatusCode> {
    let job = state
        .export_repo
        .get_job(job_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(ExportJobResponse::from(job)))
}

/// GET /admin/export — list export jobs for a workspace
pub async fn list_exports(
    State(state): State<AppState>,
    Query(q): Query<ListExportQuery>,
) -> Result<Json<Vec<ExportJobResponse>>, StatusCode> {
    let jobs = state
        .export_repo
        .list_jobs(q.workspace_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(jobs.into_iter().map(ExportJobResponse::from).collect()))
}
