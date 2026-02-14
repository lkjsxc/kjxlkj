use kjxlkj_auth::hash_password;
use kjxlkj_db::repos;
use kjxlkj_db::PgPool;
use kjxlkj_domain::Role;
use kjxlkj_workspace::{WorkspaceService, WorkspaceServiceError};
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn workspace_owner_and_membership_boundaries() {
    let pool = test_pool().await;
    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");
    reset_db(&pool).await;

    let owner_hash = hash_password("owner-password").expect("hash owner password");
    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        "owner@example.com",
        "Owner",
        &owner_hash,
        "main",
        "Main",
    )
    .await
    .expect("create owner+workspace");

    assert_eq!(workspace.owner_user_id, owner.id, "workspace owner must match owner user");

    let service = WorkspaceService::new(pool.clone());

    let editor_hash = hash_password("editor-password").expect("hash editor password");
    let editor = repos::users::create_user(
        &pool,
        "editor@example.com",
        "Editor",
        &editor_hash,
        Role::Editor.as_str(),
    )
    .await
    .expect("create editor user");

    service
        .upsert_member(
            owner.id,
            workspace.id,
            editor.id,
            Role::Editor,
            "req_owner_add_editor",
        )
        .await
        .expect("owner can upsert members");

    let members = service
        .list_members(owner.id, workspace.id)
        .await
        .expect("owner can list members");
    assert!(members.iter().any(|member| member.user_id == owner.id && member.role == "owner"));
    assert!(members.iter().any(|member| member.user_id == editor.id && member.role == "editor"));

    let viewer_hash = hash_password("viewer-password").expect("hash viewer password");
    let viewer = repos::users::create_user(
        &pool,
        "viewer@example.com",
        "Viewer",
        &viewer_hash,
        Role::Viewer.as_str(),
    )
    .await
    .expect("create viewer user");

    let forbidden = service
        .upsert_member(
            editor.id,
            workspace.id,
            viewer.id,
            Role::Viewer,
            "req_editor_add_viewer",
        )
        .await;

    assert!(matches!(forbidden, Err(WorkspaceServiceError::Forbidden)));
}

async fn test_pool() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await
        .expect("connect postgres")
}

async fn reset_db(pool: &PgPool) {
    sqlx::query(
        "TRUNCATE TABLE security_events, sessions, projects, workspace_memberships, workspaces, users RESTART IDENTITY CASCADE",
    )
    .execute(pool)
    .await
    .expect("truncate baseline tables");
}
