// View handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_db::repo::views as view_repo;
use kjxlkj_domain::types::SavedView;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateViewRequest, ErrorBody, UpdateViewRequest};

/// GET /api/views
pub async fn list(
    pool: web::Data<PgPool>,
    query: web::Query<WsFilter>,
) -> HttpResponse {
    match view_repo::list_views(pool.get_ref(), query.workspace_id).await {
        Ok(views) => HttpResponse::Ok().json(views),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// POST /api/views
pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateViewRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let view = SavedView {
        id: Uuid::now_v7(),
        workspace_id: body.workspace_id,
        query_json: body.query_json.clone(),
        sort: body.sort.clone(),
        filters: body.filters.clone(),
        owner_user_id: Uuid::nil(), // TODO: from session
    };
    match view_repo::insert_view(pool.get_ref(), &view).await {
        Ok(()) => HttpResponse::Created().json(&view),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/views/{id}
pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateViewRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match view_repo::update_view(
        pool.get_ref(), path.into_inner(), &body.query_json,
        body.sort.as_deref(), body.filters.as_ref(),
    ).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "View not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/views/{id}
pub async fn delete(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match view_repo::delete_view(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "View not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

#[derive(serde::Deserialize)]
pub struct WsFilter {
    pub workspace_id: Uuid,
}
