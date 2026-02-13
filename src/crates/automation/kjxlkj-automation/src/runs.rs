use kjxlkj_db::models::automation::AutomationRunRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::RunStatus;
use sqlx::PgPool;
use uuid::Uuid;

/// Launch an automation run for a given rule.
pub async fn launch_run(
    pool: &PgPool,
    rule_id: Uuid,
    workspace_id: Uuid,
    trigger_event_id: Option<Uuid>,
) -> Result<AutomationRunRow, DomainError> {
    let id = Uuid::new_v4();
    let run =
        repos::automation::create_run(pool, id, rule_id, workspace_id, trigger_event_id)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
    tracing::info!(run_id = %run.id, rule_id = %rule_id, "automation run launched");
    Ok(run)
}

/// Update the status of a run (used by background worker).
pub async fn update_run_status(
    pool: &PgPool,
    run_id: Uuid,
    status: RunStatus,
    result_json: Option<serde_json::Value>,
    error_message: Option<&str>,
) -> Result<(), DomainError> {
    repos::automation::update_run_status(
        pool,
        run_id,
        status,
        result_json.as_ref(),
        error_message,
    )
    .await
    .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}

pub async fn list_runs(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRunRow>, DomainError> {
    repos::automation::list_runs(pool, workspace_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn get_run(
    pool: &PgPool,
    id: Uuid,
) -> Result<AutomationRunRow, DomainError> {
    repos::automation::find_run(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "automation_run".into(),
        })
}
