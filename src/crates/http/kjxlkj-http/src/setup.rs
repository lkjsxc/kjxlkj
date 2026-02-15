//! Setup, auth, and session handlers per /docs/spec/api/http.md
//! and /docs/spec/security/auth.md.

use crate::dto::{ApiError, LoginReq, RegisterReq, SessionResp};
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

/// POST /api/setup/register — first-run owner bootstrap.
/// Per /docs/spec/security/auth.md: enabled only when no owner exists.
pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<RegisterReq>,
) -> HttpResponse {
    // Check setup lock
    match kjxlkj_db::repo::user::owner_exists(pool.get_ref()).await {
        Ok(true) => {
            return HttpResponse::Conflict()
                .json(ApiError::new("SETUP_LOCKED", "setup already completed"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
        }
        _ => {}
    }
    // Hash password
    let hash = match kjxlkj_auth::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e));
        }
    };
    let user_id = kjxlkj_domain::types::new_id();
    if let Err(e) = kjxlkj_db::repo::user::create_user(
        pool.get_ref(),
        user_id,
        &body.email,
        &body.display_name,
        "owner",
        &hash,
    )
    .await
    {
        return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
    }
    // Create session
    let session_id = kjxlkj_domain::types::new_id();
    let csrf = kjxlkj_auth::generate_csrf_token();
    let expires = kjxlkj_auth::session_expiry();
    if let Err(e) = kjxlkj_db::repo::session::create_session(
        pool.get_ref(),
        session_id,
        user_id,
        &csrf,
        &expires,
    )
    .await
    {
        return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
    }
    let cookie = middleware::make_session_cookie(session_id, &config);
    HttpResponse::Created()
        .cookie(cookie)
        .json(SessionResp {
            user_id,
            email: body.email.clone(),
            display_name: body.display_name.clone(),
            role: "owner".to_string(),
            csrf_token: csrf,
        })
}

/// POST /api/auth/login — create authenticated session.
pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<LoginReq>,
) -> HttpResponse {
    let user = match kjxlkj_db::repo::user::find_by_email(pool.get_ref(), &body.email).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .json(ApiError::new("INVALID_CREDENTIALS", "invalid credentials"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
        }
    };
    // Disabled users rejected per /docs/spec/security/auth.md
    if user.status == "disabled" {
        return HttpResponse::Unauthorized()
            .json(ApiError::new("INVALID_CREDENTIALS", "account disabled"));
    }
    if !kjxlkj_auth::verify_password(&body.password, &user.password_hash) {
        return HttpResponse::Unauthorized()
            .json(ApiError::new("INVALID_CREDENTIALS", "invalid credentials"));
    }
    let session_id = kjxlkj_domain::types::new_id();
    let csrf = kjxlkj_auth::generate_csrf_token();
    let expires = kjxlkj_auth::session_expiry();
    if let Err(e) = kjxlkj_db::repo::session::create_session(
        pool.get_ref(),
        session_id,
        user.id,
        &csrf,
        &expires,
    )
    .await
    {
        return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
    }
    let cookie = middleware::make_session_cookie(session_id, &config);
    HttpResponse::Ok().cookie(cookie).json(SessionResp {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        role: user.role,
        csrf_token: csrf,
    })
}

/// POST /api/auth/logout — revoke active session.
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if let Some(ctx) = middleware::extract_session(&req, pool.get_ref()).await {
        let _ = kjxlkj_db::repo::session::delete_session(
            pool.get_ref(),
            ctx.session_id,
        )
        .await;
    }
    HttpResponse::Ok()
        .cookie(middleware::expire_session_cookie())
        .json(serde_json::json!({"status": "logged_out"}))
}

/// GET /api/auth/session — return current session identity.
/// Per /docs/spec/ui/web-app.md: MAY return 401 before login.
pub async fn session_info(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match middleware::extract_session(&req, pool.get_ref()).await {
        Some(ctx) => HttpResponse::Ok().json(SessionResp {
            user_id: ctx.user_id,
            email: ctx.email,
            display_name: ctx.display_name,
            role: ctx.role,
            csrf_token: ctx.csrf_token,
        }),
        None => HttpResponse::Unauthorized()
            .json(ApiError::new("AUTH_REQUIRED", "not authenticated")),
    }
}
