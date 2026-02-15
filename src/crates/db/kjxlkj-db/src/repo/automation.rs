//! Automation rule/run repository per /docs/spec/domain/automation.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RuleRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, FromRow)]
pub struct RunRow {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub result_json: Option<serde_json::Value>,
}

// --- Rules ---

pub async fn create_rule(
    pool: &PgPool, id: Uuid, ws_id: Uuid, trigger: &str,
    condition: &serde_json::Value, action: &serde_json::Value,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO automation_rules (id,workspace_id,trigger,condition_json,action_json,enabled) \
         VALUES ($1,$2,$3,$4,$5,$6)"
    ).bind(id).bind(ws_id).bind(trigger).bind(condition).bind(action).bind(enabled)
     .execute(pool).await?;
    Ok(())
}

pub async fn list_rules(pool: &PgPool, ws_id: Uuid) -> Result<Vec<RuleRow>, sqlx::Error> {
    sqlx::query_as::<_, RuleRow>(
        "SELECT id, workspace_id, trigger, condition_json, action_json, \
         enabled, created_at::text as created_at \
         FROM automation_rules WHERE workspace_id = $1 ORDER BY created_at"
    ).bind(ws_id).fetch_all(pool).await
}

pub async fn get_rule(pool: &PgPool, id: Uuid) -> Result<Option<RuleRow>, sqlx::Error> {
    sqlx::query_as::<_, RuleRow>(
        "SELECT id, workspace_id, trigger, condition_json, action_json, \
         enabled, created_at::text as created_at \
         FROM automation_rules WHERE id = $1"
    ).bind(id).fetch_optional(pool).await
}

pub async fn update_rule(
    pool: &PgPool, id: Uuid, trigger: &str,
    condition: &serde_json::Value, action: &serde_json::Value,
    enabled: bool,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE automation_rules SET trigger=$1,condition_json=$2,action_json=$3,enabled=$4 \
         WHERE id=$5"
    ).bind(trigger).bind(condition).bind(action).bind(enabled).bind(id)
     .execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM automation_rules WHERE id=$1")
        .bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// --- Runs ---

pub async fn create_run(
    pool: &PgPool, id: Uuid, rule_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO automation_runs (id, rule_id) VALUES ($1, $2)"
    ).bind(id).bind(rule_id).execute(pool).await?;
    Ok(())
}

pub async fn update_run_status(
    pool: &PgPool, id: Uuid, status: &str,
    result_json: Option<&serde_json::Value>,
) -> Result<(), sqlx::Error> {
    if status == "succeeded" || status == "failed" {
        sqlx::query(
            "UPDATE automation_runs SET status=$1, result_json=$2, finished_at=NOW() WHERE id=$3"
        ).bind(status).bind(result_json).bind(id)
         .execute(pool).await?;
    } else {
        sqlx::query(
            "UPDATE automation_runs SET status=$1 WHERE id=$2"
        ).bind(status).bind(id).execute(pool).await?;
    }
    Ok(())
}

pub async fn list_runs(pool: &PgPool, ws_id: Uuid) -> Result<Vec<RunRow>, sqlx::Error> {
    sqlx::query_as::<_, RunRow>(
        "SELECT r.id, r.rule_id, r.status, \
         r.started_at::text as started_at, \
         r.finished_at::text as finished_at, \
         r.result_json \
         FROM automation_runs r \
         JOIN automation_rules ar ON r.rule_id = ar.id \
         WHERE ar.workspace_id = $1 \
         ORDER BY r.started_at DESC"
    ).bind(ws_id).fetch_all(pool).await
}

pub async fn get_run(pool: &PgPool, id: Uuid) -> Result<Option<RunRow>, sqlx::Error> {
    sqlx::query_as::<_, RunRow>(
        "SELECT id, rule_id, status, \
         started_at::text as started_at, \
         finished_at::text as finished_at, \
         result_json \
         FROM automation_runs WHERE id = $1"
    ).bind(id).fetch_optional(pool).await
}

// --- Jobs ---

#[derive(Debug, FromRow)]
pub struct JobRow {
    pub id: Uuid,
    pub job_type: String,
    pub status: String,
    pub workspace_id: Option<Uuid>,
    pub result_json: Option<serde_json::Value>,
    pub created_at: String,
    pub finished_at: Option<String>,
}

pub async fn create_job(
    pool: &PgPool, id: Uuid, job_type: &str, ws_id: Option<Uuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO jobs (id,job_type,workspace_id) VALUES ($1,$2,$3)"
    ).bind(id).bind(job_type).bind(ws_id)
     .execute(pool).await?;
    Ok(())
}

pub async fn update_job(
    pool: &PgPool, id: Uuid, status: &str,
    result_json: Option<&serde_json::Value>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE jobs SET status=$1, result_json=$2, finished_at=NOW() WHERE id=$3"
    ).bind(status).bind(result_json).bind(id)
     .execute(pool).await?;
    Ok(())
}

pub async fn update_run_result(
    pool: &PgPool, id: Uuid, result: &serde_json::Value,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE automation_runs SET result_json=$1 WHERE id=$2"
    ).bind(result).bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn get_job(pool: &PgPool, id: Uuid) -> Result<Option<JobRow>, sqlx::Error> {
    sqlx::query_as::<_, JobRow>(
        "SELECT id, job_type, status, workspace_id, \
         result_json, created_at::text as created_at, \
         finished_at::text as finished_at \
         FROM jobs WHERE id = $1"
    ).bind(id).fetch_optional(pool).await
}
