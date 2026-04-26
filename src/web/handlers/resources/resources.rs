//! Resource management handlers

use crate::core::{normalize_alias, validate_id};
use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::{resource_payload::ResourcePayload, session};
use crate::web::routes::AppState;
use axum::extract::{Json, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateInput {
    pub body: Option<String>,
    pub alias: Option<String>,
    pub is_favorite: Option<bool>,
    pub is_private: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateInput {
    pub body: String,
    pub alias: Option<String>,
    pub is_favorite: bool,
    pub is_private: bool,
}

pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    let Some(content) = body.body.clone() else {
        return Err(AppError::InvalidRequest("body is required".to_string()));
    };
    let resource = db::create_resource(
        pool,
        &db::generate_resource_id(pool).await?,
        normalize_alias(body.alias.as_deref())?.as_deref(),
        &content,
        body.is_favorite.unwrap_or(false),
        body.is_private.unwrap_or(
            db::get_settings(pool)
                .await?
                .default_new_resource_is_private,
        ),
    )
    .await?;
    Ok(http::json_status(
        StatusCode::CREATED,
        ResourcePayload::from_resource(resource),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(body): Json<UpdateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    match db::update_resource(
        pool,
        &id,
        normalize_alias(body.alias.as_deref())?.as_deref(),
        &body.body,
        body.is_favorite,
        body.is_private,
    )
    .await?
    {
        Some(resource) => Ok(http::json_status(
            StatusCode::OK,
            ResourcePayload::from_resource(resource),
        )),
        None => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}

pub async fn api_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(reference): Path<String>,
    Json(body): Json<UpdateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    let resource = db::get_resource_by_ref(pool, &reference)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("resource '{reference}' not found")))?;
    match db::update_resource(
        pool,
        &resource.id,
        normalize_alias(body.alias.as_deref())?.as_deref(),
        &body.body,
        body.is_favorite,
        body.is_private,
    )
    .await?
    {
        Some(resource) => Ok(http::json_status(
            StatusCode::OK,
            ResourcePayload::from_resource(resource),
        )),
        None => Err(AppError::NotFound(format!(
            "resource '{reference}' not found"
        ))),
    }
}

pub async fn remove(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    if db::delete_resource(pool, &id).await? {
        Ok(http::empty(StatusCode::NO_CONTENT))
    } else {
        Err(AppError::NotFound(format!("resource '{id}' not found")))
    }
}
