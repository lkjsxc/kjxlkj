use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::{repo_workspace, repo_project};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::WorkspaceId;
use kjxlkj_rbac::guard;
use kjxlkj_workspace::service as ws_svc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

// Re-export member handlers for route wiring.
pub use crate::routes_members::{list_members, upsert_member};

/// GET /workspaces per /docs/spec/api/http.md.
pub async fn list_workspaces(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    match repo_workspace::list_workspaces(pool.get_ref()).await {
        Ok(rows) => {
            let list: Vec<WorkspaceResponse> = rows
                .into_iter()
                .map(|w| WorkspaceResponse {
                    id: w.id,
                    slug: w.slug,
                    name: w.name,
                    owner_user_id: w.owner_user_id,
                    created_at: w.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /workspaces per /docs/spec/api/http.md.
pub async fn create_workspace(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateWorkspaceRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    match ws_svc::create_workspace(
        pool.get_ref(),
        &body.slug,
        &body.name,
        identity.user_id,
    )
    .await
    {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({
            "workspace_id": id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(e, &rid),
    }
}

/// PATCH /workspaces/{id} per /docs/spec/api/http.md.
pub async fn update_workspace(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreateWorkspaceRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(path.into_inner());
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_admin(role) {
        return domain_error_response(e, &rid);
    }
    match repo_workspace::update_workspace(pool.get_ref(), ws_id, &body.name, &body.slug).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "updated", "request_id": rid
        })),
        Ok(false) => domain_error_response(DomainError::NotFound("workspace".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// DELETE /workspaces/{id} per /docs/spec/api/http.md.
pub async fn delete_workspace(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(path.into_inner());
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_owner(role) {
        return domain_error_response(e, &rid);
    }
    match repo_workspace::delete_workspace(pool.get_ref(), ws_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(DomainError::NotFound("workspace".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /projects per /docs/spec/api/http.md.
pub async fn list_projects(
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
        Err(_) => return domain_error_response(DomainError::BadRequest("workspace_id required".into()), &rid),
    };
    match repo_project::list_projects(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|p| serde_json::json!({
                    "id": p.id,
                    "workspace_id": p.workspace_id,
                    "name": p.name,
                    "description": p.description,
                    "archived": p.archived,
                    "created_at": p.created_at.to_string()
                }))
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /projects per /docs/spec/api/http.md.
pub async fn create_project(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id_str = body.get("workspace_id").and_then(|v| v.as_str()).unwrap_or("");
    let ws_id = match ws_id_str.parse::<Uuid>() {
        Ok(u) => WorkspaceId(u),
        Err(_) => return domain_error_response(DomainError::BadRequest("workspace_id required".into()), &rid),
    };
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let description = body.get("description").and_then(|v| v.as_str());
    match ws_svc::create_project(pool.get_ref(), ws_id, identity.user_id, name, description).await {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({
            "project_id": id.0, "request_id": rid
        })),
        Err(e) => domain_error_response(e, &rid),
    }
}
