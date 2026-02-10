//! LSP response types returned from request handling.

use crate::types::{CompletionItem, Range};
use std::collections::HashMap;
use std::path::PathBuf;

/// Responses from LSP request handling.
#[derive(Debug, Clone)]
pub enum LspResponse {
    Initialized,
    Completions(Vec<CompletionItem>),
    Hover(Option<HoverInfo>),
    Locations(Vec<Location>),
    WorkspaceEdit(WorkspaceEdit),
    TextEdits(Vec<TextEdit>),
    ShutdownAck,
}

/// Hover information returned by the server.
#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub contents: String,
    pub range: Option<Range>,
}

/// A location in a file returned by go-to operations.
#[derive(Debug, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub range: Range,
}

/// A workspace edit (set of file edits).
#[derive(Debug, Clone, Default)]
pub struct WorkspaceEdit {
    pub changes: HashMap<PathBuf, Vec<TextEdit>>,
}

/// A single text edit.
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}
