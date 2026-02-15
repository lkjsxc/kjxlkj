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
use crate::patch_ops::apply_patch_ops;

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(
        err,
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505")
    )
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
        if is_unique_violation(&e) {
            let found = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
                Ok(Some(s)) => s.current_version.max(stream.current_version + 1),
                _ => stream.current_version + 1,
            };
            return domain_error_response(
                DomainError::VersionConflict {
                    expected: body.base_version,
                    found,
                },
                &rid,
            );
        }
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    let proj = match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => p,
        _ => {
            return domain_error_response(
                DomainError::Internal("projection missing".into()),
                &rid,
            )
        }
    };

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
        pool.get_ref(),
        event_id,
        note_id,
        new_version,
        "title_update",
        &payload,
        identity.user_id,
    )
    .await
    {
        if is_unique_violation(&e) {
            let found = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
                Ok(Some(s)) => s.current_version.max(stream.current_version + 1),
                _ => stream.current_version + 1,
            };
            return domain_error_response(
                DomainError::VersionConflict {
                    expected: body.base_version,
                    found,
                },
                &rid,
            );
        }
        return domain_error_response(DomainError::Internal(e.to_string()), &rid);
    }

    let proj = match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => p,
        _ => {
            return domain_error_response(
                DomainError::Internal("projection missing".into()),
                &rid,
            )
        }
    };

    if let Err(e) = repo_note::update_note_projection(
        pool.get_ref(),
        note_id,
        &body.title,
        new_version,
        &proj.markdown,
        &proj.rendered_html,
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
