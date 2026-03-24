//! Record CRUD handlers

use crate::config::Config;
use crate::core::{validate_id, validate_tags, Record, RecordInput};
use crate::error::AppError;
use crate::storage::{FilesystemStorage, Storage};
use actix_web::{delete, get, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

/// List all records
#[get("/v1/records")]
pub async fn list(storage: web::Data<Arc<FilesystemStorage>>) -> Result<HttpResponse, AppError> {
    let records = storage.list().await?;
    Ok(HttpResponse::Ok().json(records))
}

/// Get a single record
#[get("/v1/records/{id}")]
pub async fn fetch(
    storage: web::Data<Arc<FilesystemStorage>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    validate_id(&id)?;

    match storage.get(&id).await? {
        Some(record) => Ok(HttpResponse::Ok().json(record)),
        None => Err(AppError::NotFound(format!("record '{}' not found", id))),
    }
}

/// Create or update a record
#[put("/v1/records/{id}")]
pub async fn upsert(
    storage: web::Data<Arc<FilesystemStorage>>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<RecordInput>,
) -> Result<HttpResponse, AppError> {
    check_token(&req, &config)?;

    let id = path.into_inner();
    validate_id(&id)?;

    let tags = validate_tags(body.tags.clone());
    let existing = storage.get(&id).await?;

    let record = match existing {
        Some(r) => r.update(body.title.clone(), body.body.clone(), tags),
        None => Record::new(id.clone(), body.title.clone(), body.body.clone(), tags),
    };

    let (saved, created) = storage.upsert(&id, record).await?;

    if created {
        Ok(HttpResponse::Created().json(saved))
    } else {
        Ok(HttpResponse::Ok().json(saved))
    }
}

/// Delete a record
#[delete("/v1/records/{id}")]
pub async fn remove(
    storage: web::Data<Arc<FilesystemStorage>>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    check_token(&req, &config)?;

    let id = path.into_inner();
    validate_id(&id)?;

    if storage.delete(&id).await? {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(format!("record '{}' not found", id)))
    }
}

fn check_token(req: &HttpRequest, config: &Config) -> Result<(), AppError> {
    let token = req
        .headers()
        .get("x-admin-token")
        .and_then(|v| v.to_str().ok());

    match token {
        Some(t) if t == config.admin_token => Ok(()),
        _ => Err(AppError::Unauthorized(
            "x-admin-token is missing or invalid".to_string(),
        )),
    }
}
