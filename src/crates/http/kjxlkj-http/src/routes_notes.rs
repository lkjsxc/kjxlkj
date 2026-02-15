use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_note;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{EventId, NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// POST /notes per /docs/spec/api/http.md.
pub async fn create_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateNoteRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(body.workspace_id);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(Uuid::now_v7());
    let title = body.title.as_deref().unwrap_or("Untitled");
    let kind = body.note_kind.as_deref().unwrap_or("markdown");
    match repo_note::create_note_stream(pool.get_ref(), note_id, ws_id, title, kind).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "note_id": note_id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes per /docs/spec/api/http.md.
pub async fn list_notes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id_str = query.get("workspace_id").map(|s| s.as_str()).unwrap_or("");
    let ws_id = match ws_id_str.parse::<Uuid>() {
        Ok(u) => WorkspaceId(u),
        Err(_) => {
            return domain_error_response(
                DomainError::BadRequest("workspace_id required".into()),
                &rid,
            )
        }
    };
    match repo_note::list_notes(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<NoteStreamResponse> = rows
                .into_iter()
                .map(|n| NoteStreamResponse {
                    id: n.id,
                    workspace_id: n.workspace_id,
                    project_id: n.project_id,
                    title: n.title,
                    note_kind: n.note_kind,
                    current_version: n.current_version,
                    created_at: n.created_at.to_string(),
                    updated_at: n.updated_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes/{id} per /docs/spec/api/http.md.
pub async fn get_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(path.into_inner());
    match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => HttpResponse::Ok().json(NoteProjectionResponse {
            note_id: p.note_id,
            workspace_id: p.workspace_id,
            title: p.title,
            note_kind: p.note_kind,
            version: p.version,
            markdown: p.markdown,
            rendered_html: p.rendered_html,
            metadata_json: p.metadata_json,
        }),
        Ok(None) => domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// PATCH /notes/{id} per /docs/spec/api/http.md.
pub async fn patch_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PatchNoteRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    // Fetch current stream for version check
    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => return domain_error_response(DomainError::Internal(e.to_string()), &rid),
    };

    if stream.deleted_at.is_some() {
        return domain_error_response(DomainError::NotFound("note deleted".into()), &rid);
    }

    // Version conflict check per /docs/spec/api/http.md
    if body.base_version != stream.current_version {
        return domain_error_response(
            DomainError::VersionConflict {
                expected: body.base_version,
                found: stream.current_version,
            },
            &rid,
        );
    }

    // RBAC check
    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    // Apply patch (simplified: store ops as event, get projection)
    let new_version = stream.current_version + 1;
    let event_id = EventId(Uuid::now_v7());
    let payload = serde_json::json!({ "ops": body.ops });

    if let Err(e) = repo_note::append_note_event(
        pool.get_ref(),
        event_id,
        note_id,
        new_version,
        "patch",
        &payload,
        identity.user_id,
    )
    .await
    {
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    // Get current projection to apply patch
    let proj = match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => p,
        _ => return domain_error_response(DomainError::Internal("projection missing".into()), &rid),
    };

    // Apply ops to markdown (simplified text replacement)
    let new_markdown = apply_patch_ops(&proj.markdown, &body.ops);

    if let Err(e) = repo_note::update_note_projection(
        pool.get_ref(),
        note_id,
        &proj.title,
        new_version,
        &new_markdown,
        "",
        &proj.metadata_json,
    )
    .await
    {
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "note_id": note_id.0,
        "version": new_version,
        "request_id": rid
    }))
}

/// PATCH /notes/{id}/title per /docs/spec/api/http.md.
pub async fn patch_title(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PatchTitleRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => return domain_error_response(DomainError::Internal(e.to_string()), &rid),
    };

    if body.base_version != stream.current_version {
        return domain_error_response(
            DomainError::VersionConflict {
                expected: body.base_version,
                found: stream.current_version,
            },
            &rid,
        );
    }

    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    let new_version = stream.current_version + 1;
    let event_id = EventId(Uuid::now_v7());
    let payload = serde_json::json!({ "title": body.title });

    if let Err(e) = repo_note::append_note_event(
        pool.get_ref(), event_id, note_id, new_version,
        "title_update", &payload, identity.user_id,
    ).await {
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    let proj = match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => p,
        _ => return domain_error_response(DomainError::Internal("projection missing".into()), &rid),
    };

    if let Err(e) = repo_note::update_note_projection(
        pool.get_ref(), note_id, &body.title, new_version,
        &proj.markdown, &proj.rendered_html, &proj.metadata_json,
    ).await {
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "note_id": note_id.0,
        "version": new_version,
        "request_id": rid
    }))
}

/// DELETE /notes/{id} per /docs/spec/api/http.md.
/// Returns 204 on successful soft-delete.
pub async fn delete_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => return domain_error_response(DomainError::Internal(e.to_string()), &rid),
    };

    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    match repo_note::soft_delete_note(pool.get_ref(), note_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes/{id}/history per /docs/spec/api/http.md.
pub async fn note_history(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(path.into_inner());
    match repo_note::list_note_events(pool.get_ref(), note_id).await {
        Ok(events) => {
            let list: Vec<serde_json::Value> = events
                .into_iter()
                .map(|e| serde_json::json!({
                    "event_id": e.event_id,
                    "note_id": e.note_id,
                    "seq": e.seq,
                    "event_type": e.event_type,
                    "payload_json": e.payload_json,
                    "actor_id": e.actor_id,
                    "created_at": e.created_at.to_string()
                }))
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// Apply simplified patch ops to markdown.
fn apply_patch_ops(base: &str, ops: &[serde_json::Value]) -> String {
    let mut result = String::new();
    let chars: Vec<char> = base.chars().collect();
    let mut pos = 0usize;

    for op in ops {
        if let Some(retain) = op.get("retain").and_then(|v| v.as_u64()) {
            let end = (pos + retain as usize).min(chars.len());
            for c in &chars[pos..end] {
                result.push(*c);
            }
            pos = end;
        } else if let Some(text) = op.get("insert").and_then(|v| v.as_str()) {
            result.push_str(text);
        } else if let Some(del) = op.get("delete").and_then(|v| v.as_u64()) {
            pos = (pos + del as usize).min(chars.len());
        }
    }
    // Append remaining
    for c in &chars[pos..] {
        result.push(*c);
    }
    result
}
