//! Favorite ordering handler

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use actix_web::{put, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FavoriteOrderInput {
    pub ids: Vec<String>,
}

#[put("/records/favorites/order")]
pub async fn reorder(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<FavoriteOrderInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    db::reorder_favorites(&pool, &body.ids).await?;
    Ok(HttpResponse::NoContent().finish())
}
