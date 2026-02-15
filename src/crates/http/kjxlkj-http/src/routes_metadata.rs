use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::{repo_metadata, repo_note, repo_tag};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use kjxlkj_domain::metadata;
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// PUT /notes/{id}/metadata/{key} per /docs/spec/api/http.md.
pub async fn upsert_metadata(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, String)>,
    body: web::Json<UpsertMetadataRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let (note_uuid, key) = path.into_inner();
    let note_id = NoteId(note_uuid);

    // Validate key per /docs/spec/domain/metadata.md
    if let Some(err_msg) = metadata::validate_key(&key) {
        return domain_error_response(
            DomainError::BadRequest(err_msg.into()), &rid,
        );
    }
    if let Some(err_msg) = metadata::validate_value(&body.value) {
        return domain_error_response(
            DomainError::BadRequest(err_msg.into()), &rid,
        );
    }

    // Check note exists and RBAC
    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };
    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(), ws_id, identity.user_id,
    ).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    match repo_metadata::upsert_metadata(
        pool.get_ref(), note_id, &key, &body.value,
    ).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": note_uuid, "key": key, "request_id": rid
        })),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// DELETE /notes/{id}/metadata/{key} per /docs/spec/api/http.md.
/// Returns 204.
pub async fn delete_metadata(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, String)>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let (note_uuid, key) = path.into_inner();
    let note_id = NoteId(note_uuid);

    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };
    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(), ws_id, identity.user_id,
    ).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    match repo_metadata::delete_metadata(pool.get_ref(), note_id, &key).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// PUT /notes/{id}/tags per /docs/spec/api/http.md.
pub async fn replace_tags(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReplaceTagsRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };
    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(), ws_id, identity.user_id,
    ).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    // Find or create each tag, collect IDs
    let mut tag_ids = Vec::new();
    for tag_name in &body.tags {
        match repo_tag::find_or_create_tag(pool.get_ref(), ws_id, tag_name).await {
            Ok(id) => tag_ids.push(id),
            Err(e) => return domain_error_response(
                DomainError::Internal(e.to_string()), &rid,
            ),
        }
    }

    match repo_tag::replace_note_tags(pool.get_ref(), note_id, &tag_ids).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": note_id.0, "tags": body.tags, "request_id": rid
        })),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /tags per /docs/spec/api/http.md.
pub async fn list_tags(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_str = query.get("workspace_id").map(|s| s.as_str()).unwrap_or("");
    let ws_id = match ws_str.parse::<Uuid>() {
        Ok(u) => WorkspaceId(u),
        Err(_) => return domain_error_response(
            DomainError::BadRequest("workspace_id required".into()), &rid,
        ),
    };
    match repo_tag::list_tags(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<TagResponse> = rows.into_iter().map(|t| TagResponse {
                id: t.id, workspace_id: t.workspace_id, name: t.name,
            }).collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
