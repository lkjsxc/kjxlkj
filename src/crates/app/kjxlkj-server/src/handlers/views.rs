use crate::app_state::AppState;
use crate::authn::require_identity;
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repos;
use kjxlkj_db::repos::views::CreateSavedViewInput;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{ensure_note_write, ensure_workspace_member_read};
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ListViewsQuery {
    workspace_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct CreateViewRequest {
    workspace_id: Uuid,
    query_json: serde_json::Value,
    sort: String,
    filters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct UpdateViewRequest {
    query_json: Option<serde_json::Value>,
    sort: Option<String>,
    filters: Option<serde_json::Value>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/views", web::get().to(list_views))
        .route("/views", web::post().to(create_view))
        .route("/views/{id}", web::patch().to(update_view))
        .route("/views/{id}", web::delete().to(delete_view));
}

async fn list_views(
    req: HttpRequest,
    query: web::Query<ListViewsQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    let workspace_role = actor_workspace_role(&state, query.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let views = repos::views::list_saved_views(&state.pool, query.workspace_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "views": views.into_iter().map(saved_view_json).collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn create_view(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateViewRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;

    validate_sort(&body.sort)?;

    let workspace_role = actor_workspace_role(&state, body.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let view = repos::views::create_saved_view(
        &state.pool,
        CreateSavedViewInput {
            workspace_id: body.workspace_id,
            owner_user_id: identity.user_id,
            query_json: body.query_json.clone(),
            sort: body.sort.clone(),
            filters: body.filters.clone(),
        },
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid view payload"))?;

    Ok(HttpResponse::Created().json(json!({
        "view": saved_view_json(view),
        "request_id": request_id,
    })))
}

async fn update_view(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<UpdateViewRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let view_id = path.into_inner();

    if body.query_json.is_none() && body.sort.is_none() && body.filters.is_none() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "BAD_REQUEST",
            "at least one field is required",
        ));
    }

    if let Some(sort) = &body.sort {
        validate_sort(sort)?;
    }

    let view = repos::views::get_saved_view(&state.pool, view_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "VIEW_NOT_FOUND", "saved view not found"))?;

    let workspace_role = actor_workspace_role(&state, view.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let updated = repos::views::update_saved_view(
        &state.pool,
        view_id,
        body.query_json.clone(),
        body.sort.clone(),
        body.filters.clone(),
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid view payload"))?
    .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "VIEW_NOT_FOUND", "saved view not found"))?;

    Ok(HttpResponse::Ok().json(json!({
        "view": saved_view_json(updated),
        "request_id": request_id,
    })))
}

async fn delete_view(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let identity = require_identity(&req, &state, true).await?;
    let view_id = path.into_inner();

    let view = repos::views::get_saved_view(&state.pool, view_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "VIEW_NOT_FOUND", "saved view not found"))?;

    let workspace_role = actor_workspace_role(&state, view.workspace_id, identity.user_id).await?;
    ensure_note_write(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let deleted = repos::views::delete_saved_view(&state.pool, view_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    if !deleted {
        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "VIEW_NOT_FOUND",
            "saved view not found",
        ));
    }

    Ok(HttpResponse::NoContent().finish())
}

fn saved_view_json(view: kjxlkj_db::models::DbSavedView) -> serde_json::Value {
    json!({
        "id": view.id,
        "workspace_id": view.workspace_id,
        "owner_user_id": view.owner_user_id,
        "query_json": view.query_json,
        "sort": view.sort,
        "filters": view.filters,
        "created_at": view.created_at.format(&Rfc3339).unwrap_or_else(|_| view.created_at.to_string()),
        "updated_at": view.updated_at.format(&Rfc3339).unwrap_or_else(|_| view.updated_at.to_string()),
    })
}

fn validate_sort(sort: &str) -> Result<(), ApiError> {
    if sort.trim().is_empty() || sort.len() > 64 {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "invalid sort value",
        ));
    }

    Ok(())
}

async fn actor_workspace_role(
    state: &AppState,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Role, ApiError> {
    let role_text = repos::workspaces::actor_workspace_role(&state.pool, workspace_id, user_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    Role::from_str(&role_text)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "invalid role data"))
}
