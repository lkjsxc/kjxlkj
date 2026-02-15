use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_auth::password;
use kjxlkj_db::repo_user;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::UserId;
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateUserRequest, UserResponse};
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /users per /docs/spec/api/http.md.
pub async fn list_users(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }

    match repo_user::list_users(pool.get_ref()).await {
        Ok(users) => {
            let list: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    email: u.email,
                    display_name: u.display_name,
                    role: u.role,
                    status: u.status,
                    created_at: u.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /users per /docs/spec/api/http.md.
pub async fn create_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };

    // Per /docs/spec/domain/permissions.md: user management requires admin+
    let actor_user = match repo_user::find_user_by_id(pool.get_ref(), identity.user_id).await {
        Ok(Some(u)) => u,
        _ => return domain_error_response(DomainError::AuthRequired, &rid),
    };
    let actor_role = match guard::parse_role(&actor_user.role) {
        Some(r) => r,
        None => return domain_error_response(DomainError::Internal("bad role".into()), &rid),
    };
    if let Err(e) = guard::require_admin(actor_role) {
        return domain_error_response(e, &rid);
    }

    let role = match guard::parse_role(&body.role) {
        Some(r) => r,
        None => {
            return domain_error_response(
                DomainError::BadRequest("invalid role".into()),
                &rid,
            )
        }
    };

    let hash = match password::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => return domain_error_response(DomainError::Internal(e.to_string()), &rid),
    };

    let user_id = UserId(Uuid::now_v7());
    match repo_user::create_user(
        pool.get_ref(),
        user_id,
        &body.email,
        &body.display_name,
        role,
        &hash,
    )
    .await
    {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "user_id": user_id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// PATCH /users/{id}/role per /docs/spec/api/http.md.
pub async fn update_role(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };

    let actor_user = match repo_user::find_user_by_id(pool.get_ref(), identity.user_id).await {
        Ok(Some(u)) => u,
        _ => return domain_error_response(DomainError::AuthRequired, &rid),
    };
    let actor_role = match guard::parse_role(&actor_user.role) {
        Some(r) => r,
        None => return domain_error_response(DomainError::Internal("bad role".into()), &rid),
    };
    if let Err(e) = guard::require_admin(actor_role) {
        return domain_error_response(e, &rid);
    }

    let target_id = UserId(path.into_inner());
    let new_role_str = body
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let new_role = match guard::parse_role(new_role_str) {
        Some(r) => r,
        None => {
            return domain_error_response(
                DomainError::BadRequest("invalid role".into()),
                &rid,
            )
        }
    };

    match repo_user::update_user_role(pool.get_ref(), target_id, new_role).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "updated",
            "request_id": rid
        })),
        Ok(false) => domain_error_response(DomainError::NotFound("user".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// DELETE /users/{id} per /docs/spec/api/http.md.
pub async fn delete_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };

    let actor_user = match repo_user::find_user_by_id(pool.get_ref(), identity.user_id).await {
        Ok(Some(u)) => u,
        _ => return domain_error_response(DomainError::AuthRequired, &rid),
    };
    let actor_role = match guard::parse_role(&actor_user.role) {
        Some(r) => r,
        None => return domain_error_response(DomainError::Internal("bad role".into()), &rid),
    };
    if let Err(e) = guard::require_admin(actor_role) {
        return domain_error_response(e, &rid);
    }

    let target_id = UserId(path.into_inner());
    match repo_user::disable_user(pool.get_ref(), target_id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "disabled",
            "request_id": rid
        })),
        Ok(false) => domain_error_response(DomainError::NotFound("user".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}
