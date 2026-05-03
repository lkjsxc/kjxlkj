use super::resources::{CreateInput, UpdateInput};
use super::{http, resource_payload::ResourcePayload, resources, session};
use crate::core::{normalize_alias, validate_id};
use crate::error::AppError;
use crate::web::db;
use crate::web::routes::AppState;
use axum::extract::{Json, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;

pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user): Path<String>,
    Json(body): Json<CreateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    db::require_space(pool, &user).await?;
    session::require_session(&headers, pool).await?;
    let Some(content) = body.body.clone() else {
        return Err(AppError::InvalidRequest("body is required".to_string()));
    };
    let resource = db::create_resource_in_space(
        pool,
        &user,
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
    resources::refresh_resource_embeds(pool, &resource.body).await?;
    Ok(http::json_status(
        StatusCode::CREATED,
        ResourcePayload::from_resource(resource),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((user, id)): Path<(String, String)>,
    Json(body): Json<UpdateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    db::require_space(pool, &user).await?;
    session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    let alias = normalize_alias(body.alias.as_deref())?;
    match db::update_resource_in_space(
        pool,
        &user,
        &id,
        alias.as_deref(),
        &body.body,
        body.is_favorite,
        body.is_private,
    )
    .await?
    {
        Some(resource) => json_resource(pool, resource).await,
        None => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}

pub async fn api_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((user, reference)): Path<(String, String)>,
    Json(body): Json<UpdateInput>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    db::require_space(pool, &user).await?;
    session::require_session(&headers, pool).await?;
    let resource = db::get_resource_by_ref_in_space(pool, &user, &reference)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("resource '{reference}' not found")))?;
    let alias = normalize_alias(body.alias.as_deref())?;
    match db::update_resource_in_space(
        pool,
        &user,
        &resource.id,
        alias.as_deref(),
        &body.body,
        body.is_favorite,
        body.is_private,
    )
    .await?
    {
        Some(resource) => json_resource(pool, resource).await,
        None => Err(AppError::NotFound(format!(
            "resource '{reference}' not found"
        ))),
    }
}

pub async fn remove(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((user, id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    db::require_space(pool, &user).await?;
    session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    if db::delete_resource_in_space(pool, &user, &id).await? {
        Ok(http::empty(StatusCode::NO_CONTENT))
    } else {
        Err(AppError::NotFound(format!("resource '{id}' not found")))
    }
}

async fn json_resource(pool: &db::DbPool, resource: db::Resource) -> Result<Response, AppError> {
    resources::refresh_resource_embeds(pool, &resource.body).await?;
    Ok(http::json_status(
        StatusCode::OK,
        ResourcePayload::from_resource(resource),
    ))
}
