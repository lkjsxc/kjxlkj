use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_note;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{EventId, NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::RollbackNoteRequest;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;
use crate::patch_ops::apply_patch_ops;

/// POST /notes/{id}/rollback per /docs/spec/api/http.md.
/// Rebuilds markdown at target_version by replaying events from
/// the latest snapshot, then creates a rollback event.
pub async fn rollback_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<RollbackNoteRequest>,
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

    if stream.deleted_at.is_some() {
        return domain_error_response(
            DomainError::NotFound("note deleted".into()), &rid,
        );
    }

    // Validate target_version
    if body.target_version < 0 || body.target_version > stream.current_version {
        return domain_error_response(
            DomainError::BadRequest("invalid target_version".into()), &rid,
        );
    }

    // RBAC check
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

    // Rebuild markdown at target_version by replaying events
    let rebuilt_md = match rebuild_at_version(
        pool.get_ref(), note_id, body.target_version,
    ).await {
        Ok(md) => md,
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };

    // Create rollback event at new version
    let new_version = stream.current_version + 1;
    let event_id = EventId(Uuid::now_v7());
    let payload = serde_json::json!({
        "rollback_to": body.target_version,
        "markdown": rebuilt_md
    });

    if let Err(e) = repo_note::append_note_event(
        pool.get_ref(), event_id, note_id, new_version,
        "rollback", &payload, identity.user_id,
    ).await {
        return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        );
    }

    // Update projection with rebuilt content
    let proj = match repo_note::find_note_projection(
        pool.get_ref(), note_id,
    ).await {
        Ok(Some(p)) => p,
        _ => return domain_error_response(
            DomainError::Internal("projection missing".into()), &rid,
        ),
    };

    if let Err(e) = repo_note::update_note_projection(
        pool.get_ref(), note_id, &proj.title, new_version,
        &rebuilt_md, "", &proj.metadata_json,
    ).await {
        return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        );
    }

    // Store snapshot per /docs/spec/domain/events.md (every 100 events)
    if new_version % 100 == 0 {
        let _ = repo_note::store_snapshot(
            pool.get_ref(), note_id, new_version,
            &rebuilt_md, &proj.metadata_json,
        ).await;
    }

    HttpResponse::Ok().json(serde_json::json!({
        "note_id": note_id.0,
        "version": new_version,
        "rollback_to": body.target_version,
        "request_id": rid
    }))
}

/// Rebuild markdown at a specific version by replaying events.
/// Uses snapshot if available, otherwise replays from version 0.
async fn rebuild_at_version(
    pool: &PgPool,
    note_id: NoteId,
    target_version: i64,
) -> Result<String, sqlx::Error> {
    // Find latest snapshot before target
    let (base_md, from_seq) = match repo_note::find_latest_snapshot(
        pool, note_id, target_version,
    ).await? {
        Some(snap) => (snap.markdown, snap.at_seq),
        None => (String::new(), 0),
    };

    // Fetch events from snapshot to target
    let events = repo_note::list_note_events_from(
        pool, note_id, from_seq, target_version - from_seq,
    ).await?;

    // Replay events to rebuild markdown
    let mut markdown = base_md;
    for event in events {
        if event.seq > target_version {
            break;
        }
        match event.event_type.as_str() {
            "patch" => {
                if let Some(ops) = event.payload_json.get("ops") {
                    if let Some(ops_arr) = ops.as_array() {
                        markdown = apply_patch_ops(&markdown, ops_arr);
                    }
                }
            }
            "rollback" => {
                if let Some(md) = event.payload_json.get("markdown") {
                    if let Some(s) = md.as_str() {
                        markdown = s.to_string();
                    }
                }
            }
            _ => {} // title_update etc. don't affect body
        }
    }

    Ok(markdown)
}
