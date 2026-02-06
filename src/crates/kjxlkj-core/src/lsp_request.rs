/// LSP request/response modeling — request lifecycle, capabilities, diagnostics.

use std::collections::HashMap;

/// LSP request identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestId { Num(i64), Str(String) }

/// LSP request method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspMethod {
    Initialize, Shutdown, TextDocOpen, TextDocClose, TextDocChange,
    Completion, Hover, Definition, References, Rename,
    CodeAction, Formatting, SignatureHelp, DocumentSymbol, WorkspaceSymbol,
}

impl LspMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Initialize => "initialize", Self::Shutdown => "shutdown",
            Self::TextDocOpen => "textDocument/didOpen", Self::TextDocClose => "textDocument/didClose",
            Self::TextDocChange => "textDocument/didChange", Self::Completion => "textDocument/completion",
            Self::Hover => "textDocument/hover", Self::Definition => "textDocument/definition",
            Self::References => "textDocument/references", Self::Rename => "textDocument/rename",
            Self::CodeAction => "textDocument/codeAction", Self::Formatting => "textDocument/formatting",
            Self::SignatureHelp => "textDocument/signatureHelp",
            Self::DocumentSymbol => "textDocument/documentSymbol",
            Self::WorkspaceSymbol => "workspace/symbol",
        }
    }
}

/// Pending request tracker.
#[derive(Debug)]
pub struct PendingRequests { requests: HashMap<RequestId, LspMethod>, next_id: i64 }

impl PendingRequests {
    pub fn new() -> Self { Self { requests: HashMap::new(), next_id: 1 } }

    pub fn send(&mut self, method: LspMethod) -> RequestId {
        let id = RequestId::Num(self.next_id);
        self.next_id += 1;
        self.requests.insert(id.clone(), method);
        id
    }

    pub fn complete(&mut self, id: &RequestId) -> Option<LspMethod> {
        self.requests.remove(id)
    }

    pub fn pending_count(&self) -> usize { self.requests.len() }
    pub fn is_pending(&self, id: &RequestId) -> bool { self.requests.contains_key(id) }
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity { Error = 1, Warning = 2, Information = 3, Hint = 4 }

/// A single diagnostic.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub line: usize, pub col: usize, pub end_line: usize, pub end_col: usize,
    pub severity: DiagnosticSeverity, pub message: String, pub source: Option<String>,
}

/// Diagnostics collection per file.
#[derive(Debug, Default)]
pub struct DiagnosticStore { entries: HashMap<String, Vec<Diagnostic>> }

impl DiagnosticStore {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, uri: &str, diags: Vec<Diagnostic>) {
        self.entries.insert(uri.into(), diags);
    }

    pub fn get(&self, uri: &str) -> &[Diagnostic] {
        self.entries.get(uri).map_or(&[], |v| v.as_slice())
    }

    pub fn clear(&mut self, uri: &str) { self.entries.remove(uri); }

    pub fn error_count(&self, uri: &str) -> usize {
        self.get(uri).iter().filter(|d| d.severity == DiagnosticSeverity::Error).count()
    }

    pub fn all_files(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
}

/// Server capability flags.
#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
    pub completion: bool, pub hover: bool, pub definition: bool,
    pub references: bool, pub rename: bool, pub code_action: bool,
    pub formatting: bool, pub signature_help: bool, pub document_symbol: bool,
}

impl ServerCapabilities {
    pub fn supports(&self, method: LspMethod) -> bool {
        match method {
            LspMethod::Completion => self.completion, LspMethod::Hover => self.hover,
            LspMethod::Definition => self.definition, LspMethod::References => self.references,
            LspMethod::Rename => self.rename, LspMethod::CodeAction => self.code_action,
            LspMethod::Formatting => self.formatting, LspMethod::SignatureHelp => self.signature_help,
            LspMethod::DocumentSymbol => self.document_symbol,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_send_complete() {
        let mut p = PendingRequests::new();
        let id = p.send(LspMethod::Completion);
        assert_eq!(p.pending_count(), 1);
        assert!(p.is_pending(&id));
        let method = p.complete(&id).unwrap();
        assert_eq!(method, LspMethod::Completion);
        assert_eq!(p.pending_count(), 0);
    }

    #[test]
    fn pending_unknown_id() {
        let mut p = PendingRequests::new();
        assert!(p.complete(&RequestId::Num(999)).is_none());
    }

    #[test]
    fn diagnostic_store_set_get() {
        let mut ds = DiagnosticStore::new();
        ds.set("file.rs", vec![Diagnostic { line: 1, col: 0, end_line: 1, end_col: 5,
            severity: DiagnosticSeverity::Error, message: "err".into(), source: None }]);
        assert_eq!(ds.error_count("file.rs"), 1);
        assert_eq!(ds.get("file.rs").len(), 1);
    }

    #[test]
    fn diagnostic_store_clear() {
        let mut ds = DiagnosticStore::new();
        ds.set("a.rs", vec![]);
        ds.clear("a.rs");
        assert!(ds.get("a.rs").is_empty());
    }

    #[test]
    fn capabilities_check() {
        let caps = ServerCapabilities { completion: true, hover: false, ..Default::default() };
        assert!(caps.supports(LspMethod::Completion));
        assert!(!caps.supports(LspMethod::Hover));
        assert!(caps.supports(LspMethod::Initialize));
    }

    #[test]
    fn method_str() {
        assert_eq!(LspMethod::Completion.as_str(), "textDocument/completion");
        assert_eq!(LspMethod::Initialize.as_str(), "initialize");
    }

    #[test]
    fn multiple_files_diagnostics() {
        let mut ds = DiagnosticStore::new();
        ds.set("a.rs", vec![]); ds.set("b.rs", vec![]);
        assert_eq!(ds.all_files().len(), 2);
    }

    #[test]
    fn sequential_ids() {
        let mut p = PendingRequests::new();
        let id1 = p.send(LspMethod::Hover);
        let id2 = p.send(LspMethod::Definition);
        assert_ne!(id1, id2);
        assert_eq!(p.pending_count(), 2);
    }
}
