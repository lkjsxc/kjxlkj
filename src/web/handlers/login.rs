//! Login handlers

use crate::error::AppError;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use axum::extract::{Form, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;

/// Login form data
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub return_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub return_to: Option<String>,
}

/// Login page GET handler
pub async fn login_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<LoginQuery>,
) -> Result<Response, AppError> {
    if !crate::web::db::is_setup(&state.pool).await? {
        return Ok(http::redirect("/setup"));
    }
    let return_to = session::valid_return_to(query.return_to.as_deref());
    if session::check_session(&headers, &state.pool).await? {
        return Ok(http::see_other(&return_to));
    }
    let settings = crate::web::db::get_settings(&state.pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(http::html(templates::login_page(&site, None, &return_to)))
}

/// Login form POST handler
pub async fn login_submit(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    if !crate::web::db::is_setup(&state.pool).await? {
        return Ok(http::redirect("/setup"));
    }

    let user_id =
        crate::web::db::verify_credentials(&state.pool, &form.username, &form.password).await?;
    let return_to = session::valid_return_to(form.return_to.as_deref());

    match user_id {
        Some(id) => {
            let settings = crate::web::db::get_settings(&state.pool).await?;
            let timeout = i32::try_from(settings.session_timeout_minutes)
                .map_err(|_| AppError::StorageError("invalid session timeout".to_string()))?;
            let session_id = crate::web::db::create_session(&state.pool, id, timeout).await?;

            let mut response = http::see_other(&return_to);
            http::set_cookie(
                &mut response,
                &http::session_cookie(&session_id.to_string()),
            );
            Ok(response)
        }
        None => Ok(http::html_status(
            StatusCode::UNAUTHORIZED,
            templates::login_page(
                &SiteContext::from_settings(&crate::web::db::get_settings(&state.pool).await?),
                Some("Invalid username or password"),
                &return_to,
            ),
        )),
    }
}
