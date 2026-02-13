use kjxlkj_db::models::automation::AutomationRuleRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_rule(
    pool: &PgPool,
    workspace_id: Uuid,
    name: &str,
    trigger: &str,
    condition_json: serde_json::Value,
    action_json: serde_json::Value,
) -> Result<AutomationRuleRow, DomainError> {
    validate_trigger(trigger)?;
    let id = Uuid::new_v4();
    repos::automation::create_rule(
        pool, id, workspace_id, name, trigger, &condition_json, &action_json,
    )
    .await
    .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn list_rules(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<AutomationRuleRow>, DomainError> {
    repos::automation::list_rules(pool, workspace_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn get_rule(
    pool: &PgPool,
    id: Uuid,
) -> Result<AutomationRuleRow, DomainError> {
    repos::automation::find_rule(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "automation_rule".into(),
        })
}

pub async fn update_rule(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    trigger: &str,
    condition_json: &serde_json::Value,
    action_json: &serde_json::Value,
    enabled: bool,
) -> Result<AutomationRuleRow, DomainError> {
    validate_trigger(trigger)?;
    repos::automation::update_rule(pool, id, name, trigger, condition_json, action_json, enabled)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "automation_rule".into(),
        })
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<(), DomainError> {
    repos::automation::delete_rule(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}

fn validate_trigger(trigger: &str) -> Result<(), DomainError> {
    let valid = [
        "note.created",
        "note.updated",
        "note.deleted",
        "workspace.event",
        "manual",
        "schedule",
    ];
    if !valid.contains(&trigger) {
        return Err(DomainError::RuleInvalid {
            reason: format!("unknown trigger: {trigger}"),
        });
    }
    Ok(())
}
