use actix_web::{web, HttpRequest};
use chrono::Utc;
use uuid::Uuid;

use crate::error::AppError;
use crate::web::state::WebState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionState {
    pub session_id: Uuid,
    pub admin_id: i64,
}

pub fn session_id_from_request(request: &HttpRequest) -> Option<Uuid> {
    let raw = request.cookie("session_id")?;
    Uuid::parse_str(raw.value()).ok()
}

pub async fn valid_session(
    request: &HttpRequest,
    state: &web::Data<WebState>,
) -> Result<Option<SessionState>, AppError> {
    let Some(session_id) = session_id_from_request(request) else {
        return Ok(None);
    };

    let now = Utc::now();
    let session = state.session_store.lookup_session(session_id).await?;
    let Some(session) = session else {
        return Ok(None);
    };
    if session.expires_at <= now {
        let _ = state.session_store.delete_session(session_id).await;
        return Ok(None);
    }

    Ok(Some(SessionState {
        session_id,
        admin_id: session.admin_id,
    }))
}
