//! Diagnostic store for LSP diagnostics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Diagnostic severity levels (ordered by severity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A single diagnostic entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub line: usize,
    pub col: usize,
}

/// Per-buffer diagnostic store.
#[derive(Debug, Default)]
pub struct DiagnosticStore {
    /// Map from buffer URI to its diagnostics.
    store: HashMap<String, Vec<Diagnostic>>,
}

impl DiagnosticStore {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    /// Add a diagnostic for the given buffer.
    pub fn add(&mut self, uri: &str, diag: Diagnostic) {
        self.store.entry(uri.to_string()).or_default().push(diag);
    }

    /// Remove all diagnostics for the given buffer.
    pub fn remove(&mut self, uri: &str) {
        self.store.remove(uri);
    }

    /// Get diagnostics for a buffer.
    pub fn get(&self, uri: &str) -> &[Diagnostic] {
        self.store.get(uri).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Count of Error-severity diagnostics across all buffers.
    pub fn error_count(&self) -> usize {
        self.store
            .values()
            .flat_map(|v| v.iter())
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .count()
    }

    /// Total diagnostic count across all buffers.
    pub fn total_count(&self) -> usize {
        self.store.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get() {
        let mut store = DiagnosticStore::new();
        store.add("file:///a.rs", Diagnostic {
            message: "unused".into(),
            severity: DiagnosticSeverity::Warning,
            line: 1,
            col: 0,
        });
        assert_eq!(store.get("file:///a.rs").len(), 1);
        assert_eq!(store.total_count(), 1);
        assert_eq!(store.error_count(), 0);
    }

    #[test]
    fn remove_clears() {
        let mut store = DiagnosticStore::new();
        store.add("file:///b.rs", Diagnostic {
            message: "err".into(),
            severity: DiagnosticSeverity::Error,
            line: 5,
            col: 2,
        });
        assert_eq!(store.error_count(), 1);
        store.remove("file:///b.rs");
        assert_eq!(store.total_count(), 0);
    }

    #[test]
    fn severity_ordering() {
        assert!(DiagnosticSeverity::Error < DiagnosticSeverity::Warning);
        assert!(DiagnosticSeverity::Warning < DiagnosticSeverity::Hint);
    }
}
