use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::http::{header, HeaderMap};
use uuid::Uuid;

use crate::{
    error::ApiError,
    model::{Role, SessionRecord, Store},
    state::AppState,
};

pub const SESSION_COOKIE: &str = "kjxlkj_sid";

#[derive(Debug, Clone)]
pub struct SessionIdentity {
    pub user_id: String,
    pub role: Role,
    pub csrf_token: String,
}

pub fn now_iso() -> String {
    format!("{}Z", chrono_like_now())
}

fn chrono_like_now() -> String {
    let now = std::time::SystemTime::now();
    let secs = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("1970-01-01T00:00:{secs:02}")
}

pub fn request_id(headers: &HeaderMap) -> String {
    headers
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::encode_b64(Uuid::now_v7().as_bytes()).map_err(|_| {
        ApiError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "salt generation failed",
            Uuid::now_v7().to_string(),
        )
    })?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "password hashing failed",
                Uuid::now_v7().to_string(),
            )
        })
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(parsed) => parsed,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

pub fn session_cookie_value(headers: &HeaderMap) -> Option<String> {
    let raw_cookie = headers.get(header::COOKIE)?.to_str().ok()?;
    raw_cookie
        .split(';')
        .map(str::trim)
        .find_map(|pair| pair.strip_prefix(&format!("{SESSION_COOKIE}=")).map(ToOwned::to_owned))
}

pub async fn require_auth(
    state: &AppState,
    headers: &HeaderMap,
    request_id: String,
) -> Result<SessionIdentity, ApiError> {
    let session_id = session_cookie_value(headers).ok_or_else(|| ApiError::auth_required(request_id.clone()))?;
    let store = state.store.read().await;
    let session = store
        .sessions
        .get(&session_id)
        .cloned()
        .ok_or_else(|| ApiError::auth_required(request_id.clone()))?;
    let user = store
        .users
        .get(&session.user_id)
        .ok_or_else(|| ApiError::auth_required(request_id.clone()))?;
    if user.status != "active" {
        return Err(ApiError::auth_required(request_id));
    }
    Ok(SessionIdentity {
        user_id: user.id.clone(),
        role: user.role.clone(),
        csrf_token: session.csrf_token,
    })
}

pub fn require_role(identity: &SessionIdentity, allowed: &[Role], request_id: String) -> Result<(), ApiError> {
    if allowed.iter().any(|role| role == &identity.role) {
        return Ok(());
    }
    Err(ApiError::role_forbidden(request_id))
}

pub fn require_csrf(headers: &HeaderMap, identity: &SessionIdentity, request_id: String) -> Result<(), ApiError> {
    let token = headers
        .get("x-csrf-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if token == identity.csrf_token {
        return Ok(());
    }
    Err(ApiError::csrf_invalid(request_id))
}

pub fn issue_session(store: &mut Store, user_id: String) -> SessionRecord {
    let session = SessionRecord {
        id: Store::next_id(),
        user_id,
        csrf_token: Store::next_id(),
    };
    store.sessions.insert(session.id.clone(), session.clone());
    session
}
