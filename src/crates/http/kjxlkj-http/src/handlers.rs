//! HTTP handlers.

use actix_web::{web, HttpResponse};

use crate::dto::*;
use crate::error::*;
use kjxlkj_db::users::UserRepo;
use kjxlkj_db::workspaces::WorkspaceRepo;
use kjxlkj_domain::{User, GlobalRole, Workspace};
use kjxlkj_auth::PasswordHasher;

/// Health check handler.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

/// Readiness check handler.
pub async fn ready(pool: web::Data<kjxlkj_db::DbPool>) -> HttpResponse {
    let db_ready = pool.is_ready().await;
    HttpResponse::Ok().json(ReadyResponse {
        status: if db_ready { "ready" } else { "not_ready" }.to_string(),
        database: db_ready,
    })
}

/// Setup handler - creates first owner account.
pub async fn setup(
    pool: web::Data<kjxlkj_db::DbPool>,
    body: web::Json<SetupRequest>,
) -> ApiResult<HttpResponse> {
    let db_pool = pool.pool();
    let user_repo = UserRepo::new(db_pool);

    // Check if setup is already complete
    let count = user_repo.count().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    if count > 0 {
        return Err(ApiError::Conflict("setup already complete".to_string()));
    }

    // Create owner user
    let hasher = PasswordHasher::new();
    let password_hash = hasher.hash(&body.password).map_err(|e| ApiError::Internal(e.to_string()))?;

    let user = User::new(
        body.email.clone(),
        password_hash,
        GlobalRole::Owner,
    );

    user_repo.create(&user).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Created().json(SessionResponse {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        global_role: serde_json::to_string(&user.global_role).unwrap(),
    }))
}

/// Login handler.
pub async fn login(
    pool: web::Data<kjxlkj_db::DbPool>,
    body: web::Json<LoginRequest>,
) -> ApiResult<HttpResponse> {
    let db_pool = pool.pool();
    let user_repo = UserRepo::new(db_pool);

    // Find user
    let user = user_repo
        .find_by_email(&body.email)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::Unauthorized)?;

    // Verify password
    let hasher = PasswordHasher::new();
    let valid = hasher
        .verify(&body.password, &user.password_hash)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    if !valid {
        return Err(ApiError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(SessionResponse {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        global_role: serde_json::to_string(&user.global_role).unwrap(),
    }))
}

/// List users handler.
pub async fn list_users(pool: web::Data<kjxlkj_db::DbPool>) -> ApiResult<HttpResponse> {
    let db_pool = pool.pool();
    let user_repo = UserRepo::new(db_pool);

    let users = user_repo.list().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
            global_role: serde_json::to_string(&u.global_role).unwrap(),
            is_active: u.is_active,
            created_at: u.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// List workspaces handler.
pub async fn list_workspaces(pool: web::Data<kjxlkj_db::DbPool>) -> ApiResult<HttpResponse> {
    let db_pool = pool.pool();
    let workspace_repo = WorkspaceRepo::new(db_pool);

    let workspaces = workspace_repo
        .list()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response: Vec<WorkspaceResponse> = workspaces
        .into_iter()
        .map(|w| WorkspaceResponse {
            id: w.id,
            name: w.name,
            slug: w.slug,
            is_active: w.is_active,
            created_at: w.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// Create workspace handler.
pub async fn create_workspace(
    pool: web::Data<kjxlkj_db::DbPool>,
    body: web::Json<CreateWorkspaceRequest>,
) -> ApiResult<HttpResponse> {
    let db_pool = pool.pool();
    let workspace_repo = WorkspaceRepo::new(db_pool);

    let workspace = Workspace::new(body.name.clone(), body.slug.clone());

    workspace_repo
        .create(&workspace)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Created().json(WorkspaceResponse {
        id: workspace.id,
        name: workspace.name,
        slug: workspace.slug,
        is_active: workspace.is_active,
        created_at: workspace.created_at,
    }))
}

/// Configure routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthz", web::get().to(health))
            .route("/readyz", web::get().to(ready))
            .route("/setup/register", web::post().to(setup))
            .route("/auth/login", web::post().to(login))
            .route("/users", web::get().to(list_users))
            .route("/workspaces", web::get().to(list_workspaces))
            .route("/workspaces", web::post().to(create_workspace)),
    );
}
