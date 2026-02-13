use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateWorkspaceRequest, UpdateWorkspaceRequest, UpsertMemberRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::WorkspaceRole;
use kjxlkj_rbac::roles;
use kjxlkj_workspace::workspace_service;

/// GET /api/workspaces
pub async fn list_workspaces(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    match workspace_service::list_workspaces(pool.get_ref()).await {
        Ok(ws) => HttpResponse::Ok().json(ws),
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/workspaces
pub async fn create_workspace(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateWorkspaceRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_create_workspace(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    match workspace_service::create_workspace(
        pool.get_ref(),
        &body.name,
        &body.slug,
        auth.user_id,
    )
    .await
    {
        Ok(ws) => HttpResponse::Created().json(ws),
        Err(e) => domain_error_response(&e),
    }
}

/// PATCH /api/workspaces/{id}
pub async fn update_workspace(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateWorkspaceRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = path.into_inner();
    if let Err(e) =
        kjxlkj_rbac::guard::require_workspace_role(
            pool.get_ref(), ws_id, auth.user_id, roles::can_manage_workspace,
        ).await
    {
        return domain_error_response(&e);
    }
    let name = body.name.as_deref().unwrap_or("");
    match workspace_service::update_workspace(
        pool.get_ref(), ws_id, name,
    ).await {
        Ok(ws) => HttpResponse::Ok().json(ws),
        Err(e) => domain_error_response(&e),
    }
}

/// DELETE /api/workspaces/{id}
pub async fn delete_workspace(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = path.into_inner();
    if let Err(e) = kjxlkj_rbac::guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_manage_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match workspace_service::delete_workspace(pool.get_ref(), ws_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&e),
    }
}

/// GET /api/workspaces/{id}/members
pub async fn list_members(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = path.into_inner();
    if let Err(e) = kjxlkj_rbac::guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_view_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match repos::workspaces::list_members(pool.get_ref(), ws_id).await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// PUT /api/workspaces/{id}/members/{user_id}
pub async fn upsert_member(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    body: web::Json<UpsertMemberRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let (ws_id, member_id) = path.into_inner();
    if let Err(e) = kjxlkj_rbac::guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_manage_members,
    ).await {
        return domain_error_response(&e);
    }
    let role: WorkspaceRole = match body.role.as_str() {
        "owner" => WorkspaceRole::Owner,
        "admin" => WorkspaceRole::Admin,
        "editor" => WorkspaceRole::Editor,
        "viewer" => WorkspaceRole::Viewer,
        _ => {
            return domain_error_response(&DomainError::BadRequest {
                reason: "invalid role".into(),
            })
        }
    };
    match repos::workspaces::upsert_member(pool.get_ref(), ws_id, member_id, role).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"ok": true})),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}
