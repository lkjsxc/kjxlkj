//! Metadata, tags, backlinks, and search handlers per /docs/spec/api/http.md.

use crate::dto::*;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// PUT /api/notes/{id}/metadata/{key}
pub async fn upsert(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<(Uuid, String)>,
    body: web::Json<UpsertMetadataReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let (nid, key) = path.into_inner();
    match kjxlkj_db::repo::note::upsert_metadata(
        pool.get_ref(),
        nid,
        &key,
        &body.value,
    )
    .await
    {
        Ok(()) => HttpResponse::Ok()
            .json(serde_json::json!({"note_id": nid, "key": key})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/notes/{id}/metadata/{key} — returns 204.
pub async fn delete(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<(Uuid, String)>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let (nid, key) = path.into_inner();
    match kjxlkj_db::repo::note::delete_metadata(pool.get_ref(), nid, &key)
        .await
    {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/tags
pub async fn list_tags(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    let ws_id =
        match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok())
        {
            Some(id) => id,
            None => {
                return HttpResponse::BadRequest()
                    .json(ApiError::new("BAD_REQUEST", "workspace_id required"))
            }
        };
    match kjxlkj_db::repo::note::list_tags(pool.get_ref(), ws_id).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PUT /api/notes/{id}/tags
pub async fn replace_tags(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
    body: web::Json<ReplaceTagsReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::replace_tags(pool.get_ref(), nid, &body.tags)
        .await
    {
        Ok(()) => HttpResponse::Ok()
            .json(serde_json::json!({"note_id": nid, "tags": body.tags})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/notes/{id}/backlinks
pub async fn backlinks(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    let nid = path.into_inner();
    match kjxlkj_db::repo::note::get_backlinks(pool.get_ref(), nid).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows
                .iter()
                .map(|n| {
                    serde_json::json!({
                        "id": n.id, "title": n.title,
                        "workspace_id": n.workspace_id
                    })
                })
                .collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/search
pub async fn search(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    match kjxlkj_search::search_notes(
        pool.get_ref(),
        query.workspace_id,
        &query.q,
    )
    .await
    {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "note_id": r.note_id, "title": r.title,
                        "rank": r.rank
                    })
                })
                .collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
