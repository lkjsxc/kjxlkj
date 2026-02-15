//! Operation safety policy per /docs/spec/domain/automation.md
//! and /docs/spec/technical/librarian-agent.md.

use crate::xml_parser::ParsedOperation;

/// Safety rejection reason.
#[derive(Debug, Clone)]
pub struct SafetyRejection {
    pub operation_id: String,
    pub reason: String,
}

/// Safety policy configuration.
#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    /// Whether delete operations are allowed (default: false).
    pub allow_delete: bool,
    /// Scope workspace ID — cross-workspace writes are rejected.
    pub scope_workspace_id: Option<String>,
    /// Maximum operations per run.
    pub max_operations: usize,
    /// Whether manual review is required before apply.
    pub require_review: bool,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self {
            allow_delete: false,
            scope_workspace_id: None,
            max_operations: 50,
            require_review: false,
        }
    }
}

/// Evaluate operations against safety policy.
/// Returns (accepted, rejected) split.
pub fn evaluate_operations(
    operations: &[ParsedOperation],
    policy: &SafetyPolicy,
) -> (Vec<ParsedOperation>, Vec<SafetyRejection>) {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for op in operations {
        if let Some(reason) = check_single_operation(op, policy) {
            rejected.push(SafetyRejection {
                operation_id: op.operation_id.clone(),
                reason,
            });
        } else {
            accepted.push(op.clone());
        }
    }

    (accepted, rejected)
}

fn check_single_operation(
    op: &ParsedOperation,
    policy: &SafetyPolicy,
) -> Option<String> {
    // Default policy prevents deletion operations.
    if !policy.allow_delete && op.kind.contains("delete") {
        return Some("delete operations not allowed by policy".into());
    }

    // Cross-workspace writes are rejected.
    if let Some(ref scope_ws) = policy.scope_workspace_id {
        if let Some(ref target_path) = op.target_path {
            if target_path.contains("workspace:")
                && !target_path.contains(scope_ws)
            {
                return Some(format!(
                    "cross-workspace write rejected: {target_path}"
                ));
            }
        }
    }

    None
}

/// Build safety policy from automation rule action_json.
pub fn policy_from_action(action: &serde_json::Value) -> SafetyPolicy {
    let mut policy = SafetyPolicy::default();

    if let Some(plan) = action.get("plan") {
        if let Some(max) = plan.get("max_operations").and_then(|v| v.as_u64()) {
            policy.max_operations = max as usize;
        }
        if let Some(strict) = plan.get("strict_mode").and_then(|v| v.as_bool()) {
            if strict {
                policy.allow_delete = false;
            }
        }
    }

    if let Some(scope) = action.get("scope").and_then(|v| v.as_str()) {
        if let Some(ws_id) = scope.strip_prefix("workspace:") {
            policy.scope_workspace_id = Some(ws_id.to_string());
        }
    }

    policy
}
