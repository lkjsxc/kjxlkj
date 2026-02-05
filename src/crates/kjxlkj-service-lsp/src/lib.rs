//! LSP client service for kjxlkj editor.
//!
//! Provides Language Server Protocol client functionality.

use kjxlkj_services::{Service, ServiceMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

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
        let cloned = sev.clone();
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
}
