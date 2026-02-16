use kjxlkj_db::repo_workspace;
use sqlx::PgPool;
use uuid::Uuid;
use kjxlkj_domain::workspace::Workspace;
use kjxlkj_domain::workspace::WorkspaceState;
use time::OffsetDateTime;

/// Create a new workspace with caller as owner.
pub async fn create_workspace(
    pool: &PgPool,
    slug: &str,
    name: &str,
    owner_user_id: Uuid,
) -> Result<Workspace, sqlx::Error> {
    let id = Uuid::now_v7();
    repo_workspace::create_workspace(pool, id, slug, name, owner_user_id).await?;
    Ok(Workspace {
        id,
        slug: slug.to_string(),
        name: name.to_string(),
        owner_user_id,
        state: WorkspaceState::Active,
        created_at: OffsetDateTime::now_utc(),
    })
}

/// List workspaces for a user.
pub async fn list_workspaces(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Workspace>, sqlx::Error> {
    let rows = repo_workspace::list_workspaces_for_user(pool, user_id).await?;
    Ok(rows
        .into_iter()
        .map(|r| Workspace {
            id: r.id,
            slug: r.slug,
            name: r.name,
            owner_user_id: r.owner_user_id,
            state: match r.state.as_str() {
                "archived" => WorkspaceState::Archived,
                "deleted" => WorkspaceState::Deleted,
                _ => WorkspaceState::Active,
            },
            created_at: r.created_at,
        })
        .collect())
}
