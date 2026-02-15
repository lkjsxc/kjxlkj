use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_auth::{password, session, setup};
use kjxlkj_db::repo_user;
use kjxlkj_domain::error::DomainError;
use sqlx::PgPool;

use crate::dto::{LoginRequest, SessionResponse, SetupRegisterRequest};
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::{extract_session, SESSION_COOKIE};

/// POST /setup/register per /docs/spec/security/auth.md.
pub async fn setup_register(
    pool: web::Data<PgPool>,
    body: web::Json<SetupRegisterRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    match setup::register_owner(
        pool.get_ref(),
        &body.email,
        &body.display_name,
        &body.password,
    )
    .await
    {
        Ok(user_id) => HttpResponse::Created().json(serde_json::json!({
            "user_id": user_id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(e, &rid),
    }
}

/// POST /auth/login per /docs/spec/security/auth.md.
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let rid = new_request_id();

    let user = match repo_user::find_user_by_email(pool.get_ref(), &body.email).await {
        Ok(Some(u)) => u,
        Ok(None) => return domain_error_response(DomainError::InvalidCredentials, &rid),
        Err(e) => {
            return domain_error_response(DomainError::Internal(e.to_string()), &rid)
        }
    };

    if user.status == "disabled" {
        return domain_error_response(DomainError::InvalidCredentials, &rid);
    }

    let valid = match password::verify_password(&body.password, &user.password_hash) {
        Ok(v) => v,
        Err(e) => {
            return domain_error_response(DomainError::Internal(e.to_string()), &rid)
        }
    };

    if !valid {
        return domain_error_response(DomainError::InvalidCredentials, &rid);
    }

    let user_id = kjxlkj_domain::ids::UserId(user.id);
    match session::create_session(pool.get_ref(), user_id).await {
        Ok((session_id, csrf_token)) => {
            let cookie = Cookie::build(SESSION_COOKIE, session_id.0.to_string())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .max_age(actix_web::cookie::time::Duration::days(7))
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(SessionResponse {
                    user_id: user.id,
                    email: user.email,
                    display_name: user.display_name,
                    role: user.role,
                    csrf_token,
                })
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /auth/logout per /docs/spec/security/auth.md.
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let rid = new_request_id();
    match extract_session(&req, pool.get_ref()).await {
        Ok(identity) => {
            let _ = session::revoke_session(pool.get_ref(), identity.session_id).await;
            let mut cookie = Cookie::build(SESSION_COOKIE, "")
                .path("/")
                .http_only(true)
                .finish();
            cookie.make_removal();
            HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
                "status": "logged_out",
                "request_id": rid
            }))
        }
        Err(e) => domain_error_response(e, &rid),
    }
}

/// GET /auth/session per /docs/spec/api/http.md.
pub async fn get_session(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let rid = new_request_id();
    match extract_session(&req, pool.get_ref()).await {
        Ok(identity) => {
            match repo_user::find_user_by_id(pool.get_ref(), identity.user_id).await {
                Ok(Some(user)) => HttpResponse::Ok().json(SessionResponse {
                    user_id: user.id,
                    email: user.email,
                    display_name: user.display_name,
                    role: user.role,
                    csrf_token: identity.csrf_token,
                }),
                Ok(None) => domain_error_response(DomainError::AuthRequired, &rid),
                Err(e) => {
                    domain_error_response(DomainError::Internal(e.to_string()), &rid)
                }
            }
        }
        Err(e) => domain_error_response(e, &rid),
    }
}
