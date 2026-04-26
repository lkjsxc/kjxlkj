//! Logout handler

use crate::error::AppError;
use crate::web::handlers::{http, session};
use crate::web::routes::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Response;
use uuid::Uuid;

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if let Some(value) = session::cookie_value(&headers, "session_id") {
        if let Ok(session_id) = Uuid::parse_str(value) {
            let _ = crate::web::db::delete_session(&state.pool, session_id).await;
        }
    }

    let mut response = http::see_other("/");
    http::set_cookie(&mut response, http::clear_session_cookie());
    Ok(response)
}
