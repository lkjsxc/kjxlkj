use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use kjxlkj_db::repo_note;
use kjxlkj_db::repo_event;
use kjxlkj_db::repo_search;
use kjxlkj_domain::note::default_note_title;
use kjxlkj_domain::error::ErrorCode;
use kjxlkj_rbac::check::{self, RbacError};
use kjxlkj_domain::permission::Role;
use crate::extract;
use crate::response::{error_response, error_response_with_details};

#[derive(Deserialize)]
pub struct CreateNoteBody {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<String>,
    pub markdown: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchNoteBody {
    pub base_version: i64,
    pub markdown: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTitleBody {
    pub base_version: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct ListNotesQuery {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
}

/// GET /api/notes
pub async fn list_notes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<ListNotesQuery>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    if let Err(e) = check::require_role(&pool, query.workspace_id, identity.user_id, Role::Viewer).await {
        return rbac_error_response(e);
    }

    match repo_note::list_notes(&pool, query.workspace_id, query.project_id).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(_) => error_response(ErrorCode::InternalError, "failed to list notes"),
    }
}

/// POST /api/notes
pub async fn create_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateNoteBody>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    if let Err(e) = check::require_role(&pool, body.workspace_id, identity.user_id, Role::Editor).await {
        return rbac_error_response(e);
    }

    // Default title rule: assign datetime if no title
    let title = body.title.clone().unwrap_or_else(default_note_title);
    let note_kind = body.note_kind.as_deref().unwrap_or("markdown");
    let markdown = body.markdown.as_deref().unwrap_or("");
    let note_id = Uuid::now_v7();

    match repo_note::create_note(&pool, note_id, body.workspace_id, body.project_id, &title, note_kind, markdown).await {
        Ok(note) => {
            // Update search index
            let _ = repo_search::upsert_search_index(&pool, note_id, &title, markdown).await;
            HttpResponse::Created().json(note)
        }
        Err(_) => error_response(ErrorCode::InternalError, "failed to create note"),
    }
}

/// GET /api/notes/{id}
pub async fn get_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let note_id = path.into_inner();
    let note = match repo_note::get_note(&pool, note_id).await {
        Ok(Some(n)) => n,
        Ok(None) => return error_response(ErrorCode::NoteNotFound, "note not found"),
        Err(_) => return error_response(ErrorCode::InternalError, "failed to get note"),
    };

    if let Err(e) = check::require_role(&pool, note.workspace_id, identity.user_id, Role::Viewer).await {
        return rbac_error_response(e);
    }

    match repo_note::get_projection(&pool, note_id).await {
        Ok(Some(proj)) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": proj.note_id,
            "title": note.title,
            "version": proj.version,
            "markdown": proj.markdown,
            "metadata_json": proj.metadata_json,
            "note_kind": note.note_kind,
            "workspace_id": note.workspace_id,
            "created_at": note.created_at.to_string(),
            "updated_at": note.updated_at.to_string(),
        })),
        Ok(None) => error_response(ErrorCode::NoteNotFound, "projection not found"),
        Err(_) => error_response(ErrorCode::InternalError, "failed to get projection"),
    }
}

/// PATCH /api/notes/{id}
pub async fn patch_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PatchNoteBody>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    let note_id = path.into_inner();
    let note = match repo_note::get_note(&pool, note_id).await {
        Ok(Some(n)) => n,
        Ok(None) => return error_response(ErrorCode::NoteNotFound, "note not found"),
        Err(_) => return error_response(ErrorCode::InternalError, "db error"),
    };

    if let Err(e) = check::require_role(&pool, note.workspace_id, identity.user_id, Role::Editor).await {
        return rbac_error_response(e);
    }

    let markdown = body.markdown.as_deref().unwrap_or("");
    match repo_note::patch_note(&pool, note_id, body.base_version, markdown, identity.user_id, "user").await {
        Ok(new_version) => {
            // Update search index
            let _ = repo_search::upsert_search_index(&pool, note_id, &note.title, markdown).await;
            HttpResponse::Ok().json(serde_json::json!({
                "note_id": note_id,
                "version": new_version,
            }))
        }
        Err(repo_note::PatchError::NotFound) => {
            error_response(ErrorCode::NoteNotFound, "note not found")
        }
        Err(repo_note::PatchError::Conflict { expected, actual }) => {
            error_response_with_details(
                ErrorCode::VersionConflict,
                "version conflict",
                serde_json::json!({"expected": expected, "current": actual}),
            )
        }
        Err(repo_note::PatchError::Db(_)) => {
            error_response(ErrorCode::InternalError, "patch failed")
        }
    }
}

/// PATCH /api/notes/{id}/title
pub async fn update_title(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTitleBody>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    let note_id = path.into_inner();
    let note = match repo_note::get_note(&pool, note_id).await {
        Ok(Some(n)) => n,
        Ok(None) => return error_response(ErrorCode::NoteNotFound, "note not found"),
        Err(_) => return error_response(ErrorCode::InternalError, "db error"),
    };

    if let Err(e) = check::require_role(&pool, note.workspace_id, identity.user_id, Role::Editor).await {
        return rbac_error_response(e);
    }

    match repo_note::update_title(&pool, note_id, body.base_version, &body.title, identity.user_id, "user").await {
        Ok(new_version) => {
            HttpResponse::Ok().json(serde_json::json!({
                "note_id": note_id,
                "version": new_version,
                "title": body.title,
            }))
        }
        Err(repo_note::PatchError::Conflict { expected, actual }) => {
            error_response_with_details(
                ErrorCode::VersionConflict,
                "version conflict",
                serde_json::json!({"expected": expected, "current": actual}),
            )
        }
        Err(repo_note::PatchError::NotFound) => {
            error_response(ErrorCode::NoteNotFound, "note not found")
        }
        Err(repo_note::PatchError::Db(_)) => {
            error_response(ErrorCode::InternalError, "title update failed")
        }
    }
}

/// DELETE /api/notes/{id}
pub async fn delete_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    let note_id = path.into_inner();
    let note = match repo_note::get_note(&pool, note_id).await {
        Ok(Some(n)) => n,
        Ok(None) => return error_response(ErrorCode::NoteNotFound, "note not found"),
        Err(_) => return error_response(ErrorCode::InternalError, "db error"),
    };

    if let Err(e) = check::require_role(&pool, note.workspace_id, identity.user_id, Role::Editor).await {
        return rbac_error_response(e);
    }

    match repo_note::soft_delete(&pool, note_id).await {
        Ok(()) => {
            let _ = repo_search::remove_from_index(&pool, note_id).await;
            HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
        }
        Err(_) => error_response(ErrorCode::InternalError, "delete failed"),
    }
}

/// GET /api/notes/{id}/history
pub async fn note_history(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let note_id = path.into_inner();
    let note = match repo_note::get_note(&pool, note_id).await {
        Ok(Some(n)) => n,
        Ok(None) => return error_response(ErrorCode::NoteNotFound, "note not found"),
        Err(_) => return error_response(ErrorCode::InternalError, "db error"),
    };

    if let Err(e) = check::require_role(&pool, note.workspace_id, identity.user_id, Role::Viewer).await {
        return rbac_error_response(e);
    }

    match repo_event::get_note_history(&pool, note_id).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => error_response(ErrorCode::InternalError, "failed to get history"),
    }
}

fn rbac_error_response(e: RbacError) -> HttpResponse {
    match e {
        RbacError::Forbidden => error_response(ErrorCode::RoleForbidden, "role forbidden"),
        RbacError::NotMember => error_response(ErrorCode::WorkspaceForbidden, "not a workspace member"),
        RbacError::Db(_) => error_response(ErrorCode::InternalError, "authorization check failed"),
    }
}
