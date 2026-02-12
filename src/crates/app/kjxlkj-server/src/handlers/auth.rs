use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{
        auth_session, clear_session_cookie, create_session, enforce_auth_rate_limit, hash_password,
        revoke_session, set_session_cookie, verify_password,
    },
    error::AppError,
    models::{LoginRequest, SetupRegisterRequest},
};

pub async fn setup_register(
    state: web::Data<AppState>,
    payload: web::Json<SetupRegisterRequest>,
) -> Result<HttpResponse, AppError> {
    enforce_auth_rate_limit(&format!("setup:{}", payload.email.trim().to_lowercase()))?;
    if payload.email.trim().is_empty() || payload.password.len() < 8 {
        return Err(AppError::BadRequest("email/password invalid".to_string()));
    }

    let existing: i64 = sqlx::query_scalar("select count(*) from users")
        .fetch_one(&state.pool)
        .await?;
    if existing > 0 {
        return Err(AppError::Conflict("setup already completed".to_string()));
    }

    let user_id = Uuid::now_v7();
    let password_hash = hash_password(&payload.password)?;
    let display_name = payload
        .display_name
        .clone()
        .unwrap_or_else(|| payload.email.trim().to_string());
    sqlx::query(
        "insert into users (id, email, password_hash, display_name, role, status)
         values ($1, $2, $3, $4, 'owner', 'active')",
    )
    .bind(user_id)
    .bind(payload.email.trim().to_lowercase())
    .bind(password_hash)
    .bind(display_name)
    .execute(&state.pool)
    .await
    .map_err(|_| AppError::Conflict("email already exists".to_string()))?;

    // Bootstrap a default workspace so workspace-scoped APIs are immediately usable.
    let workspace_id = Uuid::now_v7();
    sqlx::query("insert into workspaces (id, slug, name, owner_user_id) values ($1, $2, $3, $4)")
        .bind(workspace_id)
        .bind("main")
        .bind("Main Workspace")
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| AppError::Conflict("default workspace bootstrap failed".to_string()))?;
    sqlx::query(
        "insert into workspace_members (workspace_id, user_id, role) values ($1, $2, 'owner')
         on conflict (workspace_id, user_id) do update set role = excluded.role",
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(&state.pool)
    .await?;

    let session = create_session(&state.pool, user_id, state.config.session_ttl_days).await?;
    let sid: Uuid = sqlx::query_scalar(
        "select id from sessions where user_id = $1 order by created_at desc limit 1",
    )
    .bind(user_id)
    .fetch_one(&state.pool)
    .await?;
    let mut resp = HttpResponse::Created();
    set_session_cookie(&mut resp, sid);
    Ok(resp.json(session))
}

pub async fn login(
    state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    enforce_auth_rate_limit(&format!("login:{}", payload.email.trim().to_lowercase()))?;
    let row = sqlx::query_as::<_, (Uuid, String, String)>(
        "select id, password_hash, status from users where email = $1",
    )
    .bind(payload.email.trim().to_lowercase())
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::AuthRequired)?;

    if row.2 != "active" || !verify_password(&payload.password, &row.1)? {
        return Err(AppError::AuthRequired);
    }

    let session = create_session(&state.pool, row.0, state.config.session_ttl_days).await?;
    let sid: Uuid = sqlx::query_scalar(
        "select id from sessions where user_id = $1 order by created_at desc limit 1",
    )
    .bind(row.0)
    .fetch_one(&state.pool)
    .await?;
    let mut resp = HttpResponse::Ok();
    set_session_cookie(&mut resp, sid);
    Ok(resp.json(session))
}

pub async fn logout(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    revoke_session(&state.pool, session.session_id).await?;
    let mut resp = HttpResponse::NoContent();
    clear_session_cookie(&mut resp);
    Ok(resp.finish())
}

pub async fn current_session(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let expires_at: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("select expires_at from sessions where id = $1")
            .bind(session.session_id)
            .fetch_one(&state.pool)
            .await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": session.user_id,
        "email": session.email,
        "csrf_token": session.csrf_token,
        "expires_at": expires_at,
    })))
}
