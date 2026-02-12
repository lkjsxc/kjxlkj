use std::{
    collections::{HashMap, VecDeque},
    sync::{Mutex, OnceLock},
    time::{Duration as StdDuration, Instant},
};

use actix_web::{
    cookie::{Cookie, SameSite},
    HttpRequest, HttpResponseBuilder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration as ChronoDuration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app_state::AppState, error::AppError, models::SessionResponse};

const COOKIE_NAME: &str = "kjxlkj_session";
const AUTH_RATE_LIMIT_MAX: usize = 8;
const AUTH_RATE_LIMIT_WINDOW_SECONDS: u64 = 60;

static AUTH_RATE_LIMITER: OnceLock<Mutex<HashMap<String, VecDeque<Instant>>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub csrf_token: String,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    ttl_days: i64,
) -> Result<SessionResponse, AppError> {
    let session_id = Uuid::now_v7();
    let csrf_token = Uuid::now_v7().to_string();
    let expires_at = Utc::now() + ChronoDuration::days(ttl_days);
    sqlx::query(
        "insert into sessions (id, user_id, csrf_token, expires_at) values ($1, $2, $3, $4)",
    )
    .bind(session_id)
    .bind(user_id)
    .bind(&csrf_token)
    .bind(expires_at)
    .execute(pool)
    .await?;

    let row = sqlx::query_as::<_, (String,)>("select email from users where id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(SessionResponse {
        user_id,
        email: row.0,
        csrf_token,
        expires_at,
    })
}

pub fn set_session_cookie(resp: &mut HttpResponseBuilder, session_id: Uuid) {
    let cookie = Cookie::build(COOKIE_NAME, session_id.to_string())
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();
    resp.cookie(cookie);
}

pub fn clear_session_cookie(resp: &mut HttpResponseBuilder) {
    let mut cookie = Cookie::build(COOKIE_NAME, "").path("/").finish();
    cookie.make_removal();
    resp.cookie(cookie);
}

pub async fn auth_session(req: &HttpRequest, state: &AppState) -> Result<AuthSession, AppError> {
    let sid = req
        .cookie(COOKIE_NAME)
        .ok_or(AppError::AuthRequired)?
        .value()
        .parse::<Uuid>()
        .map_err(|_| AppError::AuthRequired)?;

    let row = sqlx::query_as::<
        _,
        (
            Uuid,
            Uuid,
            String,
            chrono::DateTime<Utc>,
            String,
            String,
            String,
        ),
    >(
        "select s.id, s.user_id, s.csrf_token, s.expires_at, u.email, u.role, u.status
         from sessions s join users u on s.user_id = u.id where s.id = $1",
    )
    .bind(sid)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::AuthRequired)?;

    if row.3 <= Utc::now() {
        return Err(AppError::AuthRequired);
    }
    if row.6 != "active" {
        return Err(AppError::AuthRequired);
    }

    let expires_at = Utc::now() + chrono::Duration::days(state.config.session_ttl_days);
    sqlx::query("update sessions set expires_at = $2, last_seen_at = now() where id = $1")
        .bind(sid)
        .bind(expires_at)
        .execute(&state.pool)
        .await?;

    Ok(AuthSession {
        session_id: row.0,
        user_id: row.1,
        csrf_token: row.2,
        email: row.4,
        role: row.5,
    })
}

pub fn enforce_csrf(req: &HttpRequest, session: &AuthSession) -> Result<(), AppError> {
    let supplied = req
        .headers()
        .get("x-csrf-token")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Forbidden("csrf token required".to_string()))?;
    if supplied != session.csrf_token {
        return Err(AppError::Forbidden("csrf token mismatch".to_string()));
    }
    Ok(())
}

pub async fn revoke_session(pool: &PgPool, session_id: Uuid) -> Result<(), AppError> {
    sqlx::query("delete from sessions where id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub fn require_global_role(session: &AuthSession, minimum: &str) -> Result<(), AppError> {
    let rank = role_rank(&session.role);
    let min_rank = role_rank(minimum);
    if rank < min_rank {
        return Err(AppError::Forbidden("role forbidden".to_string()));
    }
    Ok(())
}

fn role_rank(role: &str) -> i32 {
    match role {
        "owner" => 4,
        "admin" => 3,
        "editor" => 2,
        "viewer" => 1,
        _ => 0,
    }
}

pub fn enforce_auth_rate_limit(key: &str) -> Result<(), AppError> {
    let now = Instant::now();
    let window = StdDuration::from_secs(AUTH_RATE_LIMIT_WINDOW_SECONDS);
    let limiter = AUTH_RATE_LIMITER.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = limiter.lock().map_err(|_| AppError::Internal)?;
    let bucket = guard.entry(key.to_string()).or_insert_with(VecDeque::new);

    while let Some(ts) = bucket.front().copied() {
        if now.duration_since(ts) > window {
            bucket.pop_front();
        } else {
            break;
        }
    }
    if bucket.len() >= AUTH_RATE_LIMIT_MAX {
        return Err(AppError::RateLimited);
    }
    bucket.push_back(now);
    Ok(())
}
