//! Automation repository.

use sqlx::SqlitePool;
use uuid::Uuid;
use time::OffsetDateTime;
use serde_json::Value as JsonValue;

use kjxlkj_domain::{
    AutomationRule, AutomationRun, RuleState, RunState,
    AutomationTrigger, AutomationAction, LibrarianOperation,
};

/// Automation rule repository.
pub struct AutomationRuleRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AutomationRuleRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new rule.
    pub async fn create(&self, rule: &AutomationRule) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO automation_rules (id, workspace_id, name, trigger, condition, action, state, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(rule.id.to_string())
        .bind(rule.workspace_id.to_string())
        .bind(&rule.name)
        .bind(serde_json::to_string(&rule.trigger).unwrap())
        .bind(rule.condition.as_ref().map(|c| serde_json::to_string(c).unwrap()))
        .bind(serde_json::to_string(&rule.action).unwrap())
        .bind(serde_json::to_string(&rule.state).unwrap())
        .bind(rule.created_by.to_string())
        .bind(rule.created_at)
        .bind(rule.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find rule by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRule>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, String, Option<String>, String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, workspace_id, name, trigger, condition, action, state, created_by, created_at, updated_at
            FROM automation_rules WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, workspace_id, name, trigger, condition, action, state, created_by, created_at, updated_at)| AutomationRule {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
            name,
            trigger: serde_json::from_str(&trigger).unwrap(),
            condition: condition.map(|c| serde_json::from_str(&c).unwrap()),
            action: serde_json::from_str(&action).unwrap(),
            state: serde_json::from_str(&state).unwrap_or_default(),
            created_by: Uuid::parse_str(&created_by).unwrap_or_default(),
            created_at,
            updated_at,
        }))
    }

    /// List rules for workspace.
    pub async fn list_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<AutomationRule>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, String, Option<String>, String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, workspace_id, name, trigger, condition, action, state, created_by, created_at, updated_at
            FROM automation_rules WHERE workspace_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, workspace_id, name, trigger, condition, action, state, created_by, created_at, updated_at)| AutomationRule {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                name,
                trigger: serde_json::from_str(&trigger).unwrap(),
                condition: condition.map(|c| serde_json::from_str(&c).unwrap()),
                action: serde_json::from_str(&action).unwrap(),
                state: serde_json::from_str(&state).unwrap_or_default(),
                created_by: Uuid::parse_str(&created_by).unwrap_or_default(),
                created_at,
                updated_at,
            })
            .collect())
    }

    /// Update rule state.
    pub async fn update_state(&self, id: Uuid, state: RuleState) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE automation_rules SET state = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?
            "#,
        )
        .bind(serde_json::to_string(&state).unwrap())
        .bind(id.to_string())
        .execute(self.pool)
        .await?;
        Ok(())
    }
}

/// Automation run repository.
pub struct AutomationRunRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AutomationRunRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new run.
    pub async fn create(&self, run: &AutomationRun) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO automation_runs (id, rule_id, workspace_id, trigger_event, state, operations, raw_model_output, parse_diagnostics, provider_metadata, started_at, completed_at, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(run.id.to_string())
        .bind(run.rule_id.to_string())
        .bind(run.workspace_id.to_string())
        .bind(run.trigger_event.as_ref().map(|v| serde_json::to_string(v).unwrap()))
        .bind(serde_json::to_string(&run.state).unwrap())
        .bind(serde_json::to_string(&run.operations).unwrap())
        .bind(&run.raw_model_output)
        .bind(&run.parse_diagnostics)
        .bind(run.provider_metadata.as_ref().map(|v| serde_json::to_string(v).unwrap()))
        .bind(run.started_at)
        .bind(run.completed_at)
        .bind(run.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find run by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRun>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, String, Option<String>, Option<String>, Option<String>, Option<OffsetDateTime>, Option<OffsetDateTime>, OffsetDateTime)>(
            r#"
            SELECT id, rule_id, workspace_id, trigger_event, state, operations, raw_model_output, parse_diagnostics, provider_metadata, started_at, completed_at, created_at
            FROM automation_runs WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, rule_id, workspace_id, trigger_event, state, operations, raw_model_output, parse_diagnostics, provider_metadata, started_at, completed_at, created_at)| AutomationRun {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            rule_id: Uuid::parse_str(&rule_id).unwrap_or_default(),
            workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
            trigger_event: trigger_event.and_then(|v| serde_json::from_str(&v).ok()),
            state: serde_json::from_str(&state).unwrap_or_default(),
            operations: serde_json::from_str(&operations).unwrap_or_default(),
            raw_model_output,
            parse_diagnostics,
            provider_metadata: provider_metadata.and_then(|v| serde_json::from_str(&v).ok()),
            started_at,
            completed_at,
            created_at,
        }))
    }

    /// Update run.
    pub async fn update(&self, run: &AutomationRun) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE automation_runs SET state = ?, operations = ?, raw_model_output = ?, parse_diagnostics = ?, provider_metadata = ?, started_at = ?, completed_at = ?
            WHERE id = ?
            "#,
        )
        .bind(serde_json::to_string(&run.state).unwrap())
        .bind(serde_json::to_string(&run.operations).unwrap())
        .bind(&run.raw_model_output)
        .bind(&run.parse_diagnostics)
        .bind(run.provider_metadata.as_ref().map(|v| serde_json::to_string(v).unwrap()))
        .bind(run.started_at)
        .bind(run.completed_at)
        .bind(run.id.to_string())
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List runs for workspace.
    pub async fn list_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<AutomationRun>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, String, String, Option<String>, Option<String>, Option<String>, Option<OffsetDateTime>, Option<OffsetDateTime>, OffsetDateTime)>(
            r#"
            SELECT id, rule_id, workspace_id, trigger_event, state, operations, raw_model_output, parse_diagnostics, provider_metadata, started_at, completed_at, created_at
            FROM automation_runs WHERE workspace_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, rule_id, workspace_id, trigger_event, state, operations, raw_model_output, parse_diagnostics, provider_metadata, started_at, completed_at, created_at)| AutomationRun {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                rule_id: Uuid::parse_str(&rule_id).unwrap_or_default(),
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                trigger_event: trigger_event.and_then(|v| serde_json::from_str(&v).ok()),
                state: serde_json::from_str(&state).unwrap_or_default(),
                operations: serde_json::from_str(&operations).unwrap_or_default(),
                raw_model_output,
                parse_diagnostics,
                provider_metadata: provider_metadata.and_then(|v| serde_json::from_str(&v).ok()),
                started_at,
                completed_at,
                created_at,
            })
            .collect())
    }
}
