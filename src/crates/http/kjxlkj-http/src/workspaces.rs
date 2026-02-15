//! Workspace, member, and project handlers per /docs/spec/api/http.md.

use crate::dto::*;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// GET /api/workspaces
pub async fn list(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    match kjxlkj_db::repo::workspace::list_workspaces(pool.get_ref()).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|w| {
                serde_json::json!({
                    "id": w.id, "slug": w.slug, "name": w.name,
                    "owner_user_id": w.owner_user_id, "created_at": w.created_at
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/workspaces
pub async fn create(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateWorkspaceReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_manage_workspace(role) {
        return middleware::forbidden();
    }
    let wid = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::workspace::create_workspace(
        pool.get_ref(), wid, &body.slug, &body.name, ctx.user_id,
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"id": wid, "slug": body.slug, "name": body.name})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/workspaces/{id}
pub async fn update(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<UpdateWorkspaceReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let wid = path.into_inner();
    let ws = match kjxlkj_db::repo::workspace::find_workspace(pool.get_ref(), wid).await {
        Ok(Some(w)) => w, Ok(None) => return middleware::not_found("workspace"),
        Err(e) => return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    };
    let name = body.name.as_deref().unwrap_or(&ws.name);
    let slug = body.slug.as_deref().unwrap_or(&ws.slug);
    match kjxlkj_db::repo::workspace::update_workspace(pool.get_ref(), wid, name, slug).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"id": wid})),
        Ok(false) => middleware::not_found("workspace"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/workspaces/{id}
pub async fn delete(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let wid = path.into_inner();
    match kjxlkj_db::repo::workspace::delete_workspace(pool.get_ref(), wid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("workspace"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/workspaces/{id}/members
pub async fn list_members(
    req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    match kjxlkj_db::repo::workspace::list_members(pool.get_ref(), path.into_inner()).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|m| {
                serde_json::json!({"workspace_id": m.workspace_id, "user_id": m.user_id,
                    "role": m.role, "joined_at": m.joined_at})
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PUT /api/workspaces/{id}/members/{user_id}
pub async fn upsert_member(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<(Uuid, Uuid)>, body: web::Json<UpsertMemberReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let (wid, uid) = path.into_inner();
    match kjxlkj_db::repo::workspace::upsert_member(pool.get_ref(), wid, uid, &body.role).await {
        Ok(()) => HttpResponse::Ok()
            .json(serde_json::json!({"workspace_id": wid, "user_id": uid, "role": body.role})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/projects
pub async fn list_projects(
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
    match kjxlkj_db::repo::workspace::list_projects(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|p| {
                serde_json::json!({"id": p.id, "workspace_id": p.workspace_id,
                    "name": p.name, "description": p.description, "created_at": p.created_at})
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/projects
pub async fn create_project(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateProjectReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let pid = kjxlkj_domain::types::new_id();
    let desc = body.description.as_deref().unwrap_or("");
    match kjxlkj_db::repo::workspace::create_project(
        pool.get_ref(), pid, body.workspace_id, &body.name, desc,
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"id": pid, "name": body.name})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/projects/{id}
pub async fn update_project(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<UpdateProjectReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let pid = path.into_inner();
    let name = body.name.as_deref().unwrap_or("");
    let desc = body.description.as_deref().unwrap_or("");
    match kjxlkj_db::repo::workspace::update_project(pool.get_ref(), pid, name, desc).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"id": pid})),
        Ok(false) => middleware::not_found("project"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/projects/{id}
pub async fn delete_project(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let pid = path.into_inner();
    match kjxlkj_db::repo::workspace::delete_project(pool.get_ref(), pid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("project"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
