// Setup handler per /docs/spec/security/auth.md
use actix_web::{web, HttpResponse};
use kjxlkj_auth::password;
use kjxlkj_db::repo::users;
use kjxlkj_domain::types::{Role, User, UserStatus};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{ErrorBody, RegisterRequest};

/// POST /api/setup/register — first-run owner creation
/// Setup MUST lock after first owner per spec.
pub async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();

    // Check if setup is locked
    match users::owner_exists(pool.get_ref()).await {
        Ok(true) => {
            return HttpResponse::Conflict().json(ErrorBody {
                code: "SETUP_LOCKED".into(),
                message: "Setup already completed".into(),
                details: None,
                request_id: rid,
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: e.to_string(),
                details: None,
                request_id: rid,
            });
        }
        _ => {}
    }

    // Hash password
    let hash = match password::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: e,
                details: None,
                request_id: rid,
            });
        }
    };

    let user = User {
        id: Uuid::now_v7(),
        email: body.email.clone(),
        display_name: body.display_name.clone(),
        role: Role::Owner,
        status: UserStatus::Active,
        password_hash: hash,
        created_at: String::new(),
    };

    match users::insert_user(pool.get_ref(), &user).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "role": "owner"
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        }),
    }
}
