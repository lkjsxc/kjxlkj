//! Saved views and dashboard handlers per /docs/spec/api/http.md.

use crate::dto::*;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// GET /api/views
pub async fn list(
    req: HttpRequest, pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "workspace_id required")),
    };
    match kjxlkj_db::repo::workspace::list_views(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|v| {
                serde_json::json!({
                    "id": v.id, "name": v.name, "query_json": v.query_json,
                    "sort": v.sort, "filters": v.filters
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/views
pub async fn create(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateViewReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let vid = kjxlkj_domain::types::new_id();
    let qj = body.query_json.clone().unwrap_or(serde_json::json!({}));
    let sort = body.sort.as_deref().unwrap_or("updated_at_desc");
    let filters = body.filters.clone().unwrap_or(serde_json::json!({}));
    match kjxlkj_db::repo::workspace::create_view(
        pool.get_ref(), vid, body.workspace_id, &body.name, &qj, sort, &filters, ctx.user_id,
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"id": vid, "name": body.name})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/views/{id}
pub async fn update(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<UpdateViewReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let vid = path.into_inner();
    let name = body.name.as_deref().unwrap_or("");
    let qj = body.query_json.clone().unwrap_or(serde_json::json!({}));
    let sort = body.sort.as_deref().unwrap_or("updated_at_desc");
    let filters = body.filters.clone().unwrap_or(serde_json::json!({}));
    match kjxlkj_db::repo::workspace::update_view(
        pool.get_ref(), vid, name, &qj, sort, &filters,
    ).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"id": vid})),
        Ok(false) => middleware::not_found("view"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/views/{id}
pub async fn delete(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let vid = path.into_inner();
    match kjxlkj_db::repo::workspace::delete_view(pool.get_ref(), vid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("view"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/dashboards — list widgets (optional extension).
pub async fn list_dashboards(
    req: HttpRequest, pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "workspace_id required")),
    };
    match kjxlkj_db::repo::workspace::list_widgets(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|w| {
                serde_json::json!({
                    "id": w.id, "widget_type": w.widget_type,
                    "config_json": w.config_json, "layout": w.layout
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/dashboards/widgets — upsert widget.
pub async fn upsert_widget(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<UpsertWidgetReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let wid = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::workspace::upsert_widget(
        pool.get_ref(), wid, body.workspace_id, &body.widget_type,
        &body.config_json, body.layout.as_ref(),
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"id": wid})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
