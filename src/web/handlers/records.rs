//! Record management handlers

use crate::core::{generate_id, validate_id};
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

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
    pub id: Option<String>,
}

#[post("/records")]
pub async fn create(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = generate_unique_id(&pool).await?;
    let Some(content) = body.body.clone() else {
        return Err(AppError::InvalidRequest("body is required".to_string()));
    };
    let is_private = body.is_private.unwrap_or(true);
    let record = db::create_record(&pool, &id, &content, is_private).await?;
    Ok(HttpResponse::Created().json(record))
}

#[put("/records/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    match db::update_record(&pool, &id, &body.body, body.is_private).await? {
        Some(record) => Ok(HttpResponse::Ok().json(record)),
        None => Err(AppError::NotFound(format!("note '{id}' not found"))),
    }
}

#[delete("/records/{id}")]
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
        Err(AppError::NotFound(format!("note '{id}' not found")))
    }
}

#[get("/records/{id}/history")]
pub async fn history(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    if db::get_record(&pool, &id).await?.is_none() {
        return Err(AppError::NotFound(format!("note '{id}' not found")));
    }
    let revisions = db::get_record_revisions(&pool, &id).await?;
    Ok(HttpResponse::Ok().json(revisions))
}

#[get("/records/{id}/prev")]
pub async fn previous(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    nav_response(pool, req, path.into_inner(), true).await
}

#[get("/records/{id}/next")]
pub async fn next(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    nav_response(pool, req, path.into_inner(), false).await
}

async fn nav_response(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    id: String,
    older: bool,
) -> Result<HttpResponse, AppError> {
    validate_id(&id)?;
    let is_admin = session::check_session(&req, &pool).await?;
    let record = db::get_record(&pool, &id).await?;
    match record {
        Some(record) if is_admin || !record.is_private => {
            let neighbor = if older {
                db::get_previous_id(&pool, &id, is_admin).await?
            } else {
                db::get_next_id(&pool, &id, is_admin).await?
            };
            Ok(HttpResponse::Ok().json(NavResponse { id: neighbor }))
        }
        _ => Err(AppError::NotFound(format!("note '{id}' not found"))),
    }
}

async fn generate_unique_id(pool: &DbPool) -> Result<String, AppError> {
    for _ in 0..10 {
        let id = generate_id();
        if db::get_record(pool, &id).await?.is_none() {
            return Ok(id);
        }
    }
    Err(AppError::StorageError(
        "could not generate unique id".to_string(),
    ))
}
