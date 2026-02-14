use crate::app_state::AppState;
use crate::error::ApiError;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use kjxlkj_db::repos;
use kjxlkj_domain::{Role, UserStatus};
use std::str::FromStr;
use uuid::Uuid;

pub const SESSION_COOKIE: &str = "kjxlkj_session";
pub const CSRF_HEADER: &str = "x-csrf-token";

#[derive(Debug, Clone)]
pub struct SessionIdentity {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub csrf_token: String,
    pub email: String,
    pub display_name: String,
}

pub async fn optional_identity(
    req: &HttpRequest,
    state: &AppState,
) -> Result<Option<SessionIdentity>, ApiError> {
    let Some(cookie) = req.cookie(SESSION_COOKIE) else {
        return Ok(None);
    };

    let session_id = match Uuid::parse_str(cookie.value()) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };

    let Some(session_with_user) = repos::auth::get_session_with_user(&state.pool, session_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    else {
        return Ok(None);
    };

    let role = Role::from_str(&session_with_user.role)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "invalid role storage"))?;

    if session_with_user.status != UserStatus::Active.as_str() {
        return Ok(None);
    }

    let _ = repos::auth::touch_session(&state.pool, session_id).await;

    Ok(Some(SessionIdentity {
        session_id,
        user_id: session_with_user.user_id,
        role,
        csrf_token: session_with_user.csrf_token,
        email: session_with_user.email,
        display_name: session_with_user.display_name,
    }))
}

pub async fn require_identity(
    req: &HttpRequest,
    state: &AppState,
    require_csrf: bool,
) -> Result<SessionIdentity, ApiError> {
    let Some(identity) = optional_identity(req, state).await? else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "AUTH_REQUIRED",
            "authentication required",
        ));
    };

    if require_csrf {
        let provided = req
            .headers()
            .get(CSRF_HEADER)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();

        if provided != identity.csrf_token {
            return Err(ApiError::new(
                StatusCode::FORBIDDEN,
                "CSRF_INVALID",
                "csrf token invalid",
            ));
        }
    }

    Ok(identity)
}

pub fn client_key(req: &HttpRequest) -> String {
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_owned()
}
