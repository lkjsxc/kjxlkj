use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::core::RecordInput;
use crate::error::AppError;

use super::state::AppState;

pub async fn get_health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("ok")
}

pub async fn list_records(state: web::Data<AppState>) -> impl Responder {
    respond_result(
        async move {
            let records = state.store.list().await?;
            Ok(HttpResponse::Ok().json(records))
        }
        .await,
    )
}

pub async fn get_record(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    respond_result(
        async move {
            let id = path.into_inner();
            match state.store.get(&id).await? {
                Some(record) => Ok(HttpResponse::Ok().json(record)),
                None => Err(AppError::NotFound(format!("record not found: {id}"))),
            }
        }
        .await,
    )
}

pub async fn put_record(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<RecordInput>,
    state: web::Data<AppState>,
) -> impl Responder {
    respond_result(
        async move {
            require_token(&req, &state.admin_token)?;
            let id = path.into_inner();
            let (record, created) = state.store.upsert(&id, body.into_inner()).await?;
            let status = if created {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            Ok(HttpResponse::build(status).json(record))
        }
        .await,
    )
}

pub async fn delete_record(
    req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    respond_result(
        async move {
            require_token(&req, &state.admin_token)?;
            let id = path.into_inner();
            if state.store.delete(&id).await? {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Err(AppError::NotFound(format!("record not found: {id}")))
            }
        }
        .await,
    )
}

fn require_token(req: &HttpRequest, expected: &str) -> Result<(), AppError> {
    let provided = req
        .headers()
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok());
    if provided == Some(expected) {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}

fn respond_result(result: Result<HttpResponse, AppError>) -> HttpResponse {
    match result {
        Ok(response) => response,
        Err(error) => {
            let status = match error {
                AppError::Unauthorized => StatusCode::UNAUTHORIZED,
                AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
                AppError::NotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            HttpResponse::build(status).json(json!({
                "error": error.code(),
                "message": error.to_string()
            }))
        }
    }
}
