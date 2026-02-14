// Saved views repository per /docs/spec/api/types.md
use kjxlkj_domain::types::SavedView;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_view(pool: &PgPool, v: &SavedView) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO saved_views (id, workspace_id, query_json, sort, filters, owner_user_id)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(v.id).bind(v.workspace_id).bind(&v.query_json)
    .bind(&v.sort).bind(&v.filters).bind(v.owner_user_id)
    .execute(pool).await?;
    Ok(())
}

pub async fn list_views(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<SavedView>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, serde_json::Value, Option<String>, Option<serde_json::Value>, Uuid)> =
        sqlx::query_as(
            "SELECT id, workspace_id, query_json, sort, filters, owner_user_id
             FROM saved_views WHERE workspace_id = $1",
        )
        .bind(workspace_id)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| SavedView {
        id: r.0, workspace_id: r.1, query_json: r.2,
        sort: r.3, filters: r.4, owner_user_id: r.5,
    }).collect())
}

pub async fn update_view(pool: &PgPool, id: Uuid, query: &serde_json::Value, sort: Option<&str>, filters: Option<&serde_json::Value>) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE saved_views SET query_json = $1, sort = $2, filters = $3 WHERE id = $4",
    )
    .bind(query).bind(sort).bind(filters).bind(id)
    .execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_view(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM saved_views WHERE id = $1")
        .bind(id).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}
