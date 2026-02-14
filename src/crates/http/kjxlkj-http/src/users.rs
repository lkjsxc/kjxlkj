// User handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::{require_role, AuthSession};
use kjxlkj_auth::password;
use kjxlkj_db::repo::users;
use kjxlkj_domain::types::{Role, User, UserStatus};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateUserRequest, ErrorBody, UpdateRoleRequest};

/// GET /api/users — admin+ only
pub async fn list(pool: web::Data<PgPool>, auth: AuthSession) -> HttpResponse {
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: Uuid::now_v7().to_string(),
        });
    }
    match users::list_users(pool.get_ref()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// POST /api/users — admin+ only
pub async fn create(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: rid,
        });
    }
    let role = match body.role.as_deref() {
        Some("admin") => Role::Admin,
        Some("editor") => Role::Editor,
        Some("viewer") | None => Role::Viewer,
        _ => {
            return HttpResponse::BadRequest().json(ErrorBody {
                code: "BAD_REQUEST".into(), message: "Invalid role".into(),
                details: None, request_id: rid,
            });
        }
    };

    let hash = match password::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(), message: e,
                details: None, request_id: rid,
            });
        }
    };

    let user = User {
        id: Uuid::now_v7(),
        email: body.email.clone(),
        display_name: body.display_name.clone(),
        role,
        status: UserStatus::Active,
        password_hash: hash,
        created_at: String::new(),
    };

    match users::insert_user(pool.get_ref(), &user).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "role": format!("{:?}", user.role).to_lowercase()
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/users/{id}/role — owner only
pub async fn update_role(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRoleRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Owner) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Owner role required".into(),
            details: None, request_id: rid,
        });
    }
    let role = match body.role.as_str() {
        "admin" => Role::Admin,
        "editor" => Role::Editor,
        "viewer" => Role::Viewer,
        _ => {
            return HttpResponse::BadRequest().json(ErrorBody {
                code: "BAD_REQUEST".into(), message: "Invalid role".into(),
                details: None, request_id: rid,
            });
        }
    };

    match users::update_role(pool.get_ref(), path.into_inner(), role).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "User not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/users/{id} — owner only
pub async fn delete(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Owner) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Owner role required".into(),
            details: None, request_id: rid,
        });
    }
    match users::disable_user(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "User not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}
