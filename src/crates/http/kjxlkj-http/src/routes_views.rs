//! Saved views route handlers per /docs/spec/api/http.md.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_view;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::WorkspaceId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto_views::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /views per /docs/spec/api/http.md.
pub async fn list_views(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(u) => WorkspaceId(u),
        None => {
            return domain_error_response(
                DomainError::BadRequest("workspace_id required".into()),
                &rid,
            )
        }
    };
    match repo_view::list_views(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<SavedViewResponse> = rows
                .into_iter()
                .map(|v| SavedViewResponse {
                    id: v.id,
                    workspace_id: v.workspace_id,
                    name: v.name,
                    query_json: v.query_json,
                    sort: v.sort,
                    filters: v.filters,
                    owner_user_id: v.owner_user_id,
                    created_at: v.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /views per /docs/spec/api/http.md.
pub async fn create_view(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateViewRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(body.workspace_id);
    let view_id = Uuid::now_v7();
    let filters = body.filters.clone().unwrap_or(serde_json::json!({}));
    match repo_view::create_view(
        pool.get_ref(),
        view_id,
        ws_id,
        &body.name,
        &body.query_json,
        body.sort.as_deref(),
        &filters,
        identity.user_id.0,
    )
    .await
    {
        Ok(v) => HttpResponse::Created().json(SavedViewResponse {
            id: v.id,
            workspace_id: v.workspace_id,
            name: v.name,
            query_json: v.query_json,
            sort: v.sort,
            filters: v.filters,
            owner_user_id: v.owner_user_id,
            created_at: v.created_at.to_string(),
        }),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// PATCH /views/{id} per /docs/spec/api/http.md.
pub async fn update_view(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateViewRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let view_id = path.into_inner();
    let name = body.name.as_deref().unwrap_or("");
    let query_json = body
        .query_json
        .clone()
        .unwrap_or(serde_json::json!({}));
    let filters = body.filters.clone().unwrap_or(serde_json::json!({}));
    match repo_view::update_view(
        pool.get_ref(),
        view_id,
        name,
        &query_json,
        body.sort.as_deref(),
        &filters,
    )
    .await
    {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "updated", "request_id": rid
        })),
        Ok(false) => domain_error_response(DomainError::NotFound("view".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// DELETE /views/{id} per /docs/spec/api/http.md.
pub async fn delete_view(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let view_id = path.into_inner();
    match repo_view::delete_view(pool.get_ref(), view_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(DomainError::NotFound("view".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}
