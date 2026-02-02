//! Quickfix entry types.
//!
//! Types for representing quickfix entries.

use std::path::PathBuf;

/// Kind of quickfix entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickfixKind {
    /// Error.
    Error,
    /// Warning.
    Warning,
    /// Info.
    Info,
    /// Note.
    Note,
    /// Search result.
    Search,
}

/// A single quickfix entry.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub col: usize,
    /// Entry text/message.
    pub text: String,
    /// Entry type.
    pub kind: QuickfixKind,
}

impl QuickfixEntry {
    /// Creates a new quickfix entry.
    pub fn new(path: PathBuf, line: usize, col: usize, text: &str) -> Self {
        Self {
            path,
            line,
            col,
            text: text.to_string(),
            kind: QuickfixKind::Error,
        }
    }

    /// Sets the kind.
    pub fn with_kind(mut self, kind: QuickfixKind) -> Self {
        self.kind = kind;
        self
    }

    /// Returns formatted location string.
    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.path.display(), self.line, self.col)
    }
}
