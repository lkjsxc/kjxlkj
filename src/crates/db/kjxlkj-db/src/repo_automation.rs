use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(FromRow, serde::Serialize)]
pub struct AutoRuleRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger_kind: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow, serde::Serialize)]
pub struct AutoRunRow {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: String,
    pub started_at: OffsetDateTime,
    pub finished_at: Option<OffsetDateTime>,
    pub result_json: Option<serde_json::Value>,
}

/// Create automation rule.
pub async fn create_rule(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    trigger_kind: &str,
    condition_json: &serde_json::Value,
    action_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO automation_rules
         (id, workspace_id, trigger_kind, condition_json, action_json)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(workspace_id)
    .bind(trigger_kind)
    .bind(condition_json)
    .bind(action_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// List rules for workspace.
pub async fn list_rules(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutoRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutoRuleRow>(
        "SELECT id, workspace_id, trigger_kind, condition_json,
                action_json, enabled, created_at
         FROM automation_rules
         WHERE workspace_id = $1
         ORDER BY created_at DESC"
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

/// Create a run for a rule.
pub async fn create_run(
    pool: &PgPool,
    id: Uuid,
    rule_id: Uuid,
    idempotency_key: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO automation_runs (id, rule_id, idempotency_key)
         VALUES ($1, $2, $3)
         ON CONFLICT (rule_id, idempotency_key) DO NOTHING"
    )
    .bind(id)
    .bind(rule_id)
    .bind(idempotency_key)
    .execute(pool)
    .await?;
    Ok(())
}

/// Update run status.
pub async fn update_run_status(
    pool: &PgPool,
    run_id: Uuid,
    status: &str,
    result_json: Option<&serde_json::Value>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE automation_runs
         SET status = $2, finished_at = CASE WHEN $2 IN ('succeeded','failed') THEN now() ELSE finished_at END,
             result_json = COALESCE($3, result_json)
         WHERE id = $1"
    )
    .bind(run_id)
    .bind(status)
    .bind(result_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// Get run by id.
pub async fn get_run(
    pool: &PgPool,
    run_id: Uuid,
) -> Result<Option<AutoRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutoRunRow>(
        "SELECT id, rule_id, status, started_at, finished_at, result_json
         FROM automation_runs WHERE id = $1"
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await
}

// --- Agent KV Store ---

#[derive(FromRow)]
pub struct KvRow {
    pub key: String,
    pub value: serde_json::Value,
}

/// Get agent KV entry.
pub async fn kv_get(
    pool: &PgPool,
    agent_name: &str,
    workspace_id: Uuid,
    key: &str,
) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        "SELECT value FROM agent_kv_store
         WHERE agent_name = $1 AND workspace_id = $2 AND key = $3"
    )
    .bind(agent_name)
    .bind(workspace_id)
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

/// Upsert agent KV entry.
pub async fn kv_set(
    pool: &PgPool,
    agent_name: &str,
    workspace_id: Uuid,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO agent_kv_store (agent_name, workspace_id, key, value)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (agent_name, workspace_id, key) DO UPDATE SET value = $4, updated_at = now()"
    )
    .bind(agent_name)
    .bind(workspace_id)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete agent KV entry.
pub async fn kv_delete(
    pool: &PgPool,
    agent_name: &str,
    workspace_id: Uuid,
    key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM agent_kv_store
         WHERE agent_name = $1 AND workspace_id = $2 AND key = $3"
    )
    .bind(agent_name)
    .bind(workspace_id)
    .bind(key)
    .execute(pool)
    .await?;
    Ok(())
}

/// List all agent KV entries for a workspace.
pub async fn kv_list(
    pool: &PgPool,
    agent_name: &str,
    workspace_id: Uuid,
) -> Result<Vec<KvRow>, sqlx::Error> {
    sqlx::query_as::<_, KvRow>(
        "SELECT key, value FROM agent_kv_store
         WHERE agent_name = $1 AND workspace_id = $2
         ORDER BY key"
    )
    .bind(agent_name)
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}
