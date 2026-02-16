/// PostgreSQL workspace repository per /docs/spec/domain/workspaces.md
///
/// Maps to workspaces table (migration 002).
use crate::pg_rows::pg_err;
use kjxlkj_domain::workspace::Workspace;
use kjxlkj_domain::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// PostgreSQL-backed workspace repository.
pub struct PgWorkspaceRepo {
    pool: PgPool,
}

impl PgWorkspaceRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_workspace(&self, ws: &Workspace) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO workspaces (id, name, slug, owner_id, created_at)
             VALUES ($1,$2,$3,$4,$5)")
            .bind(ws.id).bind(&ws.name).bind(&ws.slug)
            .bind(ws.owner_user_id).bind(ws.created_at)
            .execute(&self.pool).await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn list_workspaces(
        &self,
        _user_id: Uuid,
    ) -> Result<Vec<PgWorkspaceRow>, DomainError> {
        // TODO: filter by membership when RBAC joins are added
        let rows = sqlx::query_as::<_, PgWorkspaceRow>(
            "SELECT id, name, slug, owner_id, created_at
             FROM workspaces ORDER BY created_at DESC")
            .fetch_all(&self.pool).await.map_err(pg_err)?;
        Ok(rows)
    }

    pub async fn get_workspace(
        &self,
        id: Uuid,
    ) -> Result<Option<PgWorkspaceRow>, DomainError> {
        let row = sqlx::query_as::<_, PgWorkspaceRow>(
            "SELECT id, name, slug, owner_id, created_at
             FROM workspaces WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(pg_err)?;
        Ok(row)
    }
}

/// Row type for workspace from PG.
#[derive(sqlx::FromRow)]
pub struct PgWorkspaceRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub owner_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
