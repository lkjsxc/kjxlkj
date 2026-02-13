use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateNoteRequest, PatchNoteRequest, PatchNoteTitleRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::{AccessScope, NoteKind};
use kjxlkj_rbac::{guard, roles};
use kjxlkj_search::backlinks;

/// POST /api/notes
pub async fn create_note(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateNoteRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), body.workspace_id, auth.user_id, roles::can_edit_notes,
    ).await {
        return domain_error_response(&e);
    }
    let kind = match body.note_kind.as_deref().unwrap_or("markdown") {
        "markdown" => NoteKind::Markdown,
        "settings" => NoteKind::Settings,
        "media_image" => NoteKind::MediaImage,
        "media_video" => NoteKind::MediaVideo,
        _ => NoteKind::Markdown,
    };
    let scope = if body.project_id.is_some() {
        AccessScope::Project
    } else {
        AccessScope::Workspace
    };
    let note_id = uuid::Uuid::new_v4();
    match repos::notes::create(
        pool.get_ref(), note_id, body.workspace_id, body.project_id,
        &body.title, body.body.as_deref().unwrap_or(""), kind, scope,
    ).await {
        Ok(note) => {
            let body_text = body.body.as_deref().unwrap_or("");
            let _ = repos::events::append_note_event(
                pool.get_ref(), uuid::Uuid::new_v4(), note.id, note.version,
                "note.created",
                &serde_json::json!({"title": body.title}), auth.user_id,
            ).await;
            let _ = backlinks::refresh_backlinks(pool.get_ref(), note.id, body_text).await;
            HttpResponse::Created().json(note)
        }
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// GET /api/notes?workspace_id=...
pub async fn list_notes(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return domain_error_response(&DomainError::BadRequest {
            reason: "workspace_id required".into(),
        }),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_view_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match repos::notes::list_active(pool.get_ref(), ws_id).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// GET /api/notes/{id}
pub async fn get_note(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match repos::notes::find_by_id(pool.get_ref(), id).await {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => domain_error_response(&DomainError::NotFound {
            entity: "note".into(),
        }),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// PATCH /api/notes/{id} — body mutation with version check.
pub async fn patch_note(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<PatchNoteRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match repos::notes::update_body(pool.get_ref(), id, &body.body, body.version).await {
        Ok(Some(note)) => {
            let _ = repos::events::append_note_event(
                pool.get_ref(), uuid::Uuid::new_v4(), id, note.version,
                "note.updated",
                &serde_json::json!({"version": note.version}), auth.user_id,
            ).await;
            let _ = backlinks::refresh_backlinks(pool.get_ref(), id, &body.body).await;
            HttpResponse::Ok().json(note)
        }
        Ok(None) => {
            let current = repos::notes::get_version(pool.get_ref(), id).await.ok();
            domain_error_response(&DomainError::VersionConflict {
                expected: body.version,
                actual: current.flatten().unwrap_or(-1),
            })
        }
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// PATCH /api/notes/{id}/title
pub async fn patch_note_title(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<PatchNoteTitleRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match repos::notes::update_title(pool.get_ref(), id, &body.title, body.version).await {
        Ok(Some(note)) => {
            let _ = repos::events::append_note_event(
                pool.get_ref(), uuid::Uuid::new_v4(), id, note.version,
                "note.retitled",
                &serde_json::json!({"title": body.title, "version": note.version}),
                auth.user_id,
            ).await;
            HttpResponse::Ok().json(note)
        }
        Ok(None) => domain_error_response(&DomainError::VersionConflict {
            expected: body.version,
            actual: -1,
        }),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}


