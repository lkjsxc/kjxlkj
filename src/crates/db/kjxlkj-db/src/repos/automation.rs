use crate::models::automation::{AutomationRuleRow, AutomationRunRow};
use kjxlkj_domain::types::RunStatus;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_rule(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    name: &str,
    trigger: &str,
    condition_json: &serde_json::Value,
    action_json: &serde_json::Value,
) -> Result<AutomationRuleRow, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "INSERT INTO automation_rules
         (id, workspace_id, name, trigger, condition_json, action_json)
         VALUES ($1,$2,$3,$4,$5,$6) RETURNING *",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(name)
    .bind(trigger)
    .bind(condition_json)
    .bind(action_json)
    .fetch_one(pool)
    .await
}

pub async fn list_rules(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "SELECT * FROM automation_rules WHERE workspace_id = $1 ORDER BY created_at",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn find_rule(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<AutomationRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "SELECT * FROM automation_rules WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update_rule(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    trigger: &str,
    condition_json: &serde_json::Value,
    action_json: &serde_json::Value,
    enabled: bool,
) -> Result<Option<AutomationRuleRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRuleRow>(
        "UPDATE automation_rules SET name=$2, trigger=$3, condition_json=$4,
         action_json=$5, enabled=$6, updated_at=now()
         WHERE id=$1 RETURNING *",
    )
    .bind(id)
    .bind(name)
    .bind(trigger)
    .bind(condition_json)
    .bind(action_json)
    .bind(enabled)
    .fetch_optional(pool)
    .await
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM automation_rules WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(r.rows_affected() > 0)
}

pub async fn create_run(
    pool: &PgPool,
    id: Uuid,
    rule_id: Uuid,
    workspace_id: Uuid,
    trigger_event_id: Option<Uuid>,
) -> Result<AutomationRunRow, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "INSERT INTO automation_runs (id, rule_id, workspace_id,
         status, trigger_event_id)
         VALUES ($1,$2,$3,'queued'::text,$4) RETURNING *",
    )
    .bind(id)
    .bind(rule_id)
    .bind(workspace_id)
    .bind(trigger_event_id)
    .fetch_one(pool)
    .await
}

pub async fn update_run_status(
    pool: &PgPool,
    id: Uuid,
    status: RunStatus,
    result_json: Option<&serde_json::Value>,
    error_message: Option<&str>,
) -> Result<Option<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "UPDATE automation_runs SET status=$2::text, result_json=$3,
         error_message=$4, updated_at=now()
         WHERE id=$1 RETURNING *",
    )
    .bind(id)
    .bind(status)
    .bind(result_json)
    .bind(error_message)
    .fetch_optional(pool)
    .await
}

pub async fn list_runs(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "SELECT * FROM automation_runs WHERE workspace_id = $1
         ORDER BY created_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn find_run(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "SELECT * FROM automation_runs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
