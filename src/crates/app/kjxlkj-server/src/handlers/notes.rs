use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{auth_session, enforce_csrf},
    db_notes,
    error::AppError,
    models::{
        CreateNoteRequest, MetadataValueRequest, PatchNoteRequest, ReplaceTagsRequest,
        RollbackRequest,
    },
    ws::ServerEvent,
};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub include_deleted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn create_note(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let note = db_notes::create_note(
        &state.pool,
        session.user_id,
        &payload.title,
        &payload.markdown,
    )
    .await?;
    Ok(HttpResponse::Created().json(note))
}

pub async fn list_notes(
    state: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, AppError> {
    let rows = db_notes::list_notes(&state.pool, query.include_deleted.unwrap_or(false)).await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn get_note(
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let note = db_notes::get_note(&state.pool, note_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(note))
}

pub async fn patch_note(
    req: HttpRequest,
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
    payload: web::Json<PatchNoteRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let note_id = note_id.into_inner();
    let (version, seq, projection) = match db_notes::apply_note_patch(
        &state.pool,
        session.user_id,
        note_id,
        payload.base_version,
        &payload.patch_ops,
        &payload.idempotency_key,
    )
    .await
    {
        Ok(ok) => ok,
        Err(AppError::VersionConflict {
            expected_version,
            current_version,
        }) => {
            let latest = db_notes::get_note(&state.pool, note_id).await?;
            return Ok(HttpResponse::Conflict().json(json!({
                "code": "VERSION_CONFLICT",
                "message": "base version does not match current version",
                "details": {
                    "expected_version": expected_version,
                    "current_version": current_version,
                },
                "request_id": "n/a",
                "latest": latest,
            })));
        }
        Err(err) => return Err(err),
    };

    state
        .publish(
            note_id,
            ServerEvent::note_event(
                note_id,
                seq,
                version,
                "patch",
                serde_json::to_value(&projection).unwrap_or_default(),
            ),
        )
        .await;
    Ok(HttpResponse::Ok().json(projection))
}

pub async fn delete_note(
    req: HttpRequest,
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    db_notes::soft_delete_note(&state.pool, note_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn note_history(
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let rows = db_notes::note_history(&state.pool, note_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn rollback_note(
    req: HttpRequest,
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
    payload: web::Json<RollbackRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let projection = db_notes::rollback_note(
        &state.pool,
        session.user_id,
        note_id.into_inner(),
        payload.target_version,
    )
    .await?;
    Ok(HttpResponse::Ok().json(projection))
}

pub async fn put_metadata(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, String)>,
    payload: web::Json<MetadataValueRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let (note_id, key) = path.into_inner();
    let projection = db_notes::upsert_metadata(
        &state.pool,
        session.user_id,
        note_id,
        &key,
        payload.value.clone(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(projection))
}

pub async fn delete_metadata(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let (note_id, key) = path.into_inner();
    db_notes::delete_metadata(&state.pool, session.user_id, note_id, &key).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_tags(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let tags = db_notes::list_tags(&state.pool).await?;
    Ok(HttpResponse::Ok().json(tags))
}

pub async fn replace_tags(
    req: HttpRequest,
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
    payload: web::Json<ReplaceTagsRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let projection = db_notes::replace_tags(
        &state.pool,
        session.user_id,
        note_id.into_inner(),
        &payload.tags,
    )
    .await?;
    Ok(HttpResponse::Ok().json(projection))
}

pub async fn backlinks(
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let rows = db_notes::backlinks(&state.pool, note_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn search(
    state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let rows = db_notes::search(&state.pool, query.q.clone().unwrap_or_default().as_str()).await?;
    Ok(HttpResponse::Ok().json(rows))
}
