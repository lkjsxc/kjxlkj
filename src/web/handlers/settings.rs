//! Admin settings handler

use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::handlers::settings_input::{validate_settings_form, SettingsForm};
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::{templates, view};
use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, Uri};
use axum::response::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PasswordForm {
    pub current_password: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn settings_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
) -> Result<Response, AppError> {
    settings_page_inner(State(state), headers, uri, None).await
}

pub async fn settings_page_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
    Path(user): Path<String>,
) -> Result<Response, AppError> {
    db::require_space(&state.pool, &user).await?;
    settings_page_inner(State(state), headers, uri, Some(user)).await
}

async fn settings_page_inner(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
    space_slug: Option<String>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    if !session::check_session(&headers, pool).await? {
        return Ok(http::redirect(&session::login_url(&uri)));
    }
    let settings = match space_slug.as_deref() {
        Some(slug) => db::get_settings_in_space(pool, slug).await?,
        None => db::get_settings(pool).await?,
    };
    let favorites = db::list_all_favorite_resources(pool, space_slug.as_deref(), true).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(http::html(templates::settings_page(
        &settings,
        &favorites
            .iter()
            .map(|resource| view::index_item(resource, true))
            .collect::<Vec<_>>(),
        &site,
    )))
}

pub async fn settings_submit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<SettingsForm>,
) -> Result<Response, AppError> {
    settings_submit_inner(State(state), headers, Form(form), None).await
}

pub async fn settings_submit_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user): Path<String>,
    Form(form): Form<SettingsForm>,
) -> Result<Response, AppError> {
    db::require_space(&state.pool, &user).await?;
    settings_submit_inner(State(state), headers, Form(form), Some(user)).await
}

async fn settings_submit_inner(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<SettingsForm>,
    space_slug: Option<String>,
) -> Result<Response, AppError> {
    session::require_session(&headers, &state.pool).await?;
    let pool = &state.pool;
    let current = match space_slug.as_deref() {
        Some(slug) => db::get_settings_in_space(pool, slug).await?,
        None => db::get_settings(pool).await?,
    };
    let next = validate_settings_form(&form, &current)?;
    match space_slug.as_deref() {
        Some(slug) => db::update_settings_in_space(pool, slug, &next).await?,
        None => db::update_settings(pool, &next).await?,
    }
    let site = SiteContext::from_settings(&next);
    crate::web::embed_unfurl::refresh_body_embeds(
        pool,
        &next.home_intro_markdown,
        site.public_base_url.as_deref(),
    )
    .await?;
    Ok(http::see_other(
        &space_slug.map_or("/admin/settings".to_string(), |slug| {
            format!("/{slug}/settings")
        }),
    ))
}

pub async fn password_submit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<PasswordForm>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let user_id = session::require_session(&headers, pool).await?;
    if form.password.len() < 8 || form.password != form.confirm_password {
        return Err(AppError::InvalidRequest(
            "password must be at least 8 characters and match confirmation".to_string(),
        ));
    }
    if !db::verify_admin_password(pool, user_id, &form.current_password).await? {
        return Err(AppError::Unauthorized(
            "current password is invalid".to_string(),
        ));
    }
    db::update_admin_password(pool, user_id, &form.password).await?;
    let mut response = http::see_other("/login");
    http::set_cookie(&mut response, http::clear_session_cookie());
    Ok(response)
}
