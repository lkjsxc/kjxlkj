use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateProjectRequest, UpdateProjectRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_rbac::{guard, roles};
use kjxlkj_workspace::project_service;

/// GET /api/projects?workspace_id=...
pub async fn list_projects(
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
        None => {
            return domain_error_response(&kjxlkj_domain::errors::DomainError::BadRequest {
                reason: "workspace_id required".into(),
            })
        }
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_view_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match project_service::list_projects(pool.get_ref(), ws_id).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/projects
pub async fn create_project(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), body.workspace_id, auth.user_id, roles::can_manage_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match project_service::create_project(pool.get_ref(), body.workspace_id, &body.name).await {
        Ok(p) => HttpResponse::Created().json(p),
        Err(e) => domain_error_response(&e),
    }
}

/// PATCH /api/projects/{id}
pub async fn update_project(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateProjectRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match project_service::update_project(pool.get_ref(), id, &body.name).await {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => domain_error_response(&e),
    }
}

/// DELETE /api/projects/{id}
pub async fn delete_project(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match project_service::delete_project(pool.get_ref(), id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&e),
    }
}
