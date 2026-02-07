//! kjxlkj-service-git: Git integration types and parsers.

pub mod git_diff;
pub mod git_full;
pub mod git_status;

pub use git_diff::{DiffHunk, DiffLine, GutterSign, compute_gutter_signs, parse_diff_hunks};
pub use git_full::{BlameEntry, BranchInfo, LogEntry, count_changes, parse_blame_output, parse_log};
pub use git_status::{FileStatus, StatusEntry, detect_branch, parse_status_line};
