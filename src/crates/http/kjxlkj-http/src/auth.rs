// Auth handlers per /docs/spec/security/auth.md, sessions.md
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::cookie::{Cookie, SameSite};
use kjxlkj_auth::{password, session as sess};
use kjxlkj_db::repo::{sessions, users};
use kjxlkj_domain::types::Session;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{ErrorBody, LoginRequest, SessionInfo};

/// POST /api/auth/login
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();

    let user = match users::find_by_email(pool.get_ref(), &body.email).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ErrorBody {
                code: "INVALID_CREDENTIALS".into(),
                message: "Invalid email or password".into(),
                details: None, request_id: rid,
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(), message: e.to_string(),
                details: None, request_id: rid,
            });
        }
    };

    // Check disabled
    if user.status == kjxlkj_domain::types::UserStatus::Disabled {
        return HttpResponse::Unauthorized().json(ErrorBody {
            code: "INVALID_CREDENTIALS".into(),
            message: "Account disabled".into(),
            details: None, request_id: rid,
        });
    }

    // Verify password
    if !password::verify_password(&body.password, &user.password_hash) {
        return HttpResponse::Unauthorized().json(ErrorBody {
            code: "INVALID_CREDENTIALS".into(),
            message: "Invalid email or password".into(),
            details: None, request_id: rid,
        });
    }

    // Create session
    let token = sess::generate_token();
    let csrf = sess::generate_csrf_token();
    let session = Session {
        id: sess::new_session_id(),
        user_id: user.id,
        token: token.clone(),
        expires_at: String::new(),
        created_at: String::new(),
    };

    if let Err(e) = sessions::insert_session(pool.get_ref(), &session, &csrf).await {
        return HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        });
    }

    // Set cookie per /docs/spec/security/sessions.md
    let cookie = Cookie::build("session", token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(SessionInfo {
            user_id: user.id,
            email: user.email,
            display_name: user.display_name,
            role: format!("{:?}", user.role).to_lowercase(),
            csrf_token: csrf,
        })
}

/// POST /api/auth/logout
pub async fn logout(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();

    let token = match req.cookie("session") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Unauthorized().json(ErrorBody {
                code: "AUTH_REQUIRED".into(),
                message: "No session".into(),
                details: None, request_id: rid,
            });
        }
    };

    if let Ok(Some((sid, _, _))) = sessions::find_by_token(pool.get_ref(), &token).await {
        let _ = sessions::revoke_session(pool.get_ref(), sid).await;
    }

    // Clear cookie
    let cookie = Cookie::build("session", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "logged_out"}))
}

/// GET /api/auth/session
pub async fn session_info(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();

    let token = match req.cookie("session") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Unauthorized().json(ErrorBody {
                code: "AUTH_REQUIRED".into(),
                message: "No session".into(),
                details: None, request_id: rid,
            });
        }
    };

    let (_, user_id, csrf) = match sessions::find_by_token(pool.get_ref(), &token).await {
        Ok(Some(s)) => s,
        _ => {
            return HttpResponse::Unauthorized().json(ErrorBody {
                code: "AUTH_REQUIRED".into(),
                message: "Invalid session".into(),
                details: None, request_id: rid,
            });
        }
    };

    let user = match users::find_by_id(pool.get_ref(), user_id).await {
        Ok(Some(u)) => u,
        _ => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: "User not found".into(),
                details: None, request_id: rid,
            });
        }
    };

    HttpResponse::Ok().json(SessionInfo {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        role: format!("{:?}", user.role).to_lowercase(),
        csrf_token: csrf,
    })
}
