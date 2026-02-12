use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{auth_session, enforce_csrf, hash_password, require_global_role, AuthSession},
    error::AppError,
    models::{
        AutomationRule, AutomationRuleRequest, AutomationRun, CreateProjectRequest,
        CreateUserRequest, CreateWorkspaceRequest, DashboardWidget, DashboardWidgetUpsertRequest,
        Project, SavedView, SavedViewUpsertRequest, UpdateProjectRequest, UpdateRoleRequest,
        UpdateWorkspaceRequest, User, Workspace, WorkspaceMember, WorkspaceMemberUpsertRequest,
    },
    ws::ServerEvent,
};

#[derive(Debug, Deserialize)]
pub struct WorkspaceScopedQuery {
    pub workspace_id: Option<Uuid>,
}

async fn require_workspace_mutation(
    state: &AppState,
    session: &AuthSession,
    workspace_id: Uuid,
) -> Result<(), AppError> {
    if session.role == "owner" || session.role == "admin" {
        return Ok(());
    }
    let role = sqlx::query_scalar::<_, String>(
        "select role from workspace_members where workspace_id = $1 and user_id = $2",
    )
    .bind(workspace_id)
    .bind(session.user_id)
    .fetch_optional(&state.pool)
    .await?
    .unwrap_or_default();
    if matches!(role.as_str(), "owner" | "admin" | "editor") {
        return Ok(());
    }
    Err(AppError::Forbidden("role forbidden".to_string()))
}

async fn require_workspace_admin(
    state: &AppState,
    session: &AuthSession,
    workspace_id: Uuid,
) -> Result<(), AppError> {
    if session.role == "owner" || session.role == "admin" {
        return Ok(());
    }
    let role = sqlx::query_scalar::<_, String>(
        "select role from workspace_members where workspace_id = $1 and user_id = $2",
    )
    .bind(workspace_id)
    .bind(session.user_id)
    .fetch_optional(&state.pool)
    .await?
    .unwrap_or_default();
    if matches!(role.as_str(), "owner" | "admin") {
        return Ok(());
    }
    Err(AppError::Forbidden("role forbidden".to_string()))
}

async fn emit_workspace_event(
    state: &AppState,
    workspace_id: Uuid,
    actor_id: Uuid,
    event_type: &str,
    payload: serde_json::Value,
) -> Result<(), AppError> {
    let seq: i64 = sqlx::query_scalar(
        "select coalesce(max(seq), 0) + 1 from workspace_events where workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await?;

    sqlx::query(
        "insert into workspace_events (event_id, workspace_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, $3, $4, $5, $6)",
    )
    .bind(Uuid::now_v7())
    .bind(workspace_id)
    .bind(seq)
    .bind(event_type)
    .bind(&payload)
    .bind(actor_id)
    .execute(&state.pool)
    .await?;

    state
        .publish_workspace(
            workspace_id,
            ServerEvent::workspace_event(workspace_id, seq, event_type, payload),
        )
        .await;
    Ok(())
}

pub async fn list_users(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    require_global_role(&session, "admin")?;
    let users = sqlx::query_as::<_, User>(
        "select id, email, display_name, role, status, created_at from users order by created_at asc",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn create_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_global_role(&session, "admin")?;

    let id = Uuid::now_v7();
    let password_hash = hash_password(&payload.password)?;
    sqlx::query(
        "insert into users (id, email, password_hash, display_name, role, status)
         values ($1, $2, $3, $4, $5, 'active')",
    )
    .bind(id)
    .bind(payload.email.trim().to_lowercase())
    .bind(password_hash)
    .bind(payload.display_name.trim())
    .bind(payload.role.trim().to_lowercase())
    .execute(&state.pool)
    .await
    .map_err(|_| AppError::Conflict("email already exists".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "select id, email, display_name, role, status, created_at from users where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn update_user_role(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_global_role(&session, "admin")?;

    let changed = sqlx::query("update users set role = $2 where id = $1")
        .bind(path.into_inner())
        .bind(payload.role.trim().to_lowercase())
        .execute(&state.pool)
        .await?
        .rows_affected();
    if changed == 0 {
        return Err(AppError::NotFound("user not found".to_string()));
    }
    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}

pub async fn delete_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_global_role(&session, "admin")?;

    let changed = sqlx::query("update users set status = 'disabled' where id = $1")
        .bind(path.into_inner())
        .execute(&state.pool)
        .await?
        .rows_affected();
    if changed == 0 {
        return Err(AppError::NotFound("user not found".to_string()));
    }
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_workspaces(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let rows = sqlx::query_as::<_, Workspace>(
        "select w.id, w.slug, w.name, w.owner_user_id, w.created_at
         from workspaces w
         join workspace_members m on m.workspace_id = w.id
         where w.deleted_at is null and m.user_id = $1
         order by w.created_at asc",
    )
    .bind(session.user_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn create_workspace(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<CreateWorkspaceRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_global_role(&session, "editor")?;

    let workspace_id = Uuid::now_v7();
    sqlx::query("insert into workspaces (id, slug, name, owner_user_id) values ($1, $2, $3, $4)")
        .bind(workspace_id)
        .bind(payload.slug.trim())
        .bind(payload.name.trim())
        .bind(session.user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| AppError::Conflict("workspace slug already exists".to_string()))?;

    sqlx::query(
        "insert into workspace_members (workspace_id, user_id, role) values ($1, $2, 'owner')",
    )
    .bind(workspace_id)
    .bind(session.user_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "workspace_created",
        json!({"workspace_id": workspace_id}),
    )
    .await?;

    let workspace = sqlx::query_as::<_, Workspace>(
        "select id, slug, name, owner_user_id, created_at from workspaces where id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(workspace))
}

pub async fn update_workspace(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateWorkspaceRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let workspace_id = path.into_inner();
    require_workspace_admin(&state, &session, workspace_id).await?;

    sqlx::query(
        "update workspaces
         set slug = coalesce($2, slug), name = coalesce($3, name), updated_at = now()
         where id = $1 and deleted_at is null",
    )
    .bind(workspace_id)
    .bind(payload.slug.as_deref())
    .bind(payload.name.as_deref())
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "workspace_updated",
        json!({"workspace_id": workspace_id}),
    )
    .await?;

    let workspace = sqlx::query_as::<_, Workspace>(
        "select id, slug, name, owner_user_id, created_at from workspaces where id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(workspace))
}

pub async fn delete_workspace(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let workspace_id = path.into_inner();
    require_workspace_admin(&state, &session, workspace_id).await?;

    sqlx::query("update workspaces set deleted_at = now(), updated_at = now() where id = $1")
        .bind(workspace_id)
        .execute(&state.pool)
        .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "workspace_deleted",
        json!({"workspace_id": workspace_id}),
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_workspace_members(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let workspace_id = path.into_inner();
    require_workspace_admin(&state, &session, workspace_id).await?;

    let rows = sqlx::query_as::<_, WorkspaceMember>(
        "select workspace_id, user_id, role, joined_at from workspace_members where workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn upsert_workspace_member(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    payload: web::Json<WorkspaceMemberUpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let (workspace_id, user_id) = path.into_inner();
    require_workspace_admin(&state, &session, workspace_id).await?;

    sqlx::query(
        "insert into workspace_members (workspace_id, user_id, role)
         values ($1, $2, $3)
         on conflict (workspace_id, user_id)
         do update set role = excluded.role",
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(payload.role.trim().to_lowercase())
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "workspace_member_upserted",
        json!({"workspace_id": workspace_id, "user_id": user_id, "role": payload.role}),
    )
    .await?;

    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}

pub async fn list_projects(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<WorkspaceScopedQuery>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    if let Some(workspace_id) = query.workspace_id {
        require_workspace_mutation(&state, &session, workspace_id).await?;
        let rows = sqlx::query_as::<_, Project>(
            "select id, workspace_id, name, description, created_at
             from projects where workspace_id = $1 and archived_at is null order by created_at desc",
        )
        .bind(workspace_id)
        .fetch_all(&state.pool)
        .await?;
        return Ok(HttpResponse::Ok().json(rows));
    }

    let rows = sqlx::query_as::<_, Project>(
        "select p.id, p.workspace_id, p.name, p.description, p.created_at
         from projects p
         join workspace_members m on m.workspace_id = p.workspace_id
         where m.user_id = $1 and p.archived_at is null
         order by p.created_at desc",
    )
    .bind(session.user_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn create_project(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_workspace_mutation(&state, &session, payload.workspace_id).await?;

    let id = Uuid::now_v7();
    sqlx::query(
        "insert into projects (id, workspace_id, name, description) values ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(payload.workspace_id)
    .bind(payload.name.trim())
    .bind(payload.description.trim())
    .execute(&state.pool)
    .await
    .map_err(|_| AppError::Conflict("project name already exists in workspace".to_string()))?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "project_created",
        json!({"project_id": id}),
    )
    .await?;

    let row = sqlx::query_as::<_, Project>(
        "select id, workspace_id, name, description, created_at from projects where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(row))
}

pub async fn update_project(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateProjectRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let id = path.into_inner();

    let workspace_id =
        sqlx::query_scalar::<_, Uuid>("select workspace_id from projects where id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("project not found".to_string()))?;
    require_workspace_mutation(&state, &session, workspace_id).await?;

    let archived = payload.archived.unwrap_or(false);
    sqlx::query(
        "update projects
         set name = coalesce($2, name),
             description = coalesce($3, description),
             archived_at = case when $4 then now() else null end,
             updated_at = now()
         where id = $1",
    )
    .bind(id)
    .bind(payload.name.as_deref())
    .bind(payload.description.as_deref())
    .bind(archived)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "project_updated",
        json!({"project_id": id}),
    )
    .await?;

    let row = sqlx::query_as::<_, Project>(
        "select id, workspace_id, name, description, created_at from projects where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(row))
}

pub async fn delete_project(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let id = path.into_inner();

    let workspace_id =
        sqlx::query_scalar::<_, Uuid>("select workspace_id from projects where id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("project not found".to_string()))?;
    require_workspace_admin(&state, &session, workspace_id).await?;

    sqlx::query("delete from projects where id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    // Keep notes reachable: unset project_id on deletion.
    sqlx::query("update note_streams set project_id = null where project_id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;
    sqlx::query("update note_projections set project_id = null where project_id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "project_deleted",
        json!({"project_id": id}),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_views(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<WorkspaceScopedQuery>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let workspace_id = query
        .workspace_id
        .ok_or_else(|| AppError::BadRequest("workspace_id is required".to_string()))?;
    require_workspace_mutation(&state, &session, workspace_id).await?;

    let rows = sqlx::query_as::<_, SavedView>(
        "select id, workspace_id, query_json, sort, filters, owner_user_id
         from saved_views where workspace_id = $1 order by updated_at desc",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn create_view(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<SavedViewUpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_workspace_mutation(&state, &session, payload.workspace_id).await?;

    let id = Uuid::now_v7();
    sqlx::query(
        "insert into saved_views (id, workspace_id, query_json, sort, filters, owner_user_id)
         values ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(payload.workspace_id)
    .bind(&payload.query_json)
    .bind(payload.sort.trim())
    .bind(&payload.filters)
    .bind(session.user_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "view_created",
        json!({"view_id": id}),
    )
    .await?;

    let row = sqlx::query_as::<_, SavedView>(
        "select id, workspace_id, query_json, sort, filters, owner_user_id from saved_views where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(row))
}

pub async fn update_view(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<SavedViewUpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_workspace_mutation(&state, &session, payload.workspace_id).await?;
    let id = path.into_inner();

    sqlx::query(
        "update saved_views
         set query_json = $2, sort = $3, filters = $4, updated_at = now()
         where id = $1 and workspace_id = $5",
    )
    .bind(id)
    .bind(&payload.query_json)
    .bind(payload.sort.trim())
    .bind(&payload.filters)
    .bind(payload.workspace_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "view_updated",
        json!({"view_id": id}),
    )
    .await?;

    let row = sqlx::query_as::<_, SavedView>(
        "select id, workspace_id, query_json, sort, filters, owner_user_id from saved_views where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(row))
}

pub async fn delete_view(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let id = path.into_inner();

    let workspace_id =
        sqlx::query_scalar::<_, Uuid>("select workspace_id from saved_views where id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("view not found".to_string()))?;
    require_workspace_mutation(&state, &session, workspace_id).await?;

    sqlx::query("delete from saved_views where id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "view_deleted",
        json!({"view_id": id}),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_dashboards(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<WorkspaceScopedQuery>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let workspace_id = query
        .workspace_id
        .ok_or_else(|| AppError::BadRequest("workspace_id is required".to_string()))?;
    require_workspace_mutation(&state, &session, workspace_id).await?;

    let rows = sqlx::query_as::<_, DashboardWidget>(
        "select id, workspace_id, type as widget_type, config_json, layout
         from dashboard_widgets where workspace_id = $1 order by updated_at desc",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn upsert_dashboard_widget(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<DashboardWidgetUpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_workspace_mutation(&state, &session, payload.workspace_id).await?;

    let widget_id = payload.id.unwrap_or_else(Uuid::now_v7);
    sqlx::query(
        "insert into dashboard_widgets (id, workspace_id, type, config_json, layout, owner_user_id)
         values ($1, $2, $3, $4, $5, $6)
         on conflict (id)
         do update set type = excluded.type, config_json = excluded.config_json,
                       layout = excluded.layout, updated_at = now()",
    )
    .bind(widget_id)
    .bind(payload.workspace_id)
    .bind(payload.r#type.trim())
    .bind(&payload.config_json)
    .bind(&payload.layout)
    .bind(session.user_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "dashboard_widget_upserted",
        json!({"widget_id": widget_id}),
    )
    .await?;

    let row = sqlx::query_as::<_, DashboardWidget>(
        "select id, workspace_id, type as widget_type, config_json, layout
         from dashboard_widgets where id = $1",
    )
    .bind(widget_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(row))
}

pub async fn list_automation_rules(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<WorkspaceScopedQuery>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let workspace_id = query
        .workspace_id
        .ok_or_else(|| AppError::BadRequest("workspace_id is required".to_string()))?;
    require_workspace_mutation(&state, &session, workspace_id).await?;

    let rows = sqlx::query_as::<_, AutomationRule>(
        "select id, workspace_id, trigger, condition_json, action_json, enabled
         from automation_rules where workspace_id = $1 order by created_at desc",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

fn validate_rule(input: &AutomationRuleRequest) -> Result<(), AppError> {
    if input.trigger.trim().is_empty() {
        return Err(AppError::BadRequest("trigger is required".to_string()));
    }
    if input.action_json.is_null() || input.action_json == json!({}) {
        return Err(AppError::BadRequest("action_json is required".to_string()));
    }
    Ok(())
}

pub async fn create_automation_rule(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<AutomationRuleRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    require_workspace_admin(&state, &session, payload.workspace_id).await?;
    validate_rule(&payload)?;

    let id = Uuid::now_v7();
    sqlx::query(
        "insert into automation_rules
         (id, workspace_id, trigger, condition_json, action_json, enabled, created_by)
         values ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(payload.workspace_id)
    .bind(payload.trigger.trim())
    .bind(&payload.condition_json)
    .bind(&payload.action_json)
    .bind(payload.enabled)
    .bind(session.user_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "automation_rule_created",
        json!({"rule_id": id}),
    )
    .await?;

    let row = sqlx::query_as::<_, AutomationRule>(
        "select id, workspace_id, trigger, condition_json, action_json, enabled
         from automation_rules where id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(row))
}

pub async fn update_automation_rule(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<AutomationRuleRequest>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let rule_id = path.into_inner();
    require_workspace_admin(&state, &session, payload.workspace_id).await?;
    validate_rule(&payload)?;

    sqlx::query(
        "update automation_rules
         set trigger = $2, condition_json = $3, action_json = $4, enabled = $5, updated_at = now()
         where id = $1 and workspace_id = $6",
    )
    .bind(rule_id)
    .bind(payload.trigger.trim())
    .bind(&payload.condition_json)
    .bind(&payload.action_json)
    .bind(payload.enabled)
    .bind(payload.workspace_id)
    .execute(&state.pool)
    .await?;

    emit_workspace_event(
        &state,
        payload.workspace_id,
        session.user_id,
        "automation_rule_updated",
        json!({"rule_id": rule_id}),
    )
    .await?;

    let row = sqlx::query_as::<_, AutomationRule>(
        "select id, workspace_id, trigger, condition_json, action_json, enabled
         from automation_rules where id = $1",
    )
    .bind(rule_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Ok().json(row))
}

pub async fn delete_automation_rule(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let rule_id = path.into_inner();

    let workspace_id =
        sqlx::query_scalar::<_, Uuid>("select workspace_id from automation_rules where id = $1")
            .bind(rule_id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("rule not found".to_string()))?;

    require_workspace_admin(&state, &session, workspace_id).await?;
    sqlx::query("delete from automation_rules where id = $1")
        .bind(rule_id)
        .execute(&state.pool)
        .await?;

    emit_workspace_event(
        &state,
        workspace_id,
        session.user_id,
        "automation_rule_deleted",
        json!({"rule_id": rule_id}),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_automation_run(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    let run_id = path.into_inner();
    let run = sqlx::query_as::<_, AutomationRun>(
        "select id, rule_id, workspace_id, status, started_at, finished_at, result_json
         from automation_runs where id = $1",
    )
    .bind(run_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("automation run not found".to_string()))?;

    require_workspace_mutation(&state, &session, run.workspace_id).await?;
    Ok(HttpResponse::Ok().json(run))
}

pub async fn trigger_automation_for_event(
    state: &AppState,
    workspace_id: Uuid,
    triggering_event_id: Uuid,
) -> Result<(), AppError> {
    let rules = sqlx::query_as::<_, (Uuid, bool)>(
        "select id, enabled from automation_rules where workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await?;

    for (rule_id, enabled) in rules {
        if !enabled {
            continue;
        }

        let run_id = Uuid::now_v7();
        let inserted = sqlx::query(
            "insert into automation_runs
             (id, rule_id, workspace_id, triggering_event_id, status, result_json)
             values ($1, $2, $3, $4, 'succeeded', $5)
             on conflict (rule_id, triggering_event_id) do nothing",
        )
        .bind(run_id)
        .bind(rule_id)
        .bind(workspace_id)
        .bind(triggering_event_id)
        .bind(json!({"result": "noop"}))
        .execute(&state.pool)
        .await?
        .rows_affected();

        if inserted > 0 {
            state
                .publish_workspace(
                    workspace_id,
                    ServerEvent::AutomationEvent {
                        workspace_id,
                        run_id,
                        status: "succeeded".to_string(),
                        payload: json!({"rule_id": rule_id}),
                    },
                )
                .await;
        }
    }

    Ok(())
}
