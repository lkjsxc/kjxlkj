use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{LoginRequest, RegisterRequest, SessionResponse};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_auth::{password, session, setup};
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;

/// POST /api/setup/register — first-run owner bootstrap.
pub async fn setup_register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    match setup::register_owner(pool.get_ref(), &body.username, &body.password).await {
        Ok(token) => {
            let cookie = build_session_cookie(&token);
            HttpResponse::Created()
                .cookie(cookie)
                .json(serde_json::json!({"ok": true}))
        }
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/auth/login
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let user = match repos::users::find_by_username(pool.get_ref(), &body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return domain_error_response(&DomainError::InvalidCredentials),
        Err(e) => return domain_error_response(&DomainError::Internal(e.to_string())),
    };
    if user.disabled {
        return domain_error_response(&DomainError::InvalidCredentials);
    }
    match password::verify_password(&body.password, &user.password_hash) {
        Ok(true) => {}
        _ => return domain_error_response(&DomainError::InvalidCredentials),
    }
    match session::create_session(pool.get_ref(), user.id).await {
        Ok(token) => {
            let cookie = build_session_cookie(&token);
            HttpResponse::Ok()
                .cookie(cookie)
                .json(serde_json::json!({"ok": true}))
        }
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/auth/logout
pub async fn logout(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    match get_auth_user(pool.get_ref(), &req).await {
        Ok(auth) => {
            let _ = session::revoke_session(pool.get_ref(), auth.session_id).await;
            let mut cookie = Cookie::named("session");
            cookie.make_removal();
            HttpResponse::Ok().cookie(cookie).json(serde_json::json!({"ok": true}))
        }
        Err(e) => domain_error_response(&e),
    }
}

/// GET /api/auth/session
pub async fn get_session(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    match get_auth_user(pool.get_ref(), &req).await {
        Ok(auth) => HttpResponse::Ok().json(SessionResponse {
            user_id: auth.user_id,
            username: auth.user.username.clone(),
            global_role: format!("{:?}", auth.user.global_role).to_lowercase(),
        }),
        Err(e) => domain_error_response(&e),
    }
}

fn build_session_cookie(token: &str) -> Cookie<'static> {
    Cookie::build("session", token.to_string())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish()
}
