//! LSP feature types re-exported from the core crate.

use serde::{Deserialize, Serialize};

/// Completion item for LSP-style autocompletion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
}

/// Completion kind following LSP spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Keyword,
    Snippet,
    File,
}

/// Hover information from LSP.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverInfo {
    pub contents: String,
    pub language: Option<String>,
}

/// A diagnostic message (error, warning, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub line: usize,
    pub col: usize,
    pub end_line: Option<usize>,
    pub end_col: Option<usize>,
    pub source: Option<String>,
    pub code: Option<String>,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Storage for diagnostics per buffer.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticStore {
    entries: Vec<Diagnostic>,
}

impl DiagnosticStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, diagnostics: Vec<Diagnostic>) {
        self.entries = diagnostics;
    }

    pub fn get(&self) -> &[Diagnostic] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn error_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Warning)
            .count()
    }
}

/// A code action from LSP.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub is_preferred: bool,
}

/// Code action kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceOrganizeImports,
    SourceFixAll,
}

/// Filter completion items by a prefix string.
pub fn filter_completions(items: &[CompletionItem], prefix: &str) -> Vec<CompletionItem> {
    if prefix.is_empty() {
        return items.to_vec();
    }
    let lower = prefix.to_lowercase();
    items
        .iter()
        .filter(|item| {
            let text = item.filter_text.as_deref().unwrap_or(&item.label);
            text.to_lowercase().starts_with(&lower)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "lsp_features_tests.rs"]
mod tests;
