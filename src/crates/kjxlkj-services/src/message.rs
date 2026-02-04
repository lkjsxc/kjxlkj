//! Service message types.

use kjxlkj_core_types::BufferId;

/// Message from a service.
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    /// File system event.
    FsEvent(FsEvent),
    /// LSP notification.
    LspNotification(LspNotification),
    /// Git status update.
    GitUpdate(GitUpdate),
    /// Index results.
    IndexResult(IndexResult),
    /// Terminal output.
    TerminalOutput(TerminalOutput),
}

/// File system event.
#[derive(Debug, Clone)]
pub struct FsEvent {
    pub path: std::path::PathBuf,
    pub kind: FsEventKind,
}

/// File system event kind.
#[derive(Debug, Clone, Copy)]
pub enum FsEventKind {
    Created,
    Modified,
    Deleted,
}

/// LSP notification.
#[derive(Debug, Clone)]
pub struct LspNotification {
    pub buffer_id: Option<BufferId>,
    pub kind: LspNotificationKind,
}

/// LSP notification kind.
#[derive(Debug, Clone)]
pub enum LspNotificationKind {
    Diagnostics(Vec<Diagnostic>),
    Hover(String),
    Completion(Vec<CompletionItem>),
}

/// A diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Completion item.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
}

/// Completion kind.
#[derive(Debug, Clone, Copy)]
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
    Keyword,
    Snippet,
}

/// Git status update.
#[derive(Debug, Clone)]
pub struct GitUpdate {
    pub kind: GitUpdateKind,
}

/// Git update kind.
#[derive(Debug, Clone)]
pub enum GitUpdateKind {
    StatusChanged,
    BranchChanged(String),
    DiffReady(Vec<GitHunk>),
}

/// Git hunk (diff region).
#[derive(Debug, Clone)]
pub struct GitHunk {
    pub start_line: usize,
    pub line_count: usize,
    pub kind: GitHunkKind,
}

/// Git hunk kind.
#[derive(Debug, Clone, Copy)]
pub enum GitHunkKind {
    Added,
    Modified,
    Deleted,
}

/// Index search result.
#[derive(Debug, Clone)]
pub struct IndexResult {
    pub query: String,
    pub matches: Vec<IndexMatch>,
}

/// Index match.
#[derive(Debug, Clone)]
pub struct IndexMatch {
    pub path: std::path::PathBuf,
    pub line: Option<usize>,
    pub score: f64,
}

/// Terminal output.
#[derive(Debug, Clone)]
pub struct TerminalOutput {
    pub terminal_id: u64,
    pub data: Vec<u8>,
}

/// Request to a service.
#[derive(Debug, Clone)]
pub enum ServiceRequest {
    /// Read a file.
    ReadFile(std::path::PathBuf),
    /// Write a file.
    WriteFile(std::path::PathBuf, Vec<u8>),
    /// Start LSP for a language.
    StartLsp(String),
    /// Request completion.
    RequestCompletion(BufferId, usize, usize),
    /// Get git status.
    GetGitStatus,
    /// Search files.
    SearchFiles(String),
    /// Run terminal command.
    RunCommand(String),
}

/// Response from a service.
#[derive(Debug, Clone)]
pub enum ServiceResponse {
    /// File content.
    FileContent(Vec<u8>),
    /// File written successfully.
    FileWritten,
    /// Error response.
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_types() {
        let msg = ServiceMessage::FsEvent(FsEvent {
            path: std::path::PathBuf::from("/test"),
            kind: FsEventKind::Modified,
        });
        // Verify message can be created
        assert!(matches!(msg, ServiceMessage::FsEvent(_)));
    }
}
