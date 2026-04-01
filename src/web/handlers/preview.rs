//! Preview rendering handler

use crate::core::render_markdown;
use crate::error::AppError;
use crate::web::db::DbPool;
use crate::web::handlers::session;
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PreviewInput {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct PreviewPayload {
    html: String,
}

#[post("/preview")]
pub async fn preview(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<PreviewInput>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    Ok(HttpResponse::Ok().json(PreviewPayload {
        html: render_markdown(&body.body),
    }))
}
