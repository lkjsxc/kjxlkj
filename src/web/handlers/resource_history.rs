//! Resource history JSON and resource navigation handlers

use crate::core::validate_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool, ListDirection};
use crate::web::handlers::session;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryParams {
    pub direction: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct NavResponse {
    pub id: Option<String>,
}

#[get("/resources/{id}/history")]
pub async fn history(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    params: web::Query<HistoryParams>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    if db::get_resource(&pool, &id).await?.is_none() {
        return Err(AppError::NotFound(format!("resource '{id}' not found")));
    }
    let settings = db::get_settings(&pool).await?;
    let page = db::list_resource_snapshots(
        &pool,
        &id,
        true,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(page))
}

#[get("/resources/{id}/prev")]
pub async fn previous(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    nav_response(pool, req, path.into_inner(), true).await
}

#[get("/resources/{id}/next")]
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
    let resource = db::get_resource(&pool, &id).await?;
    match resource {
        Some(resource) if is_admin || !resource.is_private => {
            let neighbor = if older {
                db::get_previous_resource(&pool, &id, is_admin).await?
            } else {
                db::get_next_resource(&pool, &id, is_admin).await?
            };
            Ok(HttpResponse::Ok().json(NavResponse {
                id: neighbor.map(|note| note.id),
            }))
        }
        _ => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}
