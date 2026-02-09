//! LSP protocol types.

use serde::{Deserialize, Serialize};

/// LSP Position (0-based line and character).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// LSP Range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// LSP Location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

/// LSP Diagnostic severity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// LSP Diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub message: String,
    #[serde(default)]
    pub source: Option<String>,
}

/// LSP CompletionItem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub kind: Option<u32>,
}

/// LSP InitializeParams (simplified).
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "processId")]
    pub process_id: Option<u32>,
    #[serde(rename = "rootUri")]
    pub root_uri: Option<String>,
    pub capabilities: serde_json::Value,
}

/// LSP TextDocumentIdentifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

/// LSP TextDocumentPositionParams.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_position() {
        let pos = Position {
            line: 10,
            character: 5,
        };
        let json = serde_json::to_string(&pos).unwrap();
        assert!(json.contains("\"line\":10"));
    }
}
