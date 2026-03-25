//! Record management handlers

use crate::core::{generate_slug, validate_slug};
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateInput {
    pub body: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateInput {
    pub body: String,
    pub is_private: bool,
}

#[derive(Serialize)]
pub struct NavResponse {
    pub slug: Option<String>,
}

/// Create a new record
#[post("/records")]
pub async fn create(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateInput>,
) -> Result<HttpResponse, AppError> {
    require_session(&req, &pool).await?;
    let slug = generate_unique_slug(&pool).await?;
    let content = body
        .body
        .clone()
        .unwrap_or_else(|| "# New Note\n".to_string());
    let is_private = body.is_private.unwrap_or(true);
    let record = db::create_record(&pool, &slug, &content, is_private).await?;
    Ok(HttpResponse::Created().json(record))
}

/// Update a record
#[put("/records/{slug}")]
pub async fn update(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateInput>,
) -> Result<HttpResponse, AppError> {
    require_session(&req, &pool).await?;
    let slug = path.into_inner();
    validate_slug(&slug)?;
    match db::update_record(&pool, &slug, &body.body, body.is_private).await? {
        Some(record) => Ok(HttpResponse::Ok().json(record)),
        None => Err(AppError::NotFound(format!("note '{slug}' not found"))),
    }
}

/// Soft delete a record
#[delete("/records/{slug}")]
pub async fn remove(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    require_session(&req, &pool).await?;
    let slug = path.into_inner();
    validate_slug(&slug)?;
    if db::delete_record(&pool, &slug).await? {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(format!("note '{slug}' not found")))
    }
}

/// Get revision history
#[get("/records/{slug}/history")]
pub async fn history(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    require_session(&req, &pool).await?;
    let slug = path.into_inner();
    validate_slug(&slug)?;
    if db::get_record(&pool, &slug).await?.is_none() {
        return Err(AppError::NotFound(format!("note '{slug}' not found")));
    }
    let revisions = db::get_record_revisions(&pool, &slug).await?;
    Ok(HttpResponse::Ok().json(revisions))
}

async fn require_session(req: &HttpRequest, pool: &DbPool) -> Result<(), AppError> {
    let cookie = req.cookie("session_id");
    let session_id = cookie
        .as_ref()
        .and_then(|c| Uuid::parse_str(c.value()).ok());
    match session_id {
        Some(id) if db::validate_session(pool, id).await?.is_some() => Ok(()),
        _ => Err(AppError::Unauthorized("Session required".to_string())),
    }
}

async fn generate_unique_slug(pool: &DbPool) -> Result<String, AppError> {
    let base = generate_slug();
    if db::get_record(pool, &base).await?.is_none() {
        return Ok(base);
    }
    for i in 2..100 {
        let slug = format!("{}-{}", base, i);
        if db::get_record(pool, &slug).await?.is_none() {
            return Ok(slug);
        }
    }
    Err(AppError::StorageError(
        "Could not generate unique slug".to_string(),
    ))
}
