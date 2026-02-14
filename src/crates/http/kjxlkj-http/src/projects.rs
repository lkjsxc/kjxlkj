// Project handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::{require_role, AuthSession};
use kjxlkj_db::repo::projects as proj_repo;
use kjxlkj_domain::types::{Project, Role};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateProjectRequest, ErrorBody, UpdateProjectRequest};

/// GET /api/projects
pub async fn list(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    query: web::Query<WorkspaceFilter>,
) -> HttpResponse {
    match proj_repo::list_projects(pool.get_ref(), query.workspace_id).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// POST /api/projects
pub async fn create(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    body: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: rid,
        });
    }
    let project = Project {
        id: Uuid::now_v7(),
        workspace_id: body.workspace_id,
        name: body.name.clone(),
        description: body.description.clone(),
        created_at: String::new(),
    };
    match proj_repo::insert_project(pool.get_ref(), &project).await {
        Ok(()) => HttpResponse::Created().json(&project),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/projects/{id}
pub async fn update(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProjectRequest>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: rid,
        });
    }
    match proj_repo::update_project(
        pool.get_ref(), path.into_inner(), &body.name, body.description.as_deref(),
    ).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "Project not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/projects/{id}
pub async fn delete(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: rid,
        });
    }
    match proj_repo::delete_project(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "Project not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

#[derive(serde::Deserialize)]
pub struct WorkspaceFilter {
    pub workspace_id: Uuid,
}
