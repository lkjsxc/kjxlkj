//! Git status types and parsing.

use serde::{Deserialize, Serialize};

/// File status in the working tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Ignored,
    Conflicted,
}

impl FileStatus {
    /// Parse a single status character from `git status --porcelain`.
    fn from_char(c: char) -> Option<Self> {
        match c {
            ' ' => Some(Self::Unmodified),
            'M' => Some(Self::Modified),
            'A' => Some(Self::Added),
            'D' => Some(Self::Deleted),
            'R' => Some(Self::Renamed),
            '?' => Some(Self::Untracked),
            '!' => Some(Self::Ignored),
            'U' => Some(Self::Conflicted),
            _ => None,
        }
    }
}

/// A single status entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEntry {
    pub path: String,
    pub status: FileStatus,
    pub staged: bool,
}

/// Parse a porcelain status line (e.g. "M  src/main.rs").
pub fn parse_status_line(line: &str) -> Option<StatusEntry> {
    if line.len() < 4 {
        return None;
    }
    let bytes = line.as_bytes();
    let index_char = bytes[0] as char;
    let worktree_char = bytes[1] as char;
    let path = line[3..].to_string();

    // Untracked: "?? path"
    if index_char == '?' && worktree_char == '?' {
        return Some(StatusEntry {
            path,
            status: FileStatus::Untracked,
            staged: false,
        });
    }
    // Ignored: "!! path"
    if index_char == '!' && worktree_char == '!' {
        return Some(StatusEntry {
            path,
            status: FileStatus::Ignored,
            staged: false,
        });
    }
    // Staged change takes priority if present
    if index_char != ' ' {
        let status = FileStatus::from_char(index_char)?;
        return Some(StatusEntry {
            path,
            status,
            staged: true,
        });
    }
    // Worktree change
    let status = FileStatus::from_char(worktree_char)?;
    Some(StatusEntry {
        path,
        status,
        staged: false,
    })
}

/// Detect current branch from the .git directory (reads HEAD).
pub fn detect_branch(git_dir: &str) -> Option<String> {
    let head_path = format!("{}/HEAD", git_dir);
    let content = std::fs::read_to_string(head_path).ok()?;
    let trimmed = content.trim();
    if let Some(branch) = trimmed.strip_prefix("ref: refs/heads/") {
        Some(branch.to_string())
    } else if trimmed.len() >= 7 {
        // Detached HEAD — return short hash.
        Some(trimmed[..7].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_modified() {
        let e = parse_status_line(" M src/main.rs").unwrap();
        assert_eq!(e.status, FileStatus::Modified);
        assert!(!e.staged);
        assert_eq!(e.path, "src/main.rs");
    }

    #[test]
    fn parse_added_staged() {
        let e = parse_status_line("A  new_file.rs").unwrap();
        assert_eq!(e.status, FileStatus::Added);
        assert!(e.staged);
    }

    #[test]
    fn parse_untracked() {
        let e = parse_status_line("?? unknown.txt").unwrap();
        assert_eq!(e.status, FileStatus::Untracked);
    }

    #[test]
    fn parse_short_line_returns_none() {
        assert!(parse_status_line("AB").is_none());
    }
}
