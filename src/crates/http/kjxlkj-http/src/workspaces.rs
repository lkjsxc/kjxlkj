// Workspace handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_db::repo::workspaces as ws_repo;
use kjxlkj_domain::types::{Role, Workspace};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateWorkspaceRequest, ErrorBody, UpdateWorkspaceRequest, UpsertMemberRequest};

/// GET /api/workspaces
pub async fn list(pool: web::Data<PgPool>) -> HttpResponse {
    match ws_repo::list_workspaces(pool.get_ref()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// POST /api/workspaces
pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateWorkspaceRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let ws = Workspace {
        id: Uuid::now_v7(),
        slug: body.slug.clone(),
        name: body.name.clone(),
        owner_user_id: Uuid::nil(), // TODO: from session
        created_at: String::new(),
    };
    match ws_repo::insert_workspace(pool.get_ref(), &ws).await {
        Ok(()) => HttpResponse::Created().json(&ws),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/workspaces/{id}
pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateWorkspaceRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match ws_repo::update_workspace(pool.get_ref(), path.into_inner(), &body.name).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "WORKSPACE_NOT_FOUND".into(), message: "Workspace not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/workspaces/{id}
pub async fn delete(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match ws_repo::delete_workspace(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "WORKSPACE_NOT_FOUND".into(), message: "Workspace not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// GET /api/workspaces/{id}/members
pub async fn list_members(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    match ws_repo::list_members(pool.get_ref(), path.into_inner()).await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// PUT /api/workspaces/{id}/members/{user_id}
pub async fn upsert_member(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpsertMemberRequest>,
) -> HttpResponse {
    let (ws_id, user_id) = path.into_inner();
    let rid = Uuid::now_v7().to_string();
    let role = match body.role.as_str() {
        "owner" => Role::Owner,
        "admin" => Role::Admin,
        "editor" => Role::Editor,
        "viewer" => Role::Viewer,
        _ => {
            return HttpResponse::BadRequest().json(ErrorBody {
                code: "BAD_REQUEST".into(), message: "Invalid role".into(),
                details: None, request_id: rid,
            });
        }
    };
    match ws_repo::upsert_member(pool.get_ref(), ws_id, user_id, role).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "ok"})),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}
