// Automation run orchestration per /docs/spec/domain/automation.md
// Run lifecycle: Queued → Running → Succeeded | Failed
// Runs MUST be idempotent per (rule_id, triggering_event_id).

use serde::{Deserialize, Serialize};

/// Run lifecycle states per /docs/spec/domain/automation.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

impl RunStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "queued" => Some(Self::Queued),
            "running" => Some(Self::Running),
            "succeeded" => Some(Self::Succeeded),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }

    /// Valid state transitions per spec.
    pub fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (Self::Queued, Self::Running)
                | (Self::Running, Self::Succeeded)
                | (Self::Running, Self::Failed)
        )
    }
}

/// Deterministic failure categories for provider interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFailure {
    AuthFailed,
    RateLimited,
    Timeout,
    Unreachable,
    InvalidPayload,
}

impl ProviderFailure {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AuthFailed => "auth_failed",
            Self::RateLimited => "rate_limited",
            Self::Timeout => "timeout",
            Self::Unreachable => "unreachable",
            Self::InvalidPayload => "invalid_payload",
        }
    }
}

/// Librarian operation kind per /docs/spec/api/librarian-xml.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    CreateNote,
    RewriteNote,
    RetitleNote,
    RelinkNote,
    RetagNote,
    Defer,
}

impl OperationKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "create_note" => Some(Self::CreateNote),
            "rewrite_note" => Some(Self::RewriteNote),
            "retitle_note" => Some(Self::RetitleNote),
            "relink_note" => Some(Self::RelinkNote),
            "retag_note" => Some(Self::RetagNote),
            "defer" => Some(Self::Defer),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreateNote => "create_note",
            Self::RewriteNote => "rewrite_note",
            Self::RetitleNote => "retitle_note",
            Self::RelinkNote => "relink_note",
            Self::RetagNote => "retag_note",
            Self::Defer => "defer",
        }
    }

    /// Strict-mode subset for small models per spec.
    pub fn is_strict_allowed(&self) -> bool {
        matches!(self, Self::CreateNote | Self::RewriteNote)
    }
}

/// Parsed librarian operation from xml_attrless response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianOperation {
    pub operation_id: String,
    pub kind: OperationKind,
    pub target_note_id: Option<String>,
    pub target_path: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub reason: String,
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_transitions() {
        assert!(RunStatus::Queued.can_transition_to(&RunStatus::Running));
        assert!(RunStatus::Running.can_transition_to(&RunStatus::Succeeded));
        assert!(RunStatus::Running.can_transition_to(&RunStatus::Failed));
        assert!(!RunStatus::Queued.can_transition_to(&RunStatus::Succeeded));
        assert!(!RunStatus::Succeeded.can_transition_to(&RunStatus::Running));
        assert!(!RunStatus::Failed.can_transition_to(&RunStatus::Running));
    }

    #[test]
    fn strict_mode_filtering() {
        assert!(OperationKind::CreateNote.is_strict_allowed());
        assert!(OperationKind::RewriteNote.is_strict_allowed());
        assert!(!OperationKind::RetitleNote.is_strict_allowed());
        assert!(!OperationKind::Defer.is_strict_allowed());
    }
}
