use crate::app_state::AppState;
use crate::authn::require_identity;
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_auth::hash_password;
use kjxlkj_db::repos;
use kjxlkj_domain::Role;
use kjxlkj_rbac::ensure_global_role_update;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    email: String,
    password: String,
    display_name: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct UpdateRoleRequest {
    role: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(list_users))
        .route("/users", web::post().to(create_user))
        .route("/users/{id}/role", web::patch().to(update_role));
}

async fn list_users(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    if !matches!(identity.role, Role::Owner | Role::Admin) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN",
            "forbidden",
        )
        .with_request_id(request_id));
    }

    let users = repos::users::list_users(&state.pool)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "users": users
            .into_iter()
            .map(|user| json!({
                "id": user.id,
                "email": user.email,
                "display_name": user.display_name,
                "role": user.role,
                "status": user.status,
                "created_at": user.created_at.format(&Rfc3339).unwrap_or_else(|_| user.created_at.to_string()),
            }))
            .collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn create_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;

    if !matches!(identity.role, Role::Owner | Role::Admin) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN",
            "forbidden",
        )
        .with_request_id(request_id));
    }

    let role = Role::from_str(&body.role)
        .map_err(|_| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "RULE_INVALID", "invalid role"))?;
    let password_hash = hash_password(&body.password)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "password hash failed"))?;

    let user = repos::users::create_user(
        &state.pool,
        &body.email,
        &body.display_name,
        &password_hash,
        role.as_str(),
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid user payload"))?;

    repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        None,
        "user_created",
        json!({ "target_user_id": user.id, "role": user.role }),
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "audit failed"))?;

    Ok(HttpResponse::Created().json(json!({
        "id": user.id,
        "email": user.email,
        "display_name": user.display_name,
        "role": user.role,
        "status": user.status,
        "request_id": request_id,
    })))
}

async fn update_role(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;

    ensure_global_role_update(identity.role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let next_role = Role::from_str(&body.role)
        .map_err(|_| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "RULE_INVALID", "invalid role"))?;

    let user = repos::users::update_user_role(&state.pool, path.into_inner(), next_role.as_str())
        .await
        .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "USER_NOT_FOUND", "user not found"))?;

    repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        None,
        "user_role_updated",
        json!({ "target_user_id": user.id, "role": user.role }),
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "audit failed"))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": user.id,
        "role": user.role,
        "request_id": request_id,
    })))
}
