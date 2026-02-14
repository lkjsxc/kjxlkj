use axum::{routing::delete, routing::get, routing::patch, routing::post, routing::put, Router};

use crate::{
    handlers_automation::{rules_create, rules_delete, rules_list, rules_update},
    handlers_automation_runs::{rule_launch, run_get, run_review, runs_list},
    handlers_auth::{auth_login, auth_logout, auth_session, setup_register},
    handlers_notes_core::{notes_create, notes_get, notes_list},
    handlers_notes_history::{notes_delete, notes_history, notes_rollback},
    handlers_notes_meta::{
        note_backlinks, note_metadata_delete, note_metadata_upsert, note_tags_replace, notes_media_create,
        search, tags_list,
    },
    handlers_notes_mutations::{notes_patch, notes_title_patch},
    handlers_ops::{healthz, not_implemented, readyz},
    handlers_projects::{projects_create, projects_delete, projects_list, projects_update},
    handlers_stub::{stub_accepted, stub_no_content, stub_ok},
    handlers_users::{users_create, users_delete, users_list, users_role_update},
    handlers_views::{views_create, views_delete, views_list, views_update},
    handlers_workspace::{
        workspace_members_list, workspace_members_upsert, workspaces_create, workspaces_delete,
        workspaces_list, workspaces_update,
    },
    state::AppState,
    ws::ws_upgrade,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/setup/register", post(setup_register))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/auth/session", get(auth_session))
        .route("/api/users", get(users_list).post(users_create))
        .route("/api/users/{id}/role", patch(users_role_update))
        .route("/api/users/{id}", delete(users_delete))
        .route("/api/workspaces", get(workspaces_list).post(workspaces_create))
        .route("/api/workspaces/{id}", patch(workspaces_update).delete(workspaces_delete))
        .route("/api/workspaces/{id}/members", get(workspace_members_list))
        .route("/api/workspaces/{id}/members/{user_id}", put(workspace_members_upsert))
        .route("/api/projects", get(projects_list).post(projects_create))
        .route("/api/projects/{id}", patch(projects_update).delete(projects_delete))
        .route("/api/notes", post(notes_create).get(notes_list))
        .route("/api/notes/media", post(notes_media_create))
        .route("/api/notes/{id}", get(notes_get).patch(notes_patch).delete(notes_delete))
        .route("/api/notes/{id}/title", patch(notes_title_patch))
        .route("/api/notes/{id}/history", get(notes_history))
        .route("/api/notes/{id}/rollback", post(notes_rollback))
        .route("/api/notes/{id}/metadata/{key}", put(note_metadata_upsert).delete(note_metadata_delete))
        .route("/api/tags", get(tags_list))
        .route("/api/notes/{id}/tags", put(note_tags_replace))
        .route("/api/notes/{id}/backlinks", get(note_backlinks))
        .route("/api/search", get(search))
        .route("/api/views", get(views_list).post(views_create))
        .route("/api/views/{id}", patch(views_update).delete(views_delete))
        .route("/api/dashboards", get(stub_ok))
        .route("/api/dashboards/widgets", post(stub_ok))
        .route("/api/automation/rules", get(rules_list).post(rules_create))
        .route("/api/automation/rules/{id}", patch(rules_update).delete(rules_delete))
        .route("/api/automation/rules/{id}/launch", post(rule_launch))
        .route("/api/automation/runs", get(runs_list))
        .route("/api/automation/runs/{id}", get(run_get))
        .route("/api/automation/runs/{id}/review", post(run_review))
        .route("/api/notes/{id}/attachments", post(not_implemented))
        .route("/api/attachments/{id}", get(not_implemented).delete(stub_no_content))
        .route("/api/admin/export/markdown", post(stub_accepted))
        .route("/api/admin/export/{job_id}", get(stub_ok))
        .route("/api/admin/backup/sql", post(stub_accepted))
        .route("/api/healthz", get(healthz))
        .route("/api/readyz", get(readyz))
        .route("/ws", get(ws_upgrade))
        .with_state(state)
}
