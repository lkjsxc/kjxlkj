// Note handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::{require_role, AuthSession};
use kjxlkj_db::repo::notes as note_repo;
use kjxlkj_domain::types::{AccessScope, NoteKind, NoteStream, Role};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateNoteRequest, ErrorBody, UpdateNoteRequest, UpdateTitleRequest, MetadataRequest};

/// POST /api/notes — editor+ only
pub async fn create(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    body: web::Json<CreateNoteRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Editor) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Editor role required".into(),
            details: None, request_id: rid,
        });
    }
    let kind = match body.note_kind.as_deref() {
        Some("settings") => NoteKind::Settings,
        Some("media_image") => NoteKind::MediaImage,
        Some("media_video") => NoteKind::MediaVideo,
        _ => NoteKind::Markdown,
    };
    let scope = match body.access_scope.as_deref() {
        Some("project") => AccessScope::Project,
        Some("private") => AccessScope::Private,
        _ => AccessScope::Workspace,
    };
    let note = NoteStream {
        id: Uuid::now_v7(),
        workspace_id: body.workspace_id,
        project_id: body.project_id,
        title: body.title.clone(),
        note_kind: kind,
        access_scope: scope,
        created_at: String::new(),
        updated_at: String::new(),
        current_version: 0,
        deleted_at: None,
    };
    match note_repo::insert_note_stream(pool.get_ref(), &note).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "id": note.id, "title": note.title, "version": 0
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// GET /api/notes
pub async fn list(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    query: web::Query<WsFilter>,
) -> HttpResponse {
    match note_repo::list_notes(pool.get_ref(), query.workspace_id).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// GET /api/notes/{id}
pub async fn get(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match note_repo::find_by_id(pool.get_ref(), path.into_inner()).await {
        Ok(Some(proj)) => HttpResponse::Ok().json(proj),
        Ok(None) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOTE_NOT_FOUND".into(), message: "Note not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/notes/{id} — apply patch with version check
pub async fn update(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<UpdateNoteRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let note_id = path.into_inner();

    // Get current content to apply patch
    let current = match note_repo::find_by_id(pool.get_ref(), note_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorBody {
                code: "NOTE_NOT_FOUND".into(), message: "Note not found".into(),
                details: None, request_id: rid,
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(), message: e.to_string(),
                details: None, request_id: rid,
            });
        }
    };

    // Parse and apply patch ops
    let ops: Vec<kjxlkj_domain::patch::PatchOp> = match serde_json::from_value(
        serde_json::Value::Array(body.patch_ops.clone()),
    ) {
        Ok(o) => o,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorBody {
                code: "INVALID_PATCH".into(), message: e.to_string(),
                details: None, request_id: rid,
            });
        }
    };

    let new_markdown = match kjxlkj_domain::patch::apply_patch(&current.markdown, &ops) {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorBody {
                code: "INVALID_PATCH".into(), message: e,
                details: None, request_id: rid,
            });
        }
    };

    let payload = serde_json::json!({ "patch_ops": body.patch_ops });
    let actor_id = auth.user.id;

    match note_repo::apply_mutation(
        pool.get_ref(), note_id, body.base_version, &new_markdown, actor_id, "updated", &payload,
    ).await {
        Ok(Some(version)) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": note_id, "version": version
        })),
        Ok(None) => {
            let cv = note_repo::current_version(pool.get_ref(), note_id).await.unwrap_or(0);
            HttpResponse::Conflict().json(ErrorBody {
                code: "VERSION_CONFLICT".into(),
                message: format!("Expected version {}, current is {cv}", body.base_version),
                details: Some(serde_json::json!({
                    "expected_version": body.base_version,
                    "current_version": cv
                })),
                request_id: rid,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/notes/{id}/title
pub async fn update_title(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTitleRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let note_id = path.into_inner();
    let actor_id = auth.user.id;

    match note_repo::update_title(
        pool.get_ref(), note_id, body.base_version, &body.title, actor_id,
    ).await {
        Ok(Some(version)) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": note_id, "version": version
        })),
        Ok(None) => {
            let cv = note_repo::current_version(pool.get_ref(), note_id).await.unwrap_or(0);
            HttpResponse::Conflict().json(ErrorBody {
                code: "VERSION_CONFLICT".into(),
                message: format!("Expected version {}, current is {cv}", body.base_version),
                details: Some(serde_json::json!({
                    "expected_version": body.base_version,
                    "current_version": cv
                })),
                request_id: rid,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/notes/{id} — returns 204 per spec
pub async fn delete(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Editor) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Editor role required".into(),
            details: None, request_id: rid,
        });
    }
    match note_repo::soft_delete(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOTE_NOT_FOUND".into(), message: "Note not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// GET /api/notes/{id}/history
pub async fn history(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    match note_repo::event_history(pool.get_ref(), path.into_inner()).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// PUT /api/notes/{id}/metadata/{key}
pub async fn upsert_metadata(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<(Uuid, String)>,
    body: web::Json<MetadataRequest>,
) -> HttpResponse {
    let (note_id, key) = path.into_inner();
    let rid = Uuid::now_v7().to_string();

    // Update metadata in projection
    match sqlx::query(
        "UPDATE note_projections SET metadata_json = jsonb_set(metadata_json, ARRAY[$1], $2)
         WHERE note_id = $3",
    )
    .bind(&key)
    .bind(&body.value)
    .bind(note_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "ok"})),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/notes/{id}/metadata/{key} — returns 204 per spec
pub async fn delete_metadata(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<(Uuid, String)>,
) -> HttpResponse {
    let (note_id, key) = path.into_inner();
    let _ = sqlx::query(
        "UPDATE note_projections SET metadata_json = metadata_json - $1
         WHERE note_id = $2",
    )
    .bind(&key)
    .bind(note_id)
    .execute(pool.get_ref())
    .await;
    HttpResponse::NoContent().finish()
}

#[derive(serde::Deserialize)]
pub struct WsFilter {
    pub workspace_id: Uuid,
}
