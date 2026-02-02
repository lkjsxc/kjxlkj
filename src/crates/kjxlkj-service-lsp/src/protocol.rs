//! LSP protocol types.

use serde::{Deserialize, Serialize};

/// An LSP request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspRequest {
    /// Request ID.
    pub id: u64,
    /// Method name.
    pub method: String,
    /// Parameters.
    pub params: serde_json::Value,
}

/// An LSP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspResponse {
    /// Request ID.
    pub id: u64,
    /// Result (if successful).
    pub result: Option<serde_json::Value>,
    /// Error (if failed).
    pub error: Option<LspError>,
}

/// An LSP error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspError {
    /// Error code.
    pub code: i32,
    /// Error message.
    pub message: String,
    /// Additional data.
    pub data: Option<serde_json::Value>,
}

/// An LSP notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspNotification {
    /// Method name.
    pub method: String,
    /// Parameters.
    pub params: serde_json::Value,
}

/// Position in a document (LSP format).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LspPosition {
    /// Line number (0-based).
    pub line: u32,
    /// Character offset (0-based).
    pub character: u32,
}

/// Range in a document.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LspRange {
    /// Start position.
    pub start: LspPosition,
    /// End position.
    pub end: LspPosition,
}

/// A text document identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    /// Document URI.
    pub uri: String,
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A diagnostic message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Range of the diagnostic.
    pub range: LspRange,
    /// Severity.
    pub severity: Option<DiagnosticSeverity>,
    /// Diagnostic code.
    pub code: Option<serde_json::Value>,
    /// Source of the diagnostic.
    pub source: Option<String>,
    /// Message.
    pub message: String,
}
