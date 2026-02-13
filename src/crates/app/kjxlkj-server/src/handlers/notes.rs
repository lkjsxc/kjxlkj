use crate::app_state::AppState;
use crate::authn::require_identity;
use crate::error::{new_request_id, ApiError};
use crate::handlers::automation::evaluate_workspace_event;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repos;
use kjxlkj_db::repos::notes::NoteMutationError;
use kjxlkj_db::repos::notes_patch::PatchOp;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{ensure_note_write, ensure_workspace_member_read};
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    title: String,
    note_kind: String,
    access_scope: String,
    markdown: String,
}

#[derive(Debug, Deserialize)]
struct ListNotesQuery {
    workspace_id: Uuid,
    include_deleted: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PatchNoteRequest {
    base_version: i32,
    patch_ops: Vec<PatchOp>,
    idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTitleRequest {
    base_version: i32,
    title: String,
}

#[derive(Debug, Deserialize)]
struct RollbackRequest {
    target_version: i32,
}

#[derive(Debug, Deserialize)]
struct ReplaceTagsRequest {
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    workspace_id: Uuid,
    q: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/notes", web::post().to(create_note))
        .route("/notes", web::get().to(list_notes))
        .route("/notes/{id}", web::get().to(get_note))
        .route("/notes/{id}", web::patch().to(patch_note))
        .route("/notes/{id}/title", web::patch().to(update_title))
        .route("/notes/{id}", web::delete().to(delete_note))
        .route("/notes/{id}/history", web::get().to(note_history))
        .route("/notes/{id}/rollback", web::post().to(rollback_note))
        .route("/notes/{id}/metadata/{key}", web::put().to(upsert_metadata))
        .route("/notes/{id}/metadata/{key}", web::delete().to(delete_metadata))
        .route("/notes/{id}/tags", web::put().to(replace_tags))
        .route("/notes/{id}/backlinks", web::get().to(backlinks))
        .route("/search", web::get().to(search));
}

async fn create_note(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let workspace_role = actor_workspace_role(&state, body.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let (stream, projection) = repos::notes::create_note(
        &state.pool,
        identity.user_id,
        repos::notes::CreateNoteInput {
            workspace_id: body.workspace_id,
            project_id: body.project_id,
            title: body.title.clone(),
            note_kind: body.note_kind.clone(),
            access_scope: body.access_scope.clone(),
            markdown: body.markdown.clone(),
        },
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid note payload"))?;

    let _ = evaluate_workspace_event(
        &state,
        &request_id,
        identity.user_id,
        stream.workspace_id,
        "note_created",
        &format!("note:{}:{}", stream.id, projection.version),
        &json!({
            "note_id": stream.id,
            "version": projection.version,
        }),
    )
    .await;

    Ok(HttpResponse::Created().json(note_response_json(stream, projection, request_id)))
}

async fn list_notes(
    req: HttpRequest,
    query: web::Query<ListNotesQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let workspace_role = actor_workspace_role(&state, query.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let notes = repos::notes::list_notes(&state.pool, query.workspace_id, query.include_deleted.unwrap_or(false))
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "notes": notes
            .into_iter()
            .map(|stream| json!({
                "id": stream.id,
                "workspace_id": stream.workspace_id,
                "project_id": stream.project_id,
                "title": stream.title,
                "note_kind": stream.note_kind,
                "access_scope": stream.access_scope,
                "current_version": stream.current_version,
                "deleted_at": stream.deleted_at.and_then(|value| value.format(&Rfc3339).ok()),
                "created_at": stream.created_at.format(&Rfc3339).unwrap_or_else(|_| stream.created_at.to_string()),
                "updated_at": stream.updated_at.format(&Rfc3339).unwrap_or_else(|_| stream.updated_at.to_string()),
            }))
            .collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn get_note(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    let Some((stream, projection)) = repos::notes::get_note(&state.pool, path.into_inner())
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    Ok(HttpResponse::Ok().json(note_response_json(stream, projection, request_id)))
}

async fn patch_note(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<PatchNoteRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let note_id = path.into_inner();

    let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let idempotency_key = body
        .idempotency_key
        .clone()
        .unwrap_or_else(|| Uuid::now_v7().to_string());

    let result = repos::notes::apply_note_patch(
        &state.pool,
        identity.user_id,
        note_id,
        body.base_version,
        &body.patch_ops,
        &idempotency_key,
    )
    .await
    .map_err(map_note_mutation_error)?;

    let _ = evaluate_workspace_event(
        &state,
        &request_id,
        identity.user_id,
        stream.workspace_id,
        "note_patched",
        &format!("note:{}:{}", note_id, result.event_seq),
        &json!({
            "note_id": note_id,
            "version": result.version,
            "event_seq": result.event_seq,
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(json!({
        "note_id": note_id,
        "version": result.version,
        "event_seq": result.event_seq,
        "idempotency_key": idempotency_key,
        "request_id": request_id,
    })))
}

async fn update_title(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<UpdateTitleRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let note_id = path.into_inner();

    let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let result = repos::notes::update_note_title(
        &state.pool,
        identity.user_id,
        note_id,
        body.base_version,
        &body.title,
    )
    .await
    .map_err(map_note_mutation_error)?;

    let _ = evaluate_workspace_event(
        &state,
        &request_id,
        identity.user_id,
        stream.workspace_id,
        "note_title_updated",
        &format!("note:{}:{}", note_id, result.event_seq),
        &json!({
            "note_id": note_id,
            "version": result.version,
            "event_seq": result.event_seq,
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(json!({
        "note_id": note_id,
        "version": result.version,
        "event_seq": result.event_seq,
        "request_id": request_id,
    })))
}

async fn delete_note(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let note_id = path.into_inner();

    let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    repos::notes::soft_delete_note(&state.pool, identity.user_id, note_id)
        .await
        .map_err(map_note_mutation_error)?;

    let _ = evaluate_workspace_event(
        &state,
        &request_id,
        identity.user_id,
        stream.workspace_id,
        "note_deleted",
        &format!("note:{}:{}", note_id, stream.current_version + 1),
        &json!({
            "note_id": note_id,
            "version": stream.current_version + 1,
        }),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

async fn note_history(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let note_id = path.into_inner();

    let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let events = repos::notes::note_history(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "events": events
            .into_iter()
            .map(|event| json!({
                "event_id": event.event_id,
                "seq": event.seq,
                "event_type": event.event_type,
                "payload": event.payload_json,
                "actor_id": event.actor_id,
                "created_at": event.created_at.format(&Rfc3339).unwrap_or_else(|_| event.created_at.to_string()),
            }))
            .collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn rollback_note(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<RollbackRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let note_id = path.into_inner();

    let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"));
    };

    let workspace_role = actor_workspace_role(&state, stream.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let result = repos::notes::rollback_note(
        &state.pool,
        identity.user_id,
        note_id,
        body.target_version,
    )
    .await
    .map_err(map_note_mutation_error)?;

    let _ = evaluate_workspace_event(
        &state,
        &request_id,
        identity.user_id,
        stream.workspace_id,
        "note_rollback",
        &format!("note:{}:{}", note_id, result.event_seq),
        &json!({
            "note_id": note_id,
            "version": result.version,
            "event_seq": result.event_seq,
            "target_version": body.target_version,
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(json!({
        "note_id": note_id,
        "version": result.version,
        "event_seq": result.event_seq,
        "request_id": request_id,
    })))
}

async fn upsert_metadata(
    req: HttpRequest,
    path: web::Path<(Uuid, String)>,
    body: web::Json<serde_json::Value>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let (note_id, key) = path.into_inner();
    validate_metadata_key(&key)?;

    let workspace_id = repos::notes::note_workspace_id(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"))?;

    let workspace_role = actor_workspace_role(&state, workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    repos::notes::upsert_metadata(&state.pool, note_id, &key, body.into_inner())
        .await
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid metadata payload"))?;

    Ok(HttpResponse::Ok().json(json!({
        "note_id": note_id,
        "key": key,
        "request_id": request_id,
    })))
}

async fn delete_metadata(
    req: HttpRequest,
    path: web::Path<(Uuid, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let identity = require_identity(&req, &state, true).await?;
    let (note_id, key) = path.into_inner();
    validate_metadata_key(&key)?;

    let workspace_id = repos::notes::note_workspace_id(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"))?;

    let workspace_role = actor_workspace_role(&state, workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    repos::notes::delete_metadata_key(&state.pool, note_id, &key)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::NoContent().finish())
}

async fn replace_tags(
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<ReplaceTagsRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let note_id = path.into_inner();

    let workspace_id = repos::notes::note_workspace_id(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"))?;

    let workspace_role = actor_workspace_role(&state, workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let tags: Vec<String> = body
        .tags
        .iter()
        .map(|value| normalize_tag(value))
        .filter(|value| !value.is_empty())
        .collect();

    repos::notes::replace_tags(&state.pool, note_id, &tags)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "note_id": note_id,
        "tags": tags,
        "request_id": request_id,
    })))
}

async fn backlinks(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let note_id = path.into_inner();

    let workspace_id = repos::notes::note_workspace_id(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found"))?;

    let workspace_role = actor_workspace_role(&state, workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let links = repos::notes::note_backlinks(&state.pool, note_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "backlinks": links.into_iter().map(|row| json!({
            "target_title": row.target_title,
            "updated_at": row.updated_at.format(&Rfc3339).unwrap_or_else(|_| row.updated_at.to_string()),
        })).collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn search(
    req: HttpRequest,
    query: web::Query<SearchQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    let workspace_role = actor_workspace_role(&state, query.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let results = repos::notes::search_notes(&state.pool, query.workspace_id, &query.q)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "results": results.into_iter().map(|row| json!({
            "note_id": row.note_id,
            "title": row.title,
            "note_kind": row.note_kind,
            "version": row.version,
            "rank": row.rank,
        })).collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

fn note_response_json(
    stream: kjxlkj_db::models::DbNoteStream,
    projection: kjxlkj_db::models::DbNoteProjection,
    request_id: String,
) -> serde_json::Value {
    json!({
        "note_id": projection.note_id,
        "workspace_id": projection.workspace_id,
        "project_id": projection.project_id,
        "title": projection.title,
        "note_kind": projection.note_kind,
        "version": projection.version,
        "markdown": projection.markdown,
        "metadata_json": projection.metadata_json,
        "deleted_at": stream.deleted_at.and_then(|value| value.format(&Rfc3339).ok()),
        "request_id": request_id,
    })
}

fn map_note_mutation_error(error: NoteMutationError) -> ApiError {
    match error {
        NoteMutationError::NotFound => {
            ApiError::new(StatusCode::NOT_FOUND, "NOTE_NOT_FOUND", "note not found")
        }
        NoteMutationError::Conflict { current_version } => ApiError::new(
            StatusCode::CONFLICT,
            "VERSION_CONFLICT",
            "version conflict",
        )
        .with_details(json!({ "current_version": current_version })),
        NoteMutationError::InvalidPatch => {
            ApiError::new(StatusCode::BAD_REQUEST, "INVALID_PATCH", "invalid patch")
        }
        NoteMutationError::Database(_) => {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error")
        }
    }
}

fn validate_metadata_key(key: &str) -> Result<(), ApiError> {
    if key.starts_with("system.") {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "reserved metadata key prefix",
        ));
    }
    if key.is_empty() || key.len() > 64 || !key.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '.') {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "invalid metadata key",
        ));
    }
    Ok(())
}

fn normalize_tag(tag: &str) -> String {
    tag.trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .collect()
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
