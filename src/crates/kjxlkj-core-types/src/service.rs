//! Service message types for inter-crate communication.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{BufferId, TerminalId};

/// Requests sent from core to services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceRequest {
    /// Read file contents from disk.
    FileRead(PathBuf),
    /// Write buffer content to disk.
    FileWrite(PathBuf, String),
    /// Watch a file for external changes.
    FileWatch(PathBuf),
    /// Initialize an LSP server.
    LspInitialize(String),
    /// Request completions at position.
    LspCompletion {
        buffer_id: BufferId,
        line: usize,
        col: usize,
    },
    /// Request hover info.
    LspHover {
        buffer_id: BufferId,
        line: usize,
        col: usize,
    },
    /// Request goto definition.
    LspGotoDefinition {
        buffer_id: BufferId,
        line: usize,
        col: usize,
    },
    /// Request git repository status.
    GitStatus,
    /// Request inline diff hunks for a buffer.
    GitDiff(BufferId),
    /// Spawn a PTY process.
    TerminalSpawn {
        command: String,
        env: Vec<(String, String)>,
    },
    /// Write input bytes to PTY.
    TerminalWrite(TerminalId, Vec<u8>),
    /// Resize PTY.
    TerminalResize {
        terminal_id: TerminalId,
        cols: u16,
        rows: u16,
    },
    /// Close a terminal.
    TerminalClose(TerminalId),
    /// Fuzzy file/symbol search.
    IndexFind(String),
    /// Shutdown the service.
    Shutdown,
}

/// Responses sent from services back to core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceResponse {
    /// File read result.
    FileContents(PathBuf, Vec<u8>),
    /// File write confirmation.
    FileWritten(PathBuf),
    /// File externally changed.
    FileChanged(PathBuf),
    /// File operation error.
    FileError(PathBuf, String),
    /// LSP completion items.
    LspCompletions(Vec<CompletionItem>),
    /// LSP hover result.
    LspHoverResult(String),
    /// LSP diagnostics for a buffer.
    LspDiagnostics(BufferId, Vec<Diagnostic>),
    /// LSP goto result.
    LspGotoResult(Vec<Location>),
    /// Git status entries.
    GitStatusResult(Vec<GitStatusEntry>),
    /// Git diff hunks.
    GitDiffResult(BufferId, Vec<DiffHunk>),
    /// PTY output bytes.
    TerminalOutput(TerminalId, Vec<u8>),
    /// PTY process exited.
    TerminalExited(TerminalId, i32),
    /// Terminal spawned with ID.
    TerminalSpawned(TerminalId),
    /// Index search results.
    IndexResults(Vec<String>),
}

/// A single LSP completion item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
}

/// An LSP diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// A source code location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
    pub col: usize,
}

/// A git status entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatusEntry {
    pub path: PathBuf,
    pub status: GitFileStatus,
}

/// Git file status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitFileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
}

/// A diff hunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// A single diff line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
}

/// Diff line type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_request_variants() {
        let _ = ServiceRequest::FileRead(PathBuf::from("test.txt"));
        let _ = ServiceRequest::GitStatus;
        let _ = ServiceRequest::Shutdown;
    }
}
