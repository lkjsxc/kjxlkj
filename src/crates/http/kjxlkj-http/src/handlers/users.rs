use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateUserRequest, UpdateRoleRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_auth::password;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::GlobalRole;
use kjxlkj_rbac::roles;

/// GET /api/users
pub async fn list_users(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    match repos::users::list(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// POST /api/users
pub async fn create_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let role: GlobalRole = match body.global_role.as_str() {
        "owner" => GlobalRole::Owner,
        "admin" => GlobalRole::Admin,
        "editor" => GlobalRole::Editor,
        "viewer" => GlobalRole::Viewer,
        _ => {
            return domain_error_response(&DomainError::BadRequest {
                reason: "invalid role".into(),
            })
        }
    };
    if !roles::can_assign_global_role(auth.user.global_role, role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let hash = match password::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => return domain_error_response(&e),
    };
    let user_id = uuid::Uuid::new_v4();
    match repos::users::create(pool.get_ref(), user_id, &body.username, &hash, role).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// PATCH /api/users/{id}/role
pub async fn update_role(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateRoleRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let target_role: GlobalRole = match body.role.as_str() {
        "owner" => GlobalRole::Owner,
        "admin" => GlobalRole::Admin,
        "editor" => GlobalRole::Editor,
        "viewer" => GlobalRole::Viewer,
        _ => {
            return domain_error_response(&DomainError::BadRequest {
                reason: "invalid role".into(),
            })
        }
    };
    if !roles::can_assign_global_role(auth.user.global_role, target_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let user_id = path.into_inner();
    match repos::users::update_role(pool.get_ref(), user_id, target_role).await {
        Ok(Some(u)) => HttpResponse::Ok().json(u),
        Ok(None) => domain_error_response(&DomainError::NotFound {
            entity: "user".into(),
        }),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// DELETE /api/users/{id}
pub async fn disable_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if !roles::can_manage_users(auth.user.global_role) {
        return domain_error_response(&DomainError::RoleForbidden);
    }
    let user_id = path.into_inner();
    match repos::users::disable(pool.get_ref(), user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}
