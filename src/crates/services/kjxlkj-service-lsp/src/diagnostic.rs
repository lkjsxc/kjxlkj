//! Diagnostic data model shared between LSP, build, and index sources.
//! See /docs/spec/features/lsp/diagnostics.md.

/// Diagnostic severity levels (LSP convention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity { Error = 1, Warning = 2, Info = 3, Hint = 4 }

/// Source category for a diagnostic item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticKind { Diagnostic, Build, Grep, Todo, Quickfix }

/// Position span within a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLocation {
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub end_line: Option<u32>,
    pub end_col: Option<u32>,
}

/// A single diagnostic item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub id: u64,
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub location: DiagnosticLocation,
    pub message: String,
    pub source: String,
    pub code: Option<String>,
}

/// Per-file diagnostic store with push/replace semantics.
#[derive(Debug, Default)]
pub struct DiagnosticStore {
    items: Vec<Diagnostic>,
    next_id: u64,
}

impl DiagnosticStore {
    pub fn new() -> Self { Self { items: Vec::new(), next_id: 0 } }
    /// Replace all diagnostics for a file+source (LSP push semantics).
    pub fn replace_for_file(&mut self, file: &str, source: &str, diags: Vec<Diagnostic>) {
        self.items.retain(|d| !(d.location.file == file && d.source == source));
        self.items.extend(diags);
        self.sort();
    }
    /// Append items (grep/index incremental semantics).
    pub fn append(&mut self, diags: Vec<Diagnostic>) {
        self.items.extend(diags);
        self.sort();
    }
    /// Allocate a unique ID for a new diagnostic.
    pub fn alloc_id(&mut self) -> u64 { let id = self.next_id; self.next_id += 1; id }
    /// Get all diagnostics, sorted by severity → file → line.
    pub fn all(&self) -> &[Diagnostic] { &self.items }
    /// Get diagnostics for a specific file.
    pub fn for_file(&self, file: &str) -> Vec<&Diagnostic> {
        self.items.iter().filter(|d| d.location.file == file).collect()
    }
    /// Count by severity.
    pub fn count_by_severity(&self, sev: Severity) -> usize {
        self.items.iter().filter(|d| d.severity == sev).count()
    }
    /// Total count.
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    /// Next diagnostic after line in file, wrapping.
    pub fn next_in_file(&self, file: &str, line: u32) -> Option<&Diagnostic> {
        let file_diags: Vec<_> = self.for_file(file);
        file_diags.iter().find(|d| d.location.line > line).copied()
            .or_else(|| file_diags.first().copied())
    }
    /// Previous diagnostic before line in file, wrapping.
    pub fn prev_in_file(&self, file: &str, line: u32) -> Option<&Diagnostic> {
        let file_diags: Vec<_> = self.for_file(file);
        file_diags.iter().rev().find(|d| d.location.line < line).copied()
            .or_else(|| file_diags.last().copied())
    }
    fn sort(&mut self) {
        self.items.sort_by(|a, b| {
            a.severity.cmp(&b.severity)
                .then_with(|| a.location.file.cmp(&b.location.file))
                .then_with(|| a.location.line.cmp(&b.location.line))
                .then_with(|| a.location.col.cmp(&b.location.col))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn diag(file: &str, line: u32, sev: Severity, src: &str) -> Diagnostic {
        Diagnostic {
            id: 0, kind: DiagnosticKind::Diagnostic, severity: sev,
            location: DiagnosticLocation {
                file: file.into(), line, col: 0, end_line: None, end_col: None,
            },
            message: "msg".into(), source: src.into(), code: None,
        }
    }
    #[test]
    fn replace_for_file_removes_old() {
        let mut s = DiagnosticStore::new();
        s.replace_for_file("a.rs", "lsp", vec![diag("a.rs", 1, Severity::Error, "lsp")]);
        assert_eq!(s.len(), 1);
        s.replace_for_file("a.rs", "lsp", vec![diag("a.rs", 5, Severity::Warning, "lsp")]);
        assert_eq!(s.len(), 1);
        assert_eq!(s.all()[0].location.line, 5);
    }
    #[test]
    fn replace_preserves_other_files() {
        let mut s = DiagnosticStore::new();
        s.replace_for_file("a.rs", "lsp", vec![diag("a.rs", 1, Severity::Error, "lsp")]);
        s.replace_for_file("b.rs", "lsp", vec![diag("b.rs", 2, Severity::Warning, "lsp")]);
        s.replace_for_file("a.rs", "lsp", vec![]);
        assert_eq!(s.len(), 1);
        assert_eq!(s.all()[0].location.file, "b.rs");
    }
    #[test]
    fn sort_by_severity_then_file_then_line() {
        let mut s = DiagnosticStore::new();
        s.append(vec![
            diag("b.rs", 10, Severity::Warning, "x"),
            diag("a.rs", 5, Severity::Error, "x"),
            diag("a.rs", 1, Severity::Hint, "x"),
        ]);
        assert_eq!(s.all()[0].severity, Severity::Error);
        assert_eq!(s.all()[1].severity, Severity::Warning);
        assert_eq!(s.all()[2].severity, Severity::Hint);
    }
    #[test]
    fn count_by_severity() {
        let mut s = DiagnosticStore::new();
        s.append(vec![
            diag("a.rs", 1, Severity::Error, "x"),
            diag("a.rs", 2, Severity::Error, "x"),
            diag("a.rs", 3, Severity::Warning, "x"),
        ]);
        assert_eq!(s.count_by_severity(Severity::Error), 2);
        assert_eq!(s.count_by_severity(Severity::Warning), 1);
        assert_eq!(s.count_by_severity(Severity::Hint), 0);
    }
    #[test]
    fn for_file_filters() {
        let mut s = DiagnosticStore::new();
        s.append(vec![diag("a.rs", 1, Severity::Error, "x"), diag("b.rs", 2, Severity::Error, "x")]);
        assert_eq!(s.for_file("a.rs").len(), 1);
        assert_eq!(s.for_file("c.rs").len(), 0);
    }
    #[test]
    fn next_in_file_wraps() {
        let mut s = DiagnosticStore::new();
        s.append(vec![diag("a.rs", 5, Severity::Error, "x"), diag("a.rs", 10, Severity::Error, "x")]);
        let n = s.next_in_file("a.rs", 7).unwrap();
        assert_eq!(n.location.line, 10);
        let n = s.next_in_file("a.rs", 10).unwrap();
        assert_eq!(n.location.line, 5); // wraps
    }
    #[test]
    fn prev_in_file_wraps() {
        let mut s = DiagnosticStore::new();
        s.append(vec![diag("a.rs", 5, Severity::Error, "x"), diag("a.rs", 10, Severity::Error, "x")]);
        let p = s.prev_in_file("a.rs", 7).unwrap();
        assert_eq!(p.location.line, 5);
        let p = s.prev_in_file("a.rs", 5).unwrap();
        assert_eq!(p.location.line, 10); // wraps
    }
    #[test]
    fn alloc_id_increments() {
        let mut s = DiagnosticStore::new();
        assert_eq!(s.alloc_id(), 0);
        assert_eq!(s.alloc_id(), 1);
        assert_eq!(s.alloc_id(), 2);
    }
}
