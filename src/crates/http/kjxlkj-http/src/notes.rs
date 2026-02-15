//! Note CRUD, history, and rollback handlers per /docs/spec/api/http.md.

use crate::dto::*;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// POST /api/notes — create note stream.
pub async fn create(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateNoteReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_edit(role) {
        return middleware::forbidden();
    }
    let nid = kjxlkj_domain::types::new_id();
    let title = body.title.as_deref().unwrap_or("Untitled");
    let kind = body.note_kind.as_deref().unwrap_or("markdown");
    let scope = body.access_scope.as_deref().unwrap_or("workspace");
    match kjxlkj_db::repo::note::create_note(
        pool.get_ref(), nid, body.workspace_id, body.project_id, title, kind, scope,
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({
                "id": nid, "workspace_id": body.workspace_id,
                "title": title, "note_kind": kind, "current_version": 0
            })),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/notes/media — create standalone media note from upload.
pub async fn create_media(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateNoteReq>,
) -> HttpResponse {
    // Media note creation delegates to standard create with note_kind override
    let mut body = body.into_inner();
    if body.note_kind.is_none() {
        body.note_kind = Some("media_image".to_string());
    }
    create(req, pool, config, web::Json(body)).await
}

/// GET /api/notes — list notes in workspace.
pub async fn list(
    req: HttpRequest, pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "workspace_id required")),
    };
    match kjxlkj_db::repo::note::list_notes(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|n| {
                serde_json::json!({
                    "id": n.id, "title": n.title, "note_kind": n.note_kind,
                    "workspace_id": n.workspace_id, "current_version": n.current_version,
                    "updated_at": n.updated_at
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/notes/{id} — fetch note projection.
pub async fn get(
    req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::get_projection(pool.get_ref(), nid).await {
        Ok(Some(p)) => HttpResponse::Ok().json(serde_json::json!({
            "note_id": p.note_id, "title": p.title, "note_kind": p.note_kind,
            "version": p.version, "markdown": p.markdown,
            "metadata_json": p.metadata_json, "workspace_id": p.workspace_id
        })),
        Ok(None) => middleware::not_found("note"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/notes/{id} — apply note mutation with version check.
pub async fn patch(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<PatchNoteReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let nid = path.into_inner();
    // Get current projection to apply patch
    let proj = kjxlkj_db::repo::note::get_projection(pool.get_ref(), nid).await;
    let md = match proj {
        Ok(Some(p)) => p.markdown,
        Ok(None) => return middleware::not_found("note"),
        Err(e) => return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    };
    // Parse and apply patch ops
    let ops: Vec<kjxlkj_domain::patch::PatchOp> =
        match serde_json::from_value(body.patch_ops.clone()) {
            Ok(o) => o,
            Err(e) => return HttpResponse::BadRequest()
                .json(ApiError::new("INVALID_PATCH", e.to_string())),
        };
    let new_md = match kjxlkj_domain::patch::apply_patch(&md, &ops) {
        Ok(m) => m,
        Err(e) => return HttpResponse::BadRequest()
            .json(ApiError::new("INVALID_PATCH", e)),
    };
    let payload = serde_json::json!({"patch_ops": body.patch_ops});
    match kjxlkj_db::repo::note::apply_mutation(
        pool.get_ref(), nid, body.base_version, &new_md, None,
        "patch", &payload, ctx.user_id,
    ).await {
        Ok(Some(ver)) => {
            // Update backlinks
            let links = kjxlkj_domain::patch::extract_wiki_links(&new_md);
            let _ = kjxlkj_db::repo::note::update_backlinks(pool.get_ref(), nid, &links).await;
            HttpResponse::Ok().json(serde_json::json!({"note_id": nid, "version": ver}))
        }
        Ok(None) => {
            let cur = kjxlkj_db::repo::note::get_note(pool.get_ref(), nid).await
                .ok().flatten().map(|n| n.current_version).unwrap_or(0);
            middleware::version_conflict(cur)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/notes/{id}/title — update note title with version check.
pub async fn update_title(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<UpdateTitleReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::update_title(
        pool.get_ref(), nid, body.base_version, &body.title, ctx.user_id,
    ).await {
        Ok(Some(ver)) => HttpResponse::Ok()
            .json(serde_json::json!({"note_id": nid, "version": ver, "title": body.title})),
        Ok(None) => {
            let cur = kjxlkj_db::repo::note::get_note(pool.get_ref(), nid).await
                .ok().flatten().map(|n| n.current_version).unwrap_or(0);
            middleware::version_conflict(cur)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/notes/{id} — soft-delete note stream. Returns 204.
pub async fn delete(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::soft_delete(pool.get_ref(), nid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("note"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/notes/{id}/history — list event history.
pub async fn history(
    req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::get_history(pool.get_ref(), nid).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|e| {
                serde_json::json!({
                    "event_id": e.event_id, "seq": e.seq, "event_type": e.event_type,
                    "actor_id": e.actor_id, "created_at": e.created_at
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/notes/{id}/rollback — rollback to selected version.
pub async fn rollback(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<RollbackReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let nid = path.into_inner();
    // Rebuild from events up to target_version
    let events = match kjxlkj_db::repo::note::get_history(pool.get_ref(), nid).await {
        Ok(e) => e, Err(e) => return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    };
    let mut md = String::new();
    for ev in &events {
        if ev.seq > body.target_version { break; }
        if ev.event_type == "patch" {
            if let Some(ops_val) = ev.payload_json.get("patch_ops") {
                let ops: Vec<kjxlkj_domain::patch::PatchOp> =
                    serde_json::from_value(ops_val.clone()).unwrap_or_default();
                md = kjxlkj_domain::patch::apply_patch(&md, &ops).unwrap_or(md.clone());
            }
        }
    }
    // Get current version
    let cur = kjxlkj_db::repo::note::get_note(pool.get_ref(), nid).await
        .ok().flatten().map(|n| n.current_version).unwrap_or(0);
    let payload = serde_json::json!({"rollback_to": body.target_version});
    match kjxlkj_db::repo::note::apply_mutation(
        pool.get_ref(), nid, cur, &md, None, "rollback", &payload, ctx.user_id,
    ).await {
        Ok(Some(ver)) => HttpResponse::Ok()
            .json(serde_json::json!({"note_id": nid, "version": ver})),
        Ok(None) => middleware::version_conflict(cur),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
