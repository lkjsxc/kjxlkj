//! Extended git types: log, blame, branches.

use crate::git_diff::DiffHunk;
use serde::{Deserialize, Serialize};

/// A git log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

/// Parse `git log --format="%H%n%an%n%aI%n%s"` output.
pub fn parse_log(output: &str) -> Vec<LogEntry> {
    let lines: Vec<&str> = output.lines().collect();
    let mut entries = Vec::new();
    let mut i = 0;
    while i + 3 < lines.len() {
        entries.push(LogEntry {
            hash: lines[i].to_string(),
            author: lines[i + 1].to_string(),
            date: lines[i + 2].to_string(),
            message: lines[i + 3].to_string(),
        });
        i += 4;
    }
    entries
}

/// A blame entry for a range of lines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameEntry {
    pub hash: String,
    pub author: String,
    pub line_start: usize,
    pub line_count: usize,
}

/// Parse `git blame --porcelain` output (simplified).
pub fn parse_blame_output(output: &str) -> Vec<BlameEntry> {
    let mut entries = Vec::new();
    let mut current_hash = String::new();
    let mut current_author = String::new();
    let mut current_line: usize = 0;
    let mut current_count: usize = 1;

    for line in output.lines() {
        if line.len() >= 40 && line.chars().take(40).all(|c| c.is_ascii_hexdigit()) {
            // Commit header line: <hash> <orig-line> <final-line> [<count>]
            let parts: Vec<&str> = line.split_whitespace().collect();
            current_hash = parts[0].to_string();
            if parts.len() >= 3 {
                current_line = parts[2].parse().unwrap_or(1);
            }
            if parts.len() >= 4 {
                current_count = parts[3].parse().unwrap_or(1);
            }
        } else if let Some(author) = line.strip_prefix("author ") {
            current_author = author.to_string();
        } else if line.starts_with('\t') {
            // Content line — finalize this entry.
            entries.push(BlameEntry {
                hash: current_hash.clone(),
                author: current_author.clone(),
                line_start: current_line,
                line_count: current_count,
            });
        }
    }
    entries
}

/// Branch info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub upstream: Option<String>,
}

/// Count total added and removed lines across hunks.
pub fn count_changes(hunks: &[DiffHunk]) -> (usize, usize) {
    let mut added = 0usize;
    let mut removed = 0usize;
    for hunk in hunks {
        for line in &hunk.lines {
            match line.kind {
                crate::git_diff::DiffLineKind::Added => added += 1,
                crate::git_diff::DiffLineKind::Removed => removed += 1,
                _ => {}
            }
        }
    }
    (added, removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_log_entries() {
        let output = "abc123\nAlice\n2025-01-01\nInitial commit\ndef456\nBob\n2025-01-02\nFix bug\n";
        let entries = parse_log(output);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].author, "Alice");
        assert_eq!(entries[1].message, "Fix bug");
    }

    #[test]
    fn count_changes_basic() {
        use crate::git_diff::{DiffHunk, DiffLine, DiffLineKind};
        let hunks = vec![DiffHunk {
            old_start: 1,
            old_count: 2,
            new_start: 1,
            new_count: 3,
            lines: vec![
                DiffLine { kind: DiffLineKind::Context, content: "a".into() },
                DiffLine { kind: DiffLineKind::Added, content: "b".into() },
                DiffLine { kind: DiffLineKind::Removed, content: "c".into() },
            ],
        }];
        assert_eq!(count_changes(&hunks), (1, 1));
    }
}
