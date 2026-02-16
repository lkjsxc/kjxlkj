/// In-memory AutomationRepo implementation.
///
/// Spec: /docs/spec/domain/automation.md
use crate::repo::AutomationRepo;
use kjxlkj_domain::automation::*;
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Thread-safe in-memory automation store.
pub struct InMemoryAutomationRepo {
    rules: RwLock<HashMap<Uuid, AutomationRule>>,
    runs: RwLock<HashMap<Uuid, AutomationRun>>,
}

impl InMemoryAutomationRepo {
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(HashMap::new()),
            runs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryAutomationRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomationRepo for InMemoryAutomationRepo {
    fn create_rule(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        let mut rules = self.rules.write().unwrap();
        rules.insert(rule.id, rule.clone());
        Ok(())
    }

    fn list_rules(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<AutomationRule>, DomainError> {
        let rules = self.rules.read().unwrap();
        let results: Vec<AutomationRule> = rules
            .values()
            .filter(|r| workspace_id.is_nil() || r.workspace_id == workspace_id)
            .cloned()
            .collect();
        Ok(results)
    }

    fn update_rule(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        let mut rules = self.rules.write().unwrap();
        rules.insert(rule.id, rule.clone());
        Ok(())
    }

    fn create_run(&self, run: &AutomationRun) -> Result<(), DomainError> {
        let mut runs = self.runs.write().unwrap();
        runs.insert(run.id, run.clone());
        Ok(())
    }

    fn list_runs(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<AutomationRun>, DomainError> {
        let runs = self.runs.read().unwrap();
        if workspace_id.is_nil() {
            return Ok(runs.values().cloned().collect());
        }
        let rules = self.rules.read().unwrap();
        let ws_rule_ids: Vec<Uuid> = rules
            .values()
            .filter(|r| r.workspace_id == workspace_id)
            .map(|r| r.id)
            .collect();
        let results: Vec<AutomationRun> = runs
            .values()
            .filter(|r| ws_rule_ids.contains(&r.rule_id))
            .cloned()
            .collect();
        Ok(results)
    }

    fn get_run(&self, id: Uuid) -> Result<Option<AutomationRun>, DomainError> {
        let runs = self.runs.read().unwrap();
        Ok(runs.get(&id).cloned())
    }

    fn update_run(&self, run: &AutomationRun) -> Result<(), DomainError> {
        let mut runs = self.runs.write().unwrap();
        runs.insert(run.id, run.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_crud() {
        let repo = InMemoryAutomationRepo::new();
        let ws_id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();
        let rule = AutomationRule {
            id: Uuid::new_v4(),
            workspace_id: ws_id,
            trigger: "note_created".into(),
            condition_json: serde_json::json!({}),
            action_json: serde_json::json!({"kind": "kjxlkj_agent"}),
            enabled: true,
            created_at: now,
            updated_at: now,
        };
        repo.create_rule(&rule).unwrap();
        let list = repo.list_rules(ws_id).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_run_lifecycle() {
        let repo = InMemoryAutomationRepo::new();
        let ws_id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();
        let rule = AutomationRule {
            id: Uuid::new_v4(),
            workspace_id: ws_id,
            trigger: "manual".into(),
            condition_json: serde_json::json!({}),
            action_json: serde_json::json!({}),
            enabled: true,
            created_at: now,
            updated_at: now,
        };
        repo.create_rule(&rule).unwrap();
        let run = AutomationRun {
            id: Uuid::new_v4(),
            rule_id: rule.id,
            status: RunStatus::Queued,
            started_at: None,
            finished_at: None,
            result_json: None,
            created_at: now,
        };
        repo.create_run(&run).unwrap();
        let runs = repo.list_runs(ws_id).unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].status, RunStatus::Queued);
    }
}
