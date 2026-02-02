//! LSP protocol types.

use kjxlkj_core_types::Range;
use serde::{Deserialize, Serialize};

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    /// Error.
    Error = 1,
    /// Warning.
    Warning = 2,
    /// Information.
    Information = 3,
    /// Hint.
    Hint = 4,
}

/// A diagnostic message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Range in the document.
    pub range: Range,
    /// Severity.
    pub severity: Option<DiagnosticSeverity>,
    /// Message.
    pub message: String,
    /// Source.
    pub source: Option<String>,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    pub fn new(range: Range, message: impl Into<String>) -> Self {
        Self {
            range,
            severity: None,
            message: message.into(),
            source: None,
        }
    }

    /// Sets the severity.
    pub fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
        self.severity = Some(severity);
        self
    }
}
