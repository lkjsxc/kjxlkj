use actix_web::{web, HttpRequest, HttpResponse, cookie::Cookie};
use serde::Deserialize;
use sqlx::PgPool;
use kjxlkj_auth::{password, session, setup};
use kjxlkj_db::repo_user;
use kjxlkj_domain::error::ErrorCode;
use crate::extract::{self, SESSION_COOKIE};
use crate::response::error_response;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub display_name: Option<String>,
    pub password: String,
}

/// POST /api/setup/register - first-run owner bootstrap.
pub async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let display = body.display_name.clone().unwrap_or_default();
    match setup::register_owner(&pool, &body.username, &display, &body.password).await {
        Ok(user_id) => HttpResponse::Created().json(serde_json::json!({
            "user_id": user_id,
            "username": body.username,
        })),
        Err(setup::SetupError::Locked) => {
            error_response(ErrorCode::SetupLocked, "setup already complete")
        }
        Err(_) => {
            error_response(ErrorCode::InternalError, "registration failed")
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// POST /api/auth/login - create session.
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let user = match repo_user::find_user_by_username(&pool, &body.username).await {
        Ok(Some(u)) => u,
        _ => return error_response(ErrorCode::InvalidCredentials, "invalid credentials"),
    };

    if user.is_disabled {
        return error_response(ErrorCode::InvalidCredentials, "account disabled");
    }

    let valid = match password::verify_password(&body.password, &user.password_hash) {
        Ok(v) => v,
        Err(_) => return error_response(ErrorCode::InternalError, "verification failed"),
    };

    if !valid {
        return error_response(ErrorCode::InvalidCredentials, "invalid credentials");
    }

    match session::create(&pool, user.id).await {
        Ok((token, session_id, csrf)) => {
            let cookie = Cookie::build(SESSION_COOKIE, &token)
                .path("/")
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Lax)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(serde_json::json!({
                    "session_id": session_id,
                    "user_id": user.id,
                    "username": user.username,
                    "csrf_token": csrf,
                }))
        }
        Err(_) => error_response(ErrorCode::InternalError, "session creation failed"),
    }
}

/// POST /api/auth/logout - revoke current session.
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if let Some(identity) = extract::extract_identity(&req, &pool).await {
        let _ = session::revoke(&pool, identity.session_id).await;
    }
    let mut cookie = Cookie::build(SESSION_COOKIE, "")
        .path("/")
        .http_only(true)
        .finish();
    cookie.make_removal();
    HttpResponse::Ok().cookie(cookie).json(serde_json::json!({"ok": true}))
}

/// GET /api/auth/session - current session identity.
pub async fn get_session(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match extract::extract_identity(&req, &pool).await {
        Some(identity) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": identity.user_id,
            "username": identity.username,
            "is_owner": identity.is_owner,
            "csrf_token": identity.csrf_token,
        })),
        None => error_response(ErrorCode::AuthRequired, "not authenticated"),
    }
}
