//! LSP client service for kjxlkj editor.
//!\n//! Provides Language Server Protocol client functionality.

use kjxlkj_services::{Service, ServiceMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// LSP position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspPosition {
    /// Line number (0-indexed).
    pub line: u32,
    /// Character offset (0-indexed).
    pub character: u32,
}

/// LSP range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspRange {
    /// Start position.
    pub start: LspPosition,
    /// End position.
    pub end: LspPosition,
}

/// LSP diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// LSP diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Range of the diagnostic.
    pub range: LspRange,
    /// Severity.
    pub severity: Option<DiagnosticSeverity>,
    /// Message.
    pub message: String,
    /// Source (e.g., "rust-analyzer").
    pub source: Option<String>,
}

/// Completion item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    /// Label.
    pub label: String,
    /// Kind.
    pub kind: Option<u32>,
    /// Detail.
    pub detail: Option<String>,
    /// Insert text.
    pub insert_text: Option<String>,
}

/// LSP client capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    /// Text document capabilities.
    pub text_document: Option<TextDocumentClientCapabilities>,
}

/// Text document client capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    /// Completion capabilities.
    pub completion: Option<CompletionClientCapabilities>,
    /// Synchronization capabilities.
    pub synchronization: Option<SyncCapabilities>,
}

/// Completion client capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
    /// Dynamic registration supported.
    pub dynamic_registration: Option<bool>,
    /// Snippet support.
    pub snippet_support: Option<bool>,
}

/// Sync capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncCapabilities {
    /// Dynamic registration.
    pub dynamic_registration: Option<bool>,
    /// Will save support.
    pub will_save: Option<bool>,
    /// Did save support.
    pub did_save: Option<bool>,
}

/// Initialize params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    /// Process ID.
    pub process_id: Option<u32>,
    /// Root URI.
    pub root_uri: Option<String>,
    /// Client capabilities.
    pub capabilities: ClientCapabilities,
    /// Client info.
    pub client_info: Option<ClientInfo>,
}

/// Client info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client name.
    pub name: String,
    /// Client version.
    pub version: Option<String>,
}

impl InitializeParams {
    /// Create new initialize params.
    pub fn new(root_uri: Option<String>) -> Self {
        Self {
            process_id: Some(std::process::id()),
            root_uri,
            capabilities: ClientCapabilities::default(),
            client_info: Some(ClientInfo {
                name: "kjxlkj".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        }
    }
}

/// Server capabilities (initialize result).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// Text document sync kind.
    pub text_document_sync: Option<TextDocumentSyncKind>,
    /// Completion provider.
    pub completion_provider: Option<bool>,
    /// Definition provider.
    pub definition_provider: Option<bool>,
    /// Hover provider.
    pub hover_provider: Option<bool>,
}

/// Text document sync kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TextDocumentSyncKind {
    /// No sync.
    None = 0,
    /// Full sync.
    Full = 1,
    /// Incremental sync.
    Incremental = 2,
}

/// Text document item for didOpen.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    /// Document URI.
    pub uri: String,
    /// Language ID.
    pub language_id: String,
    /// Version.
    pub version: i32,
    /// Text content.
    pub text: String,
}

impl TextDocumentItem {
    /// Create a new text document item.
    pub fn new(uri: String, language_id: String, text: String) -> Self {
        Self {
            uri,
            language_id,
            version: 1,
            text,
        }
    }
}

/// Text document identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    /// Document URI.
    pub uri: String,
}

/// Versioned text document identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    /// Document URI.
    pub uri: String,
    /// Version.
    pub version: i32,
}

/// Content change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
    /// Range of change (None for full sync).
    pub range: Option<LspRange>,
    /// New text.
    pub text: String,
}

/// didOpen params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
    /// Document being opened.
    pub text_document: TextDocumentItem,
}

/// didChange params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
    /// Document being changed.
    pub text_document: VersionedTextDocumentIdentifier,
    /// Content changes.
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// didClose params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseTextDocumentParams {
    /// Document being closed.
    pub text_document: TextDocumentIdentifier,
}

// ============================================================================
// Hover
// ============================================================================

/// Hover result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    /// Contents (markdown or plain text).
    pub contents: HoverContents,
    /// Range to highlight.
    pub range: Option<LspRange>,
}

/// Hover contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    /// Plain string.
    String(String),
    /// Markup content.
    Markup(MarkupContent),
    /// Multiple parts.
    Parts(Vec<MarkedString>),
}

impl HoverContents {
    /// Get as plain text.
    pub fn as_text(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Markup(m) => m.value.clone(),
            Self::Parts(parts) => parts.iter().map(|p| p.value()).collect::<Vec<_>>().join("\n\n"),
        }
    }
}

/// Marked string (language-tagged code or plain).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkedString {
    /// Plain string.
    String(String),
    /// Language-tagged code.
    Code { language: String, value: String },
}

impl MarkedString {
    /// Get value.
    pub fn value(&self) -> &str {
        match self {
            Self::String(s) => s,
            Self::Code { value, .. } => value,
        }
    }
}

/// Markup content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent {
    /// Kind (plaintext or markdown).
    pub kind: MarkupKind,
    /// The content.
    pub value: String,
}

/// Markup kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    /// Plain text.
    Plaintext,
    /// Markdown.
    Markdown,
}

// ============================================================================
// Signature Help
// ============================================================================

/// Signature help result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
    /// Available signatures.
    pub signatures: Vec<SignatureInformation>,
    /// Active signature index.
    pub active_signature: Option<u32>,
    /// Active parameter index.
    pub active_parameter: Option<u32>,
}

/// A single signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformation {
    /// Label (the signature).
    pub label: String,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
    /// Parameters.
    pub parameters: Option<Vec<ParameterInformation>>,
    /// Active parameter (overrides SignatureHelp.active_parameter).
    pub active_parameter: Option<u32>,
}

/// Parameter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    /// Label (parameter name or [start, end] offsets).
    pub label: ParameterLabel,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
}

/// Parameter label.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterLabel {
    /// Simple string label.
    String(String),
    /// Offset range [start, end].
    Offsets([u32; 2]),
}

// ============================================================================
// Code Actions
// ============================================================================

/// Code action kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionKind(pub String);

impl CodeActionKind {
    /// Quick fix.
    pub const QUICKFIX: &'static str = "quickfix";
    /// Refactor.
    pub const REFACTOR: &'static str = "refactor";
    /// Refactor extract.
    pub const REFACTOR_EXTRACT: &'static str = "refactor.extract";
    /// Refactor inline.
    pub const REFACTOR_INLINE: &'static str = "refactor.inline";
    /// Refactor rewrite.
    pub const REFACTOR_REWRITE: &'static str = "refactor.rewrite";
    /// Source action.
    pub const SOURCE: &'static str = "source";
    /// Organize imports.
    pub const SOURCE_ORGANIZE_IMPORTS: &'static str = "source.organizeImports";

    /// Create from string.
    pub fn new(kind: impl Into<String>) -> Self {
        Self(kind.into())
    }

    /// Is this a quickfix?
    pub fn is_quickfix(&self) -> bool {
        self.0.starts_with(Self::QUICKFIX)
    }

    /// Is this a refactor?
    pub fn is_refactor(&self) -> bool {
        self.0.starts_with(Self::REFACTOR)
    }
}

/// A code action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    /// Title (display text).
    pub title: String,
    /// Kind.
    pub kind: Option<CodeActionKind>,
    /// Diagnostics this action resolves.
    pub diagnostics: Option<Vec<Diagnostic>>,
    /// Is preferred action.
    pub is_preferred: Option<bool>,
    /// Workspace edit to apply.
    pub edit: Option<WorkspaceEdit>,
    /// Command to execute.
    pub command: Option<Command>,
}

/// A command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Title.
    pub title: String,
    /// Command ID.
    pub command: String,
    /// Arguments.
    pub arguments: Option<Vec<serde_json::Value>>,
}

/// Workspace edit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEdit {
    /// Document changes keyed by URI.
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
    /// Versioned document changes.
    pub document_changes: Option<Vec<TextDocumentEdit>>,
}

/// Text edit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    /// Range to replace.
    pub range: LspRange,
    /// New text.
    pub new_text: String,
}

/// Text document edit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
    /// Document.
    pub text_document: VersionedTextDocumentIdentifier,
    /// Edits.
    pub edits: Vec<TextEdit>,
}

// ============================================================================
// Navigation (Go to Definition, References, etc.)
// ============================================================================

/// Location result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Document URI.
    pub uri: String,
    /// Range.
    pub range: LspRange,
}

/// Location link (with origin span).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
    /// Origin selection range (the clicked range).
    pub origin_selection_range: Option<LspRange>,
    /// Target URI.
    pub target_uri: String,
    /// Target range (full definition).
    pub target_range: LspRange,
    /// Target selection range (symbol name).
    pub target_selection_range: LspRange,
}

/// Definition response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionResponse {
    /// Single location.
    Single(Location),
    /// Multiple locations.
    Multiple(Vec<Location>),
    /// Location links.
    Links(Vec<LocationLink>),
}

// ============================================================================
// Rename
// ============================================================================

/// Prepare rename result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareRenameResponse {
    /// Range to rename.
    Range(LspRange),
    /// Range with placeholder.
    RangeWithPlaceholder { range: LspRange, placeholder: String },
    /// Default behavior.
    DefaultBehavior { default_behavior: bool },
}

/// Rename params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameParams {
    /// Document.
    pub text_document: TextDocumentIdentifier,
    /// Position.
    pub position: LspPosition,
    /// New name.
    pub new_name: String,
}

// ============================================================================
// Code Lens
// ============================================================================

/// Code lens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLens {
    /// Range.
    pub range: LspRange,
    /// Command (may be unresolved).
    pub command: Option<Command>,
    /// Data for resolve.
    pub data: Option<serde_json::Value>,
}

// ============================================================================
// Formatting
// ============================================================================

/// Formatting options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
    /// Tab size.
    pub tab_size: u32,
    /// Insert spaces instead of tabs.
    pub insert_spaces: bool,
    /// Trim trailing whitespace.
    pub trim_trailing_whitespace: Option<bool>,
    /// Insert final newline.
    pub insert_final_newline: Option<bool>,
    /// Trim final newlines.
    pub trim_final_newlines: Option<bool>,
}

impl FormattingOptions {
    /// Create with tab size and spaces.
    pub fn new(tab_size: u32, insert_spaces: bool) -> Self {
        Self {
            tab_size,
            insert_spaces,
            ..Default::default()
        }
    }
}

// ============================================================================
// Symbols
// ============================================================================

/// Symbol kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/// Document symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    /// Name.
    pub name: String,
    /// Detail.
    pub detail: Option<String>,
    /// Kind.
    pub kind: SymbolKind,
    /// Range.
    pub range: LspRange,
    /// Selection range.
    pub selection_range: LspRange,
    /// Children.
    pub children: Option<Vec<DocumentSymbol>>,
}

/// Symbol information (flat).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformation {
    /// Name.
    pub name: String,
    /// Kind.
    pub kind: SymbolKind,
    /// Location.
    pub location: Location,
    /// Container name.
    pub container_name: Option<String>,
}

// ============================================================================
// Completion (extended)
// ============================================================================

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

impl CompletionItemKind {
    /// Get icon character.
    pub fn icon(&self) -> char {
        match self {
            Self::Function | Self::Method => 'ƒ',
            Self::Struct | Self::Class => '□',
            Self::Enum | Self::EnumMember => '◇',
            Self::Variable | Self::Field => '∴',
            Self::Module => '◫',
            Self::Keyword => '⌘',
            Self::Snippet => '✎',
            Self::File | Self::Folder => '📄',
            Self::Constant => 'π',
            Self::Interface => '◌',
            Self::Property => '◉',
            _ => '○',
        }
    }
}

/// Extended completion item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemEx {
    /// Label.
    pub label: String,
    /// Kind.
    pub kind: Option<CompletionItemKind>,
    /// Detail.
    pub detail: Option<String>,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
    /// Sort text.
    pub sort_text: Option<String>,
    /// Filter text.
    pub filter_text: Option<String>,
    /// Insert text.
    pub insert_text: Option<String>,
    /// Insert text format.
    pub insert_text_format: Option<InsertTextFormat>,
    /// Text edit.
    pub text_edit: Option<TextEdit>,
    /// Additional text edits.
    pub additional_text_edits: Option<Vec<TextEdit>>,
    /// Preselect.
    pub preselect: Option<bool>,
}

/// Insert text format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum InsertTextFormat {
    /// Plain text.
    PlainText = 1,
    /// Snippet.
    Snippet = 2,
}

/// Completion list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
    /// Is incomplete (more items on continued typing).
    pub is_incomplete: bool,
    /// Items.
    pub items: Vec<CompletionItemEx>,
}

/// LSP server configuration.
#[derive(Debug, Clone)]
pub struct LspServerConfig {
    /// Language ID.
    pub language_id: String,
    /// Server command.
    pub command: String,
    /// Server arguments.
    pub args: Vec<String>,
    /// Root patterns for workspace detection.
    pub root_patterns: Vec<String>,
}

impl LspServerConfig {
    /// Create Rust Analyzer config.
    pub fn rust_analyzer() -> Self {
        Self {
            language_id: "rust".to_string(),
            command: "rust-analyzer".to_string(),
            args: Vec::new(),
            root_patterns: vec!["Cargo.toml".to_string()],
        }
    }

    /// Create TypeScript config.
    pub fn typescript() -> Self {
        Self {
            language_id: "typescript".to_string(),
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            root_patterns: vec!["package.json".to_string(), "tsconfig.json".to_string()],
        }
    }
}

/// LSP service.
pub struct LspService {
    /// Service name.
    name: String,
    /// Server configs.
    configs: HashMap<String, LspServerConfig>,
}

impl LspService {
    /// Create a new LSP service.
    pub fn new() -> Self {
        Self {
            name: "lsp".to_string(),
            configs: HashMap::new(),
        }
    }

    /// Add a server configuration.
    pub fn add_config(&mut self, config: LspServerConfig) {
        self.configs.insert(config.language_id.clone(), config);
    }

    /// Get server config for language.
    pub fn get_config(&self, language_id: &str) -> Option<&LspServerConfig> {
        self.configs.get(language_id)
    }
}

impl Default for LspService {
    fn default() -> Self {
        let mut service = Self::new();
        service.add_config(LspServerConfig::rust_analyzer());
        service.add_config(LspServerConfig::typescript());
        service
    }
}

impl Service for LspService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("LSP service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("LSP service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_service_new() {
        let service = LspService::new();
        assert_eq!(service.name(), "lsp");
    }

    #[test]
    fn test_lsp_config() {
        let mut service = LspService::new();
        service.add_config(LspServerConfig::rust_analyzer());
        assert!(service.get_config("rust").is_some());
        assert!(service.get_config("python").is_none());
    }

    #[test]
    fn test_diagnostic_severity() {
        let severity = DiagnosticSeverity::Error;
        assert_eq!(severity, DiagnosticSeverity::Error);
    }

    #[test]
    fn test_completion_item() {
        let item = CompletionItem {
            label: "test".to_string(),
            kind: Some(1),
            detail: Some("A test item".to_string()),
            insert_text: None,
        };
        assert_eq!(item.label, "test");
    }

    #[test]
    fn test_lsp_position() {
        let pos = LspPosition { line: 10, character: 5 };
        assert_eq!(pos.line, 10);
        assert_eq!(pos.character, 5);
    }

    #[test]
    fn test_lsp_range() {
        let range = LspRange {
            start: LspPosition { line: 0, character: 0 },
            end: LspPosition { line: 1, character: 10 },
        };
        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 1);
    }

    #[test]
    fn test_diagnostic() {
        let diag = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 5, character: 0 },
                end: LspPosition { line: 5, character: 10 },
            },
            severity: Some(DiagnosticSeverity::Warning),
            message: "test warning".to_string(),
            source: Some("test".to_string()),
        };
        assert_eq!(diag.message, "test warning");
        assert_eq!(diag.severity, Some(DiagnosticSeverity::Warning));
    }

    #[test]
    fn test_lsp_server_config_rust() {
        let config = LspServerConfig::rust_analyzer();
        assert_eq!(config.language_id, "rust");
        assert_eq!(config.command, "rust-analyzer");
    }

    #[test]
    fn test_lsp_server_config_typescript() {
        let config = LspServerConfig::typescript();
        assert_eq!(config.language_id, "typescript");
    }

    #[test]
    fn test_lsp_service_default() {
        let service = LspService::default();
        assert_eq!(service.name(), "lsp");
    }

    #[test]
    fn test_diagnostic_severity_variants() {
        assert_ne!(DiagnosticSeverity::Error, DiagnosticSeverity::Warning);
        assert_ne!(DiagnosticSeverity::Information, DiagnosticSeverity::Hint);
    }

    #[test]
    fn test_lsp_service_default_has_configs() {
        let service = LspService::default();
        // Default should have rust and typescript
        assert!(service.get_config("rust").is_some());
        assert!(service.get_config("typescript").is_some());
    }

    #[test]
    fn test_lsp_position_clone() {
        let pos = LspPosition { line: 10, character: 5 };
        let cloned = pos.clone();
        assert_eq!(pos.line, cloned.line);
        assert_eq!(pos.character, cloned.character);
    }

    #[test]
    fn test_lsp_range_clone() {
        let range = LspRange {
            start: LspPosition { line: 0, character: 0 },
            end: LspPosition { line: 1, character: 10 },
        };
        let cloned = range.clone();
        assert_eq!(range.start.line, cloned.start.line);
        assert_eq!(range.end.character, cloned.end.character);
    }

    #[test]
    fn test_completion_item_clone() {
        let item = CompletionItem {
            label: "test".to_string(),
            kind: Some(1),
            detail: Some("A test".to_string()),
            insert_text: Some("test()".to_string()),
        };
        let cloned = item.clone();
        assert_eq!(item.label, cloned.label);
        assert_eq!(item.insert_text, cloned.insert_text);
    }

    #[test]
    fn test_diagnostic_clone() {
        let diag = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 10 },
            },
            severity: Some(DiagnosticSeverity::Error),
            message: "error".to_string(),
            source: None,
        };
        let cloned = diag.clone();
        assert_eq!(diag.message, cloned.message);
    }

    #[test]
    fn test_lsp_server_config_python() {
        // Test custom config creation
        let config = LspServerConfig {
            language_id: "python".to_string(),
            command: "pyright".to_string(),
            args: vec!["--stdio".to_string()],
            root_patterns: vec!["pyproject.toml".to_string()],
        };
        assert_eq!(config.language_id, "python");
    }

    #[test]
    fn test_diagnostic_severity_clone() {
        let sev = DiagnosticSeverity::Warning;
        let cloned = sev;
        assert_eq!(sev, cloned);
    }

    #[test]
    fn test_diagnostic_severity_copy() {
        let sev = DiagnosticSeverity::Hint;
        let copied = sev; // Copy
        assert_eq!(sev, copied);
    }

    #[test]
    fn test_lsp_position_clone_values() {
        let pos = LspPosition { line: 10, character: 20 };
        let cloned = pos.clone();
        assert_eq!(cloned.line, 10);
        assert_eq!(cloned.character, 20);
    }

    #[test]
    fn test_lsp_range_clone_values() {
        let range = LspRange {
            start: LspPosition { line: 0, character: 0 },
            end: LspPosition { line: 1, character: 10 },
        };
        let cloned = range.clone();
        assert_eq!(cloned.start.line, 0);
        assert_eq!(cloned.end.line, 1);
    }

    #[test]
    fn test_completion_item_clone_values() {
        let item = CompletionItem {
            label: "test".to_string(),
            kind: Some(1),
            detail: Some("detail".to_string()),
            insert_text: None,
        };
        let cloned = item.clone();
        assert_eq!(cloned.label, "test");
        assert_eq!(cloned.kind, Some(1));
    }

    #[test]
    fn test_diagnostic_clone_values() {
        let diag = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 10 },
            },
            severity: Some(DiagnosticSeverity::Error),
            message: "test error".to_string(),
            source: Some("test".to_string()),
        };
        let cloned = diag.clone();
        assert_eq!(cloned.message, "test error");
    }

    #[test]
    fn test_diagnostic_severity_all_values() {
        assert_eq!(DiagnosticSeverity::Error as i32, 1);
        assert_eq!(DiagnosticSeverity::Warning as i32, 2);
        assert_eq!(DiagnosticSeverity::Information as i32, 3);
        assert_eq!(DiagnosticSeverity::Hint as i32, 4);
    }

    #[test]
    fn test_lsp_position_debug() {
        let pos = LspPosition { line: 5, character: 10 };
        let debug = format!("{:?}", pos);
        assert!(debug.contains("line"));
        assert!(debug.contains("5"));
    }

    #[test]
    fn test_lsp_range_debug() {
        let range = LspRange {
            start: LspPosition { line: 0, character: 0 },
            end: LspPosition { line: 1, character: 5 },
        };
        let debug = format!("{:?}", range);
        assert!(debug.contains("start"));
        assert!(debug.contains("end"));
    }

    #[test]
    fn test_completion_item_debug() {
        let item = CompletionItem {
            label: "test".to_string(),
            kind: None,
            detail: None,
            insert_text: None,
        };
        let debug = format!("{:?}", item);
        assert!(debug.contains("test"));
    }

    #[test]
    fn test_diagnostic_debug() {
        let diag = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 5 },
            },
            severity: Some(DiagnosticSeverity::Warning),
            message: "warning message".to_string(),
            source: None,
        };
        let debug = format!("{:?}", diag);
        assert!(debug.contains("warning message"));
    }

    #[test]
    fn test_lsp_server_config_debug() {
        let config = LspServerConfig::rust_analyzer();
        let debug = format!("{:?}", config);
        assert!(debug.contains("rust"));
    }

    #[test]
    fn test_diagnostic_severity_debug() {
        let sev = DiagnosticSeverity::Information;
        let debug = format!("{:?}", sev);
        assert!(debug.contains("Information"));
    }

    #[test]
    fn test_completion_item_with_insert_text() {
        let item = CompletionItem {
            label: "println".to_string(),
            kind: Some(3),
            detail: Some("macro".to_string()),
            insert_text: Some("println!($0)".to_string()),
        };
        assert_eq!(item.insert_text, Some("println!($0)".to_string()));
    }

    #[test]
    fn test_diagnostic_with_source() {
        let diag = Diagnostic {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 1 },
            },
            severity: Some(DiagnosticSeverity::Hint),
            message: "hint".to_string(),
            source: Some("clippy".to_string()),
        };
        assert_eq!(diag.source, Some("clippy".to_string()));
    }

    #[test]
    fn test_lsp_position_zero() {
        let pos = LspPosition { line: 0, character: 0 };
        assert_eq!(pos.line, 0);
        assert_eq!(pos.character, 0);
    }

    #[test]
    fn test_lsp_position_max() {
        let pos = LspPosition { line: u32::MAX, character: u32::MAX };
        assert_eq!(pos.line, u32::MAX);
        assert_eq!(pos.character, u32::MAX);
    }

    #[test]
    fn test_initialize_params_new() {
        let params = InitializeParams::new(Some("file:///project".to_string()));
        assert!(params.process_id.is_some());
        assert_eq!(params.root_uri, Some("file:///project".to_string()));
        assert!(params.client_info.is_some());
    }

    #[test]
    fn test_text_document_item_new() {
        let item = TextDocumentItem::new(
            "file:///test.rs".to_string(),
            "rust".to_string(),
            "fn main() {}".to_string(),
        );
        assert_eq!(item.version, 1);
        assert_eq!(item.language_id, "rust");
    }

    #[test]
    fn test_did_open_params() {
        let item = TextDocumentItem::new(
            "file:///test.rs".to_string(),
            "rust".to_string(),
            "fn main() {}".to_string(),
        );
        let params = DidOpenTextDocumentParams { text_document: item };
        assert_eq!(params.text_document.uri, "file:///test.rs");
    }

    #[test]
    fn test_did_change_params() {
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: "file:///test.rs".to_string(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                text: "fn main() { println!(\"hello\"); }".to_string(),
            }],
        };
        assert_eq!(params.text_document.version, 2);
        assert_eq!(params.content_changes.len(), 1);
    }

    #[test]
    fn test_did_close_params() {
        let params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: "file:///test.rs".to_string(),
            },
        };
        assert_eq!(params.text_document.uri, "file:///test.rs");
    }

    #[test]
    fn test_text_document_sync_kind() {
        assert_eq!(TextDocumentSyncKind::None as u8, 0);
        assert_eq!(TextDocumentSyncKind::Full as u8, 1);
        assert_eq!(TextDocumentSyncKind::Incremental as u8, 2);
    }

    #[test]
    fn test_server_capabilities_default() {
        let caps = ServerCapabilities::default();
        assert!(caps.text_document_sync.is_none());
        assert!(caps.completion_provider.is_none());
    }

    #[test]
    fn test_client_capabilities_default() {
        let caps = ClientCapabilities::default();
        assert!(caps.text_document.is_none());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Hover Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_hover_contents_string() {
        let contents = HoverContents::String("Hello".to_string());
        assert_eq!(contents.as_text(), "Hello");
    }

    #[test]
    fn test_hover_contents_markup() {
        let contents = HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "# Title".to_string(),
        });
        assert_eq!(contents.as_text(), "# Title");
    }

    #[test]
    fn test_marked_string_value() {
        let plain = MarkedString::String("plain".to_string());
        assert_eq!(plain.value(), "plain");

        let code = MarkedString::Code {
            language: "rust".to_string(),
            value: "fn foo()".to_string(),
        };
        assert_eq!(code.value(), "fn foo()");
    }

    #[test]
    fn test_markup_kind() {
        let plain = MarkupKind::Plaintext;
        let md = MarkupKind::Markdown;
        assert_ne!(plain, md);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Signature Help Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_signature_help() {
        let help = SignatureHelp {
            signatures: vec![],
            active_signature: Some(0),
            active_parameter: Some(1),
        };
        assert!(help.signatures.is_empty());
        assert_eq!(help.active_parameter, Some(1));
    }

    #[test]
    fn test_signature_information() {
        let sig = SignatureInformation {
            label: "fn foo(a: i32, b: i32)".to_string(),
            documentation: None,
            parameters: Some(vec![]),
            active_parameter: None,
        };
        assert!(sig.label.contains("foo"));
    }

    #[test]
    fn test_parameter_label() {
        let string_label = ParameterLabel::String("param".to_string());
        let offset_label = ParameterLabel::Offsets([10, 15]);

        match string_label {
            ParameterLabel::String(s) => assert_eq!(s, "param"),
            _ => panic!("Expected string"),
        }

        match offset_label {
            ParameterLabel::Offsets([start, end]) => {
                assert_eq!(start, 10);
                assert_eq!(end, 15);
            }
            _ => panic!("Expected offsets"),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Code Action Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_code_action_kind_constants() {
        assert_eq!(CodeActionKind::QUICKFIX, "quickfix");
        assert_eq!(CodeActionKind::REFACTOR, "refactor");
        assert_eq!(CodeActionKind::SOURCE, "source");
    }

    #[test]
    fn test_code_action_kind_is_quickfix() {
        let kind = CodeActionKind::new("quickfix.import");
        assert!(kind.is_quickfix());
        assert!(!kind.is_refactor());
    }

    #[test]
    fn test_code_action_kind_is_refactor() {
        let kind = CodeActionKind::new("refactor.extract");
        assert!(kind.is_refactor());
        assert!(!kind.is_quickfix());
    }

    #[test]
    fn test_code_action() {
        let action = CodeAction {
            title: "Import foo".to_string(),
            kind: Some(CodeActionKind::new("quickfix.import")),
            diagnostics: None,
            is_preferred: Some(true),
            edit: None,
            command: None,
        };
        assert_eq!(action.title, "Import foo");
        assert!(action.is_preferred.unwrap());
    }

    #[test]
    fn test_text_edit() {
        let edit = TextEdit {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 5 },
            },
            new_text: "hello".to_string(),
        };
        assert_eq!(edit.new_text, "hello");
    }

    #[test]
    fn test_workspace_edit_default() {
        let edit = WorkspaceEdit::default();
        assert!(edit.changes.is_none());
        assert!(edit.document_changes.is_none());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Navigation Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_location() {
        let loc = Location {
            uri: "file:///test.rs".to_string(),
            range: LspRange {
                start: LspPosition { line: 10, character: 0 },
                end: LspPosition { line: 10, character: 20 },
            },
        };
        assert_eq!(loc.uri, "file:///test.rs");
        assert_eq!(loc.range.start.line, 10);
    }

    #[test]
    fn test_location_link() {
        let link = LocationLink {
            origin_selection_range: None,
            target_uri: "file:///def.rs".to_string(),
            target_range: LspRange {
                start: LspPosition { line: 5, character: 0 },
                end: LspPosition { line: 10, character: 0 },
            },
            target_selection_range: LspRange {
                start: LspPosition { line: 5, character: 4 },
                end: LspPosition { line: 5, character: 10 },
            },
        };
        assert_eq!(link.target_uri, "file:///def.rs");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Rename Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_rename_params() {
        let params = RenameParams {
            text_document: TextDocumentIdentifier {
                uri: "file:///test.rs".to_string(),
            },
            position: LspPosition { line: 5, character: 10 },
            new_name: "new_name".to_string(),
        };
        assert_eq!(params.new_name, "new_name");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Code Lens Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_code_lens() {
        let lens = CodeLens {
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 0, character: 10 },
            },
            command: Some(Command {
                title: "Run test".to_string(),
                command: "rust-analyzer.runSingle".to_string(),
                arguments: None,
            }),
            data: None,
        };
        assert!(lens.command.is_some());
        assert_eq!(lens.command.unwrap().title, "Run test");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Formatting Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_formatting_options_new() {
        let opts = FormattingOptions::new(4, true);
        assert_eq!(opts.tab_size, 4);
        assert!(opts.insert_spaces);
    }

    #[test]
    fn test_formatting_options_default() {
        let opts = FormattingOptions::default();
        assert_eq!(opts.tab_size, 0);
        assert!(!opts.insert_spaces);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Symbol Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_symbol_kind_values() {
        assert_eq!(SymbolKind::File as u8, 1);
        assert_eq!(SymbolKind::Function as u8, 12);
        assert_eq!(SymbolKind::Struct as u8, 23);
    }

    #[test]
    fn test_document_symbol() {
        let sym = DocumentSymbol {
            name: "main".to_string(),
            detail: Some("fn main()".to_string()),
            kind: SymbolKind::Function,
            range: LspRange {
                start: LspPosition { line: 0, character: 0 },
                end: LspPosition { line: 10, character: 0 },
            },
            selection_range: LspRange {
                start: LspPosition { line: 0, character: 3 },
                end: LspPosition { line: 0, character: 7 },
            },
            children: None,
        };
        assert_eq!(sym.name, "main");
        assert_eq!(sym.kind, SymbolKind::Function);
    }

    #[test]
    fn test_symbol_information() {
        let sym = SymbolInformation {
            name: "MyStruct".to_string(),
            kind: SymbolKind::Struct,
            location: Location {
                uri: "file:///test.rs".to_string(),
                range: LspRange {
                    start: LspPosition { line: 5, character: 0 },
                    end: LspPosition { line: 15, character: 0 },
                },
            },
            container_name: Some("module".to_string()),
        };
        assert_eq!(sym.name, "MyStruct");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Completion Extended Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_completion_item_kind_icon() {
        assert_eq!(CompletionItemKind::Function.icon(), 'ƒ');
        assert_eq!(CompletionItemKind::Struct.icon(), '□');
        assert_eq!(CompletionItemKind::Enum.icon(), '◇');
        assert_eq!(CompletionItemKind::Variable.icon(), '∴');
        assert_eq!(CompletionItemKind::Snippet.icon(), '✎');
    }

    #[test]
    fn test_completion_item_kind_values() {
        assert_eq!(CompletionItemKind::Text as u8, 1);
        assert_eq!(CompletionItemKind::Method as u8, 2);
        assert_eq!(CompletionItemKind::Snippet as u8, 15);
    }

    #[test]
    fn test_insert_text_format() {
        assert_eq!(InsertTextFormat::PlainText as u8, 1);
        assert_eq!(InsertTextFormat::Snippet as u8, 2);
    }

    #[test]
    fn test_completion_list() {
        let list = CompletionList {
            is_incomplete: true,
            items: vec![],
        };
        assert!(list.is_incomplete);
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_completion_item_ex() {
        let item = CompletionItemEx {
            label: "println".to_string(),
            kind: Some(CompletionItemKind::Function),
            detail: Some("macro".to_string()),
            documentation: None,
            sort_text: None,
            filter_text: None,
            insert_text: Some("println!($0)".to_string()),
            insert_text_format: Some(InsertTextFormat::Snippet),
            text_edit: None,
            additional_text_edits: None,
            preselect: Some(true),
        };
        assert_eq!(item.label, "println");
        assert!(item.preselect.unwrap());
    }
}
