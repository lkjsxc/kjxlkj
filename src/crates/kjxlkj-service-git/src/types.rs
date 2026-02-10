//! Git domain types.

use std::path::PathBuf;

/// File status in the working tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Untracked,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmerged,
    Ignored,
}

/// A status entry for a file in the repository.
#[derive(Debug, Clone)]
pub struct StatusEntry {
    pub path: PathBuf,
    pub status: FileStatus,
    /// Index status (staged).
    pub staged: Option<FileStatus>,
}

/// Gutter sign type for a changed line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitSign {
    Added,
    Changed,
    Removed,
    TopDelete,
    ChangeDelete,
}

/// A diff hunk describing a contiguous changed region.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Start line in the current buffer (1-based).
    pub start_line: usize,
    /// Number of lines in the hunk.
    pub count: usize,
    /// The sign type for gutter display.
    pub sign: GitSign,
    /// The old text (from HEAD) for preview.
    pub old_text: Vec<String>,
    /// The new text (working tree) for preview.
    pub new_text: Vec<String>,
}

/// A blame entry for a single line or range.
#[derive(Debug, Clone)]
pub struct BlameEntry {
    pub commit: String,
    pub author: String,
    pub date: String,
    pub summary: String,
    pub line_start: usize,
    pub line_count: usize,
}

/// Commands sent to the git service.
#[derive(Debug, Clone)]
pub enum GitCommand {
    /// Refresh status for all files.
    RefreshStatus,
    /// Compute hunks for a specific buffer.
    ComputeHunks { file: PathBuf },
    /// Get blame for a file.
    Blame { file: PathBuf },
    /// Stage a hunk.
    StageHunk { file: PathBuf, hunk_index: usize },
    /// Reset a hunk to HEAD.
    ResetHunk { file: PathBuf, hunk_index: usize },
    /// Open diff view for a file.
    Diff { file: PathBuf },
}

/// Notifications from the git service to the core.
#[derive(Debug, Clone)]
pub enum GitNotification {
    /// Updated status for all files.
    StatusUpdated(Vec<StatusEntry>),
    /// Updated hunks for a file.
    HunksUpdated { file: PathBuf, hunks: Vec<DiffHunk> },
    /// Blame results for a file.
    BlameResult {
        file: PathBuf,
        entries: Vec<BlameEntry>,
    },
    /// Diff content for viewing.
    DiffContent { file: PathBuf, content: String },
    /// Error from git operation.
    Error(String),
}

/// Action to take on a hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkAction {
    Stage,
    Reset,
    Preview,
}
