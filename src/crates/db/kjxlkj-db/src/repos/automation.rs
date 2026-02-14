use crate::models::{DbAutomationRule, DbAutomationRun};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateAutomationRuleInput {
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub actor_id: Uuid,
}

pub async fn list_rules(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<DbAutomationRule>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRule>(
        "SELECT id, workspace_id, trigger, condition_json, action_json, enabled,
                created_by, updated_by, created_at, updated_at
         FROM automation_rules
         WHERE workspace_id = $1
         ORDER BY updated_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn create_rule(
    pool: &PgPool,
    input: CreateAutomationRuleInput,
) -> Result<DbAutomationRule, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRule>(
        "INSERT INTO automation_rules
         (id, workspace_id, trigger, condition_json, action_json, enabled, created_by, updated_by)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
         RETURNING id, workspace_id, trigger, condition_json, action_json, enabled,
                   created_by, updated_by, created_at, updated_at",
    )
    .bind(Uuid::now_v7())
    .bind(input.workspace_id)
    .bind(input.trigger)
    .bind(input.condition_json)
    .bind(input.action_json)
    .bind(input.enabled)
    .bind(input.actor_id)
    .fetch_one(pool)
    .await
}

pub async fn get_rule(pool: &PgPool, rule_id: Uuid) -> Result<Option<DbAutomationRule>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRule>(
        "SELECT id, workspace_id, trigger, condition_json, action_json, enabled,
                created_by, updated_by, created_at, updated_at
         FROM automation_rules
         WHERE id = $1",
    )
    .bind(rule_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_rule(
    pool: &PgPool,
    rule_id: Uuid,
    trigger: Option<String>,
    condition_json: Option<serde_json::Value>,
    action_json: Option<serde_json::Value>,
    enabled: Option<bool>,
    actor_id: Uuid,
) -> Result<Option<DbAutomationRule>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRule>(
        "UPDATE automation_rules
         SET trigger = COALESCE($2, trigger),
             condition_json = COALESCE($3, condition_json),
             action_json = COALESCE($4, action_json),
             enabled = COALESCE($5, enabled),
             updated_by = $6,
             updated_at = NOW()
         WHERE id = $1
         RETURNING id, workspace_id, trigger, condition_json, action_json, enabled,
                   created_by, updated_by, created_at, updated_at",
    )
    .bind(rule_id)
    .bind(trigger)
    .bind(condition_json)
    .bind(action_json)
    .bind(enabled)
    .bind(actor_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_rule(pool: &PgPool, rule_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM automation_rules WHERE id = $1")
        .bind(rule_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_run(pool: &PgPool, run_id: Uuid) -> Result<Option<DbAutomationRun>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRun>(
    "SELECT id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                error_code, error_detail, started_at, finished_at, created_at
         FROM automation_runs
         WHERE id = $1",
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_runs(
    pool: &PgPool,
    workspace_id: Uuid,
    limit: i64,
) -> Result<Vec<DbAutomationRun>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRun>(
        "SELECT id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                error_code, error_detail, started_at, finished_at, created_at
         FROM automation_runs
         WHERE workspace_id = $1
         ORDER BY created_at DESC
         LIMIT $2",
    )
    .bind(workspace_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn list_enabled_rules_by_trigger(
    pool: &PgPool,
    workspace_id: Uuid,
    trigger: &str,
) -> Result<Vec<DbAutomationRule>, sqlx::Error> {
    sqlx::query_as::<_, DbAutomationRule>(
        "SELECT id, workspace_id, trigger, condition_json, action_json, enabled,
                created_by, updated_by, created_at, updated_at
         FROM automation_rules
         WHERE workspace_id = $1 AND trigger = $2 AND enabled = TRUE
         ORDER BY created_at ASC",
    )
    .bind(workspace_id)
    .bind(trigger)
    .fetch_all(pool)
    .await
}

pub async fn queue_run(
    pool: &PgPool,
    rule_id: Uuid,
    workspace_id: Uuid,
    triggering_event_id: &str,
    provider_kind: Option<&str>,
    model: Option<&str>,
    actor_id: Uuid,
) -> Result<DbAutomationRun, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let inserted = sqlx::query_as::<_, DbAutomationRun>(
        "INSERT INTO automation_runs
         (id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json)
         VALUES ($1, $2, $3, $4, 'queued', $5, $6, '{}'::jsonb)
         ON CONFLICT (rule_id, triggering_event_id) DO NOTHING
         RETURNING id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                   error_code, error_detail, started_at, finished_at, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(rule_id)
    .bind(workspace_id)
    .bind(triggering_event_id)
    .bind(provider_kind)
    .bind(model)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(run) = inserted {
        append_workspace_event_tx(
            &mut tx,
            workspace_id,
            actor_id,
            "automation_run_queued",
            json!({
                "event_code": "LIBRARIAN_RUN_QUEUED",
                "run_id": run.id,
                "rule_id": run.rule_id,
                "triggering_event_id": run.triggering_event_id,
                "status": run.status,
                "provider_kind": run.provider_kind,
                "model": run.model,
            }),
        )
        .await?;

        tx.commit().await?;
        return Ok(run);
    }

    let existing = sqlx::query_as::<_, DbAutomationRun>(
        "SELECT id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                error_code, error_detail, started_at, finished_at, created_at
         FROM automation_runs
         WHERE rule_id = $1 AND triggering_event_id = $2",
    )
    .bind(rule_id)
    .bind(triggering_event_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(existing)
}

pub async fn mark_run_running(
    pool: &PgPool,
    run_id: Uuid,
    actor_id: Uuid,
) -> Result<Option<DbAutomationRun>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let run = sqlx::query_as::<_, DbAutomationRun>(
        "UPDATE automation_runs
         SET status = 'running', started_at = NOW()
         WHERE id = $1 AND status = 'queued'
         RETURNING id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                   error_code, error_detail, started_at, finished_at, created_at",
    )
    .bind(run_id)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref run_row) = run {
        append_workspace_event_tx(
            &mut tx,
            run_row.workspace_id,
            actor_id,
            "automation_run_running",
            json!({
                "event_code": "LIBRARIAN_RUN_RUNNING",
                "run_id": run_row.id,
                "rule_id": run_row.rule_id,
                "triggering_event_id": run_row.triggering_event_id,
                "status": run_row.status,
                "provider_kind": run_row.provider_kind,
                "model": run_row.model,
            }),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(run)
}

pub async fn mark_run_succeeded(
    pool: &PgPool,
    run_id: Uuid,
    actor_id: Uuid,
    result_json: serde_json::Value,
) -> Result<Option<DbAutomationRun>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let run = sqlx::query_as::<_, DbAutomationRun>(
        "UPDATE automation_runs
         SET status = 'succeeded', result_json = $2, finished_at = NOW(),
             error_code = NULL, error_detail = NULL
         WHERE id = $1 AND status IN ('queued', 'running')
         RETURNING id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                   error_code, error_detail, started_at, finished_at, created_at",
    )
    .bind(run_id)
    .bind(result_json)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref run_row) = run {
        append_workspace_event_tx(
            &mut tx,
            run_row.workspace_id,
            actor_id,
            "automation_run_succeeded",
            automation_run_event_payload(run_row, "LIBRARIAN_RUN_SUCCEEDED"),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(run)
}

pub async fn mark_run_failed(
    pool: &PgPool,
    run_id: Uuid,
    actor_id: Uuid,
    error_code: &str,
    error_detail: &str,
    result_json: Option<serde_json::Value>,
) -> Result<Option<DbAutomationRun>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let run = sqlx::query_as::<_, DbAutomationRun>(
        "UPDATE automation_runs
         SET status = 'failed', error_code = $2, error_detail = $3,
             result_json = COALESCE($4, result_json),
             finished_at = NOW()
         WHERE id = $1 AND status IN ('queued', 'running')
         RETURNING id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                   error_code, error_detail, started_at, finished_at, created_at",
    )
    .bind(run_id)
    .bind(error_code)
    .bind(error_detail)
    .bind(result_json)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref run_row) = run {
        append_workspace_event_tx(
            &mut tx,
            run_row.workspace_id,
            actor_id,
            "automation_run_failed",
            automation_run_event_payload(run_row, "LIBRARIAN_RUN_FAILED"),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(run)
}

pub async fn record_run_review(
    pool: &PgPool,
    run_id: Uuid,
    actor_id: Uuid,
    apply: bool,
    decisions: Vec<serde_json::Value>,
    result_json: serde_json::Value,
) -> Result<Option<DbAutomationRun>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let run = sqlx::query_as::<_, DbAutomationRun>(
        "UPDATE automation_runs
         SET result_json = $2
         WHERE id = $1
         RETURNING id, rule_id, workspace_id, triggering_event_id, status, provider_kind, model, result_json,
                   error_code, error_detail, started_at, finished_at, created_at",
    )
    .bind(run_id)
    .bind(result_json)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref run_row) = run {
        append_workspace_event_tx(
            &mut tx,
            run_row.workspace_id,
            actor_id,
            "automation_run_reviewed",
            json!({
                "event_code": "LIBRARIAN_RUN_REVIEWED",
                "run_id": run_row.id,
                "rule_id": run_row.rule_id,
                "status": run_row.status,
                "apply": apply,
                "decisions": decisions,
                "applied_count": run_row
                    .result_json
                    .get("operation_report")
                    .and_then(|value| value.get("applied_operations"))
                    .and_then(|value| value.as_array())
                    .map_or(0, Vec::len),
                "rejected_count": run_row
                    .result_json
                    .get("operation_report")
                    .and_then(|value| value.get("rejected_operations"))
                    .and_then(|value| value.as_array())
                    .map_or(0, Vec::len),
            }),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(run)
}

async fn append_workspace_event_tx(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Uuid,
    actor_id: Uuid,
    event_type: &str,
    payload_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let seq: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(seq), 0) + 1 FROM workspace_events WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&mut **tx)
    .await?;

    sqlx::query(
        "INSERT INTO workspace_events (workspace_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(workspace_id)
    .bind(seq)
    .bind(event_type)
    .bind(payload_json)
    .bind(actor_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

fn automation_run_event_payload(
    run: &DbAutomationRun,
    event_code: &str,
) -> serde_json::Value {
    let operation_preview = operation_report_values(&run.result_json, "parsed_operations");
    let operation_applied = operation_report_values(&run.result_json, "applied_operations");
    let operation_rejected = operation_report_values(&run.result_json, "rejected_operations");

    json!({
        "event_code": event_code,
        "run_id": run.id,
        "rule_id": run.rule_id,
        "triggering_event_id": run.triggering_event_id,
        "status": run.status,
        "provider_kind": run.provider_kind,
        "model": run.model,
        "error_code": run.error_code,
        "error_detail": run.error_detail,
        "operation_preview": operation_preview,
        "operation_applied": operation_applied,
        "operation_rejected": operation_rejected,
        "operation_preview_count": operation_report_count(&run.result_json, "parsed_operations"),
        "operation_applied_count": operation_report_count(&run.result_json, "applied_operations"),
        "operation_rejected_count": operation_report_count(&run.result_json, "rejected_operations"),
    })
}

fn operation_report_count(result_json: &serde_json::Value, key: &str) -> usize {
    result_json
        .get("operation_report")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .map_or(0, Vec::len)
}

fn operation_report_values(result_json: &serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    result_json
        .get("operation_report")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default()
}
