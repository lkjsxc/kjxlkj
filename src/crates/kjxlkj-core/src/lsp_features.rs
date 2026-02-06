/// LSP feature integration — completion, hover, diagnostics, code actions.

use std::collections::HashMap;

/// LSP completion item kind (subset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind { Text, Method, Function, Constructor, Field, Variable, Class, Interface, Module, Property, Keyword, Snippet, File }

/// A completion item from LSP.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
    pub sort_text: Option<String>,
}

impl CompletionItem {
    pub fn new(label: impl Into<String>, kind: CompletionKind) -> Self {
        Self { label: label.into(), kind, detail: None, insert_text: None, sort_text: None }
    }
    pub fn text_to_insert(&self) -> &str { self.insert_text.as_deref().unwrap_or(&self.label) }
}

/// Hover information.
#[derive(Debug, Clone)]
pub struct HoverInfo { pub contents: String, pub range: Option<(usize, usize)> }

/// Diagnostic severity from LSP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity { Error, Warning, Information, Hint }

/// A diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<String>,
}

/// Code action kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeActionKind { QuickFix, Refactor, RefactorExtract, RefactorInline, Source, SourceOrganizeImports }

/// A code action.
#[derive(Debug, Clone)]
pub struct CodeAction { pub title: String, pub kind: CodeActionKind, pub is_preferred: bool }

/// Buffer diagnostics store.
#[derive(Debug, Default)]
pub struct DiagnosticStore { by_buffer: HashMap<u64, Vec<Diagnostic>> }

impl DiagnosticStore {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, buf_id: u64, diags: Vec<Diagnostic>) { self.by_buffer.insert(buf_id, diags); }
    pub fn get(&self, buf_id: u64) -> &[Diagnostic] {
        self.by_buffer.get(&buf_id).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn clear(&mut self, buf_id: u64) { self.by_buffer.remove(&buf_id); }
    pub fn error_count(&self, buf_id: u64) -> usize {
        self.get(buf_id).iter().filter(|d| d.severity == DiagnosticSeverity::Error).count()
    }
    pub fn total_count(&self) -> usize { self.by_buffer.values().map(|v| v.len()).sum() }
}

/// Filter completions by prefix.
pub fn filter_completions<'a>(items: &'a [CompletionItem], prefix: &str) -> Vec<&'a CompletionItem> {
    let lower = prefix.to_lowercase();
    items.iter().filter(|i| i.label.to_lowercase().starts_with(&lower)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_item_insert() {
        let c = CompletionItem::new("println!", CompletionKind::Function);
        assert_eq!(c.text_to_insert(), "println!");
    }

    #[test]
    fn completion_with_insert_text() {
        let mut c = CompletionItem::new("println!", CompletionKind::Snippet);
        c.insert_text = Some("println!(\"$1\")".into());
        assert_eq!(c.text_to_insert(), "println!(\"$1\")");
    }

    #[test]
    fn filter_completions_test() {
        let items = vec![
            CompletionItem::new("foo", CompletionKind::Function),
            CompletionItem::new("bar", CompletionKind::Variable),
            CompletionItem::new("fooBar", CompletionKind::Method),
        ];
        assert_eq!(filter_completions(&items, "foo").len(), 2);
    }

    #[test]
    fn diagnostic_store() {
        let mut store = DiagnosticStore::new();
        store.set(1, vec![Diagnostic { line: 1, col: 0, end_line: 1, end_col: 5,
            severity: DiagnosticSeverity::Error, message: "err".into(), source: None, code: None }]);
        assert_eq!(store.error_count(1), 1);
        assert_eq!(store.total_count(), 1);
    }

    #[test]
    fn diagnostic_clear() {
        let mut store = DiagnosticStore::new();
        store.set(1, vec![]); store.clear(1);
        assert_eq!(store.get(1).len(), 0);
    }

    #[test]
    fn hover_info() {
        let h = HoverInfo { contents: "fn foo()".into(), range: Some((0, 3)) };
        assert!(h.range.is_some());
    }

    #[test]
    fn code_action_kinds() {
        let a = CodeAction { title: "Remove unused".into(), kind: CodeActionKind::QuickFix, is_preferred: true };
        assert!(a.is_preferred);
    }

    #[test]
    fn severity_ordering() {
        assert!(DiagnosticSeverity::Error < DiagnosticSeverity::Warning);
        assert!(DiagnosticSeverity::Warning < DiagnosticSeverity::Hint);
    }
}
