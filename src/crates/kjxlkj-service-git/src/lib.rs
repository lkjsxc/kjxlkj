//! kjxlkj-service-git: Git integration types and parsers.

pub mod git_diff;
pub mod git_full;
pub mod git_status;

pub use git_diff::{compute_gutter_signs, parse_diff_hunks, DiffHunk, DiffLine, GutterSign};
pub use git_full::{
    count_changes, parse_blame_output, parse_log, BlameEntry, BranchInfo, LogEntry,
};
pub use git_status::{detect_branch, parse_status_line, FileStatus, StatusEntry};
