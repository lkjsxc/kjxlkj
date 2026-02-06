//! LSP client service — language server protocol integration.

pub mod protocol;

use std::path::PathBuf;

/// Configuration for an LSP server.
#[derive(Debug, Clone)]
pub struct LspServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub root_dir: PathBuf,
}

/// Represents the state of an LSP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspConnectionState {
    Disconnected, Initializing, Ready, ShuttingDown,
}

/// Position in a text document (0-indexed line and character).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LspPosition { pub line: u32, pub character: u32 }

/// Range in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LspRange { pub start: LspPosition, pub end: LspPosition }

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity { Error, Warning, Information, Hint }

/// A diagnostic from a language server.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: LspRange,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<String>,
}

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Text, Method, Function, Constructor, Field, Variable,
    Class, Interface, Module, Property, Unit, Value, Enum,
    Keyword, Snippet, Color, File, Reference, Folder,
    EnumMember, Constant, Struct, Event, Operator, TypeParameter,
}

/// A completion item from a language server.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub sort_text: Option<String>,
}

/// Hover information returned by textDocument/hover.
#[derive(Debug, Clone)]
pub struct HoverInfo { pub contents: String, pub range: Option<LspRange> }

/// Text edit to apply to a document.
#[derive(Debug, Clone)]
pub struct TextEdit { pub range: LspRange, pub new_text: String }

/// A code action from a language server.
#[derive(Debug, Clone)]
pub struct CodeAction { pub title: String, pub kind: Option<String>, pub edits: Vec<TextEdit> }

/// A location in a file (for go-to-definition, references, etc.).
#[derive(Debug, Clone)]
pub struct Location { pub uri: String, pub range: LspRange }

/// Server capabilities received from initialize response.
#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
    pub hover: bool, pub completion: bool, pub definition: bool,
    pub references: bool, pub rename: bool, pub code_action: bool,
    pub formatting: bool, pub signature_help: bool,
    pub code_lens: bool, pub document_symbol: bool,
}

/// An LSP service instance managing communication with a language server.
pub struct LspService {
    config: Option<LspServerConfig>,
    state: LspConnectionState,
    pub capabilities: ServerCapabilities,
    pub diagnostics: Vec<Diagnostic>,
}

impl LspService {
    pub fn new() -> Self {
        Self { config: None, state: LspConnectionState::Disconnected,
            capabilities: ServerCapabilities::default(), diagnostics: Vec::new() }
    }
    pub fn configure(&mut self, config: LspServerConfig) { self.config = Some(config); }
    pub fn state(&self) -> LspConnectionState { self.state }
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow::anyhow!("not configured"))?;
        tracing::info!(server = %config.name, "starting LSP server");
        self.state = LspConnectionState::Initializing;
        self.state = LspConnectionState::Ready;
        Ok(())
    }
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.state = LspConnectionState::ShuttingDown;
        self.state = LspConnectionState::Disconnected;
        Ok(())
    }
}

impl Default for LspService { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsp_default_state() {
        let svc = LspService::new();
        assert_eq!(svc.state(), LspConnectionState::Disconnected);
        assert!(svc.diagnostics.is_empty());
    }

    #[test]
    fn lsp_configure() {
        let mut svc = LspService::new();
        svc.configure(LspServerConfig {
            name: "rust-analyzer".into(), command: "rust-analyzer".into(),
            args: vec![], root_dir: PathBuf::from("/tmp"),
        });
        assert_eq!(svc.state(), LspConnectionState::Disconnected);
    }

    #[test]
    fn diagnostic_type() {
        let d = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 5 },
            },
            severity: DiagnosticSeverity::Error,
            message: "err".into(), source: Some("test".into()), code: None,
        };
        assert_eq!(d.severity, DiagnosticSeverity::Error);
    }

    #[test]
    fn completion_item_kind() {
        let item = CompletionItem {
            label: "println!".into(), kind: Some(CompletionItemKind::Function),
            detail: None, documentation: None, insert_text: None, sort_text: None,
        };
        assert_eq!(item.kind, Some(CompletionItemKind::Function));
    }

    #[test]
    fn capabilities_default_false() {
        let caps = ServerCapabilities::default();
        assert!(!caps.hover && !caps.completion && !caps.definition);
    }

    #[test]
    fn code_action_edits() {
        let action = CodeAction {
            title: "Fix".into(), kind: Some("quickfix".into()),
            edits: vec![TextEdit {
                range: LspRange { start: LspPosition { line: 0, character: 0 },
                    end: LspPosition { line: 0, character: 0 } },
                new_text: "use std;\n".into(),
            }],
        };
        assert_eq!(action.edits.len(), 1);
    }

    #[test]
    fn hover_info_type() {
        let h = HoverInfo { contents: "fn main()".into(), range: None };
        assert!(h.contents.contains("main"));
    }

    #[test]
    fn location_type() {
        let loc = Location {
            uri: "file:///test.rs".into(),
            range: LspRange { start: LspPosition { line: 5, character: 3 },
                end: LspPosition { line: 5, character: 8 } },
        };
        assert!(loc.uri.contains("test.rs"));
    }
}
