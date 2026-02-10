//! LSP protocol types.

use std::path::PathBuf;

/// Position in a document (0-based line and character).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// A range in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Completion item kind (subset of LSP CompletionItemKind).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Folder,
}

/// A completion item returned by the language server.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub sort_text: Option<String>,
}

impl CompletionItem {
    pub fn new(label: impl Into<String>, kind: CompletionKind) -> Self {
        Self {
            label: label.into(),
            kind,
            detail: None,
            documentation: None,
            insert_text: None,
            sort_text: None,
        }
    }
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A diagnostic message from the language server.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<String>,
}

/// LSP request types sent to the language server.
#[derive(Debug, Clone)]
pub enum LspRequest {
    Initialize {
        root: PathBuf,
    },
    Completion {
        file: PathBuf,
        position: Position,
    },
    Hover {
        file: PathBuf,
        position: Position,
    },
    Definition {
        file: PathBuf,
        position: Position,
    },
    References {
        file: PathBuf,
        position: Position,
    },
    Rename {
        file: PathBuf,
        position: Position,
        new_name: String,
    },
    Format {
        file: PathBuf,
    },
    Shutdown,
}

/// LSP notification types received from the language server.
#[derive(Debug, Clone)]
pub enum LspNotification {
    Diagnostics {
        file: PathBuf,
        diagnostics: Vec<Diagnostic>,
    },
    Progress {
        token: String,
        message: String,
        percentage: Option<u32>,
    },
    ServerReady,
    ServerError(String),
}
