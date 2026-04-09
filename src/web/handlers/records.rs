//! Record management handlers

use crate::core::{normalize_alias, validate_id};
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::{resource_payload::ResourcePayload, session};
use actix_web::{delete, post, put, web, HttpRequest, HttpResponse};
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

#[post("/resources/notes")]
pub async fn create(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let Some(content) = body.body.clone() else {
        return Err(AppError::InvalidRequest("body is required".to_string()));
    };
    let record = db::create_record(
        &pool,
        &db::generate_resource_id(&pool).await?,
        normalize_alias(body.alias.as_deref())?.as_deref(),
        &content,
        body.is_favorite.unwrap_or(false),
        body.is_private
            .unwrap_or(db::get_settings(&pool).await?.default_new_resource_is_private),
    )
    .await?;
    Ok(HttpResponse::Created().json(ResourcePayload::from_record(record)))
}

#[put("/resources/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    match db::update_record(
        &pool,
        &id,
        normalize_alias(body.alias.as_deref())?.as_deref(),
        &body.body,
        body.is_favorite,
        body.is_private,
    )
    .await?
    {
        Some(record) => Ok(HttpResponse::Ok().json(ResourcePayload::from_record(record))),
        None => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}

#[delete("/resources/{id}")]
pub async fn remove(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    if db::delete_record(&pool, &id).await? {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(format!("resource '{id}' not found")))
    }
}
