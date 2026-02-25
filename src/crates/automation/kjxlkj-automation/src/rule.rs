//! Automation rule management

use chrono::{DateTime, Utc};
use uuid::Uuid;

use kjxlkj_domain::{AutomationRule, RuleTrigger, RuleAction};
use kjxlkj_db::Result;

/// Rule repository trait
pub trait RuleRepo: Send + Sync {
    fn create_rule(&self, rule: AutomationRule) -> impl Future<Output = Result<AutomationRule>>;
    fn get_rule(&self, rule_id: Uuid) -> impl Future<Output = Result<Option<AutomationRule>>>;
    fn list_rules(&self, workspace_id: Uuid) -> impl Future<Output = Result<Vec<AutomationRule>>>;
    fn update_rule(&self, rule: AutomationRule) -> impl Future<Output = Result<AutomationRule>>;
    fn delete_rule(&self, rule_id: Uuid) -> impl Future<Output = Result<()>>;
}

use std::future::Future;
