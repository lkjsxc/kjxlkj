// Automation repository per /docs/spec/domain/automation.md
use kjxlkj_domain::types::{AutomationRule, AutomationRun, RunStatus};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_rule(pool: &PgPool, r: &AutomationRule) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO automation_rules (id, workspace_id, trigger, condition_json, action_json, enabled)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(r.id).bind(r.workspace_id).bind(&r.trigger)
    .bind(&r.condition_json).bind(&r.action_json).bind(r.enabled)
    .execute(pool).await?;
    Ok(())
}

pub async fn list_rules(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<AutomationRule>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, String, serde_json::Value, serde_json::Value, bool)> =
        sqlx::query_as(
            "SELECT id, workspace_id, trigger, condition_json, action_json, enabled
             FROM automation_rules WHERE workspace_id = $1",
        )
        .bind(workspace_id)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| AutomationRule {
        id: r.0, workspace_id: r.1, trigger: r.2,
        condition_json: r.3, action_json: r.4, enabled: r.5,
    }).collect())
}

pub async fn update_rule(pool: &PgPool, id: Uuid, action: &serde_json::Value, enabled: bool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE automation_rules SET action_json = $1, enabled = $2 WHERE id = $3",
    )
    .bind(action).bind(enabled).bind(id)
    .execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM automation_rules WHERE id = $1")
        .bind(id).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn insert_run(pool: &PgPool, run: &AutomationRun) -> Result<(), sqlx::Error> {
    let status_str = run_status_str(run.status);
    sqlx::query(
        "INSERT INTO automation_runs (id, rule_id, status, started_at)
         VALUES ($1, $2, $3, now())",
    )
    .bind(run.id).bind(run.rule_id).bind(status_str)
    .execute(pool).await?;
    Ok(())
}

pub async fn list_runs(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<AutomationRun>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT ar.id, ar.rule_id, ar.status
         FROM automation_runs ar
         JOIN automation_rules rl ON rl.id = ar.rule_id
         WHERE rl.workspace_id = $1
         ORDER BY ar.started_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| AutomationRun {
        id: r.0, rule_id: r.1, status: parse_run_status(&r.2),
        started_at: String::new(), finished_at: None, result_json: None,
    }).collect())
}

fn run_status_str(s: RunStatus) -> &'static str {
    match s {
        RunStatus::Pending => "pending",
        RunStatus::Running => "running",
        RunStatus::Completed => "completed",
        RunStatus::Failed => "failed",
        RunStatus::Cancelled => "cancelled",
    }
}

fn parse_run_status(s: &str) -> RunStatus {
    match s {
        "running" => RunStatus::Running,
        "completed" => RunStatus::Completed,
        "failed" => RunStatus::Failed,
        "cancelled" => RunStatus::Cancelled,
        _ => RunStatus::Pending,
    }
}
