//! User management handlers per /docs/spec/api/http.md users section.

use crate::dto::{ApiError, CreateUserReq, UpdateRoleReq, UserResp};
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// GET /api/users
pub async fn list(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_admin_users(role) {
        return middleware::forbidden();
    }
    match kjxlkj_db::repo::user::list_users(pool.get_ref()).await {
        Ok(rows) => {
            let users: Vec<UserResp> = rows
                .into_iter()
                .map(|u| UserResp {
                    id: u.id,
                    email: u.email,
                    display_name: u.display_name,
                    role: u.role,
                    status: u.status,
                    created_at: u.created_at,
                })
                .collect();
            HttpResponse::Ok().json(users)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/users
pub async fn create(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateUserReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_admin_users(role) {
        return middleware::forbidden();
    }
    let new_role = body.role.as_deref().unwrap_or("editor");
    if kjxlkj_rbac::parse_role(new_role).is_none() {
        return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "invalid role"));
    }
    let hash = match kjxlkj_auth::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e));
        }
    };
    let uid = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::user::create_user(
        pool.get_ref(),
        uid,
        &body.email,
        &body.display_name,
        new_role,
        &hash,
    )
    .await
    {
        Ok(()) => HttpResponse::Created().json(UserResp {
            id: uid,
            email: body.email.clone(),
            display_name: body.display_name.clone(),
            role: new_role.to_string(),
            status: "active".to_string(),
            created_at: String::new(),
        }),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/users/{id}/role
pub async fn update_role(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRoleReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_admin_users(role) {
        return middleware::forbidden();
    }
    if kjxlkj_rbac::parse_role(&body.role).is_none() {
        return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "invalid role"));
    }
    let uid = path.into_inner();
    match kjxlkj_db::repo::user::update_role(pool.get_ref(), uid, &body.role).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"id": uid, "role": body.role})),
        Ok(false) => middleware::not_found("user"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/users/{id}
pub async fn delete(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_admin_users(role) {
        return middleware::forbidden();
    }
    let uid = path.into_inner();
    match kjxlkj_db::repo::user::disable_user(pool.get_ref(), uid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("user"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
