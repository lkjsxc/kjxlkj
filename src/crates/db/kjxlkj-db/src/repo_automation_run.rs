use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Per /docs/spec/domain/automation.md: run lifecycle.
/// States: queued → running → succeeded | failed.
#[derive(Debug, FromRow)]
pub struct AutomationRunRow {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: String,
    pub triggering_event_id: Option<Uuid>,
    pub result_json: JsonValue,
}

/// Create run with idempotency per (rule_id, triggering_event_id).
pub async fn create_run(
    pool: &PgPool,
    id: Uuid,
    rule_id: Uuid,
    triggering_event_id: Option<Uuid>,
) -> Result<AutomationRunRow, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "INSERT INTO automation_runs (id, rule_id, status, triggering_event_id)
         VALUES ($1, $2, 'queued', $3)
         ON CONFLICT (rule_id, triggering_event_id) DO NOTHING
         RETURNING id, rule_id, status, triggering_event_id, result_json",
    )
    .bind(id)
    .bind(rule_id)
    .bind(triggering_event_id)
    .fetch_one(pool)
    .await
}

/// Find existing run for idempotency check.
pub async fn find_run_by_trigger(
    pool: &PgPool,
    rule_id: Uuid,
    triggering_event_id: Uuid,
) -> Result<Option<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "SELECT id, rule_id, status, triggering_event_id, result_json
         FROM automation_runs
         WHERE rule_id = $1 AND triggering_event_id = $2",
    )
    .bind(rule_id)
    .bind(triggering_event_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_run(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "SELECT id, rule_id, status, triggering_event_id, result_json
         FROM automation_runs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_runs(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRunRow>, sqlx::Error> {
    sqlx::query_as::<_, AutomationRunRow>(
        "SELECT r.id, r.rule_id, r.status, r.triggering_event_id, r.result_json
         FROM automation_runs r
         JOIN automation_rules a ON a.id = r.rule_id
         WHERE a.workspace_id = $1
         ORDER BY r.created_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

/// Transition run to 'running'.
pub async fn start_run(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE automation_runs SET status = 'running', started_at = now() WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Transition run to 'succeeded'.
pub async fn complete_run(
    pool: &PgPool,
    id: Uuid,
    result_json: &JsonValue,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE automation_runs SET status = 'succeeded', finished_at = now(), result_json = $2
         WHERE id = $1",
    )
    .bind(id)
    .bind(result_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// Transition run to 'failed'.
pub async fn fail_run(
    pool: &PgPool,
    id: Uuid,
    result_json: &JsonValue,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE automation_runs SET status = 'failed', finished_at = now(), result_json = $2
         WHERE id = $1",
    )
    .bind(id)
    .bind(result_json)
    .execute(pool)
    .await?;
    Ok(())
}
