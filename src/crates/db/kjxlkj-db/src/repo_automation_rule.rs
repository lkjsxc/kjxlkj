use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Per /docs/spec/domain/automation.md: rule model.
#[derive(Debug, FromRow)]
pub struct AutomationRuleRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger_type: String,
    pub condition_json: JsonValue,
    pub action_json: JsonValue,
    pub enabled: bool,
}

pub async fn create_rule(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    name: &str,
    trigger_type: &str,
    condition_json: &JsonValue,
    action_json: &JsonValue,
) -> Result<AutomationRuleRow, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "INSERT INTO automation_rules (id, workspace_id, name, trigger_type, condition_json, action_json)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, workspace_id, name, trigger_type, condition_json, action_json, enabled",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(name)
    .bind(trigger_type)
    .bind(condition_json)
    .bind(action_json)
    .fetch_one(pool)
    .await
}

pub async fn find_rule(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<AutomationRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "SELECT id, workspace_id, name, trigger_type, condition_json, action_json, enabled
         FROM automation_rules WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_rules(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "SELECT id, workspace_id, name, trigger_type, condition_json, action_json, enabled
         FROM automation_rules WHERE workspace_id = $1
         ORDER BY created_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn update_rule(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    trigger_type: &str,
    condition_json: &JsonValue,
    action_json: &JsonValue,
    enabled: bool,
) -> Result<AutomationRuleRow, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "UPDATE automation_rules
         SET name = $2, trigger_type = $3, condition_json = $4,
             action_json = $5, enabled = $6, updated_at = now()
         WHERE id = $1
         RETURNING id, workspace_id, name, trigger_type, condition_json, action_json, enabled",
    )
    .bind(id)
    .bind(name)
    .bind(trigger_type)
    .bind(condition_json)
    .bind(action_json)
    .bind(enabled)
    .fetch_one(pool)
    .await
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM automation_rules WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
