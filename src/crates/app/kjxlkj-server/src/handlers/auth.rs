use crate::app_state::AppState;
use crate::authn::{client_key, require_identity, SESSION_COOKIE};
use crate::error::{new_request_id, ApiError};
use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry, verify_password};
use kjxlkj_db::repos;
use kjxlkj_domain::UserStatus;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct SetupRegisterRequest {
    email: String,
    password: String,
    display_name: String,
    workspace_name: Option<String>,
    workspace_slug: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/setup/register", web::post().to(setup_register))
        .route("/auth/login", web::post().to(login))
        .route("/auth/logout", web::post().to(logout))
        .route("/auth/session", web::get().to(session));
}

async fn setup_register(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SetupRegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_auth_rate_limit(&state, &req, "setup", &request_id)?;

    if repos::auth::owner_exists(&state.pool)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "SETUP_LOCKED",
            "setup already completed",
        )
        .with_request_id(request_id));
    }

    let password_hash = hash_password(&body.password).map_err(|_| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "password hashing failed",
        )
        .with_request_id(request_id.clone())
    })?;

    let workspace_name = body
        .workspace_name
        .clone()
        .unwrap_or_else(|| "Main Workspace".to_owned());
    let workspace_slug = body
        .workspace_slug
        .clone()
        .unwrap_or_else(|| slugify(&workspace_name));

    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &state.pool,
        &body.email,
        &body.display_name,
        &password_hash,
        &workspace_slug,
        &workspace_name,
    )
    .await
    .map_err(|_| {
        ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid setup payload")
            .with_request_id(request_id.clone())
    })?;

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(
        &state.pool,
        session_id,
        owner.id,
        &csrf_token,
        session_expiry(7),
    )
    .await
    .map_err(|_| {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "session creation failed")
            .with_request_id(request_id.clone())
    })?;

    repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(owner.id),
        Some(workspace.id),
        "owner_bootstrap_completed",
        json!({ "email": owner.email }),
    )
    .await
    .map_err(|_| {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "audit event failed")
            .with_request_id(request_id.clone())
    })?;

    Ok(HttpResponse::Created()
        .cookie(session_cookie(session_id, state.secure_cookies))
        .json(json!({
            "user_id": owner.id,
            "workspace_id": workspace.id,
            "role": owner.role,
            "csrf_token": csrf_token,
            "request_id": request_id,
        })))
}

async fn login(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_auth_rate_limit(&state, &req, "login", &request_id)?;

    let user = repos::auth::find_user_by_email(&state.pool, &body.email)
        .await
        .map_err(|_| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error")
                .with_request_id(request_id.clone())
        })?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "invalid credentials",
            )
            .with_request_id(request_id.clone())
        })?;

    let verified = verify_password(&user.password_hash, &body.password).map_err(|_| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "invalid credentials",
        )
        .with_request_id(request_id.clone())
    })?;

    if !verified || user.status != UserStatus::Active.as_str() {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "invalid credentials",
        )
        .with_request_id(request_id));
    }

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&state.pool, session_id, user.id, &csrf_token, session_expiry(7))
        .await
        .map_err(|_| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "session creation failed")
                .with_request_id(request_id.clone())
        })?;

    Ok(HttpResponse::Ok()
        .cookie(session_cookie(session_id, state.secure_cookies))
        .json(json!({
            "user_id": user.id,
            "role": user.role,
            "csrf_token": csrf_token,
            "request_id": request_id,
        })))
}

async fn logout(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true)
        .await?
        .clone();

    repos::auth::revoke_session(&state.pool, identity.session_id)
        .await
        .map_err(|_| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "logout failed")
                .with_request_id(request_id.clone())
        })?;

    Ok(HttpResponse::NoContent()
        .cookie(clear_session_cookie(state.secure_cookies))
        .finish())
}

async fn session(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    Ok(HttpResponse::Ok().json(json!({
        "user_id": identity.user_id,
        "email": identity.email,
        "display_name": identity.display_name,
        "role": identity.role.as_str(),
        "csrf_token": identity.csrf_token,
        "request_id": request_id,
    })))
}

fn enforce_auth_rate_limit(
    state: &AppState,
    req: &HttpRequest,
    bucket: &str,
    request_id: &str,
) -> Result<(), ApiError> {
    let key = format!("{}:{}", client_key(req), bucket);
    if state.auth_rate_limiter.check(&key) {
        return Ok(());
    }

    Err(ApiError::new(
        StatusCode::TOO_MANY_REQUESTS,
        "RATE_LIMITED",
        "rate limited",
    )
    .with_request_id(request_id.to_owned()))
}

fn session_cookie(session_id: uuid::Uuid, secure: bool) -> Cookie<'static> {
    let mut builder = Cookie::build(SESSION_COOKIE, session_id.to_string())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax);

    if secure {
        builder = builder.secure(true);
    }

    builder.finish()
}

fn clear_session_cookie(secure: bool) -> Cookie<'static> {
    let mut builder = Cookie::build(SESSION_COOKIE, "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(0));

    if secure {
        builder = builder.secure(true);
    }

    builder.finish()
}

fn slugify(input: &str) -> String {
    let slug = input
        .chars()
        .flat_map(|c| c.to_lowercase())
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();
    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "workspace".to_owned()
    } else {
        trimmed.to_owned()
    }
}
