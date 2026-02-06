//! Git status, diff, and blame models — data types for git integration.
//!
//! Provides types for representing file status, line-level diffs,
//! and blame annotations without requiring an actual git binary.

/// Status of a file relative to HEAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// A single file's status entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusEntry {
    pub path: String,
    pub status: FileStatus,
    pub staged: bool,
}

/// Diff hunk header with line ranges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub header: String,
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff hunk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    Context(String),
    Added(String),
    Removed(String),
}

/// Blame annotation for a single line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlameLine {
    pub commit_hash: String,
    pub author: String,
    pub line_number: usize,
    pub content: String,
}

/// Gutter indicator for changed lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GutterSign {
    Added,
    Modified,
    Removed,
}

/// Parse a simplified unified diff text into hunks.
pub fn parse_diff_hunks(diff_text: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut current: Option<DiffHunk> = None;
    for line in diff_text.lines() {
        if line.starts_with("@@") {
            if let Some(h) = current.take() { hunks.push(h); }
            let (old_start, old_count, new_start, new_count) = parse_hunk_header(line);
            current = Some(DiffHunk {
                old_start, old_count, new_start, new_count,
                header: line.to_string(), lines: Vec::new(),
            });
        } else if let Some(ref mut h) = current {
            let dl = if let Some(rest) = line.strip_prefix('+') {
                DiffLine::Added(rest.to_string())
            } else if let Some(rest) = line.strip_prefix('-') {
                DiffLine::Removed(rest.to_string())
            } else {
                DiffLine::Context(line.strip_prefix(' ').unwrap_or(line).to_string())
            };
            h.lines.push(dl);
        }
    }
    if let Some(h) = current { hunks.push(h); }
    hunks
}

fn parse_hunk_header(header: &str) -> (usize, usize, usize, usize) {
    // Format: @@ -old_start,old_count +new_start,new_count @@
    let parts: Vec<&str> = header.split_whitespace().collect();
    let (os, oc) = parse_range(parts.get(1).unwrap_or(&"-0,0"));
    let (ns, nc) = parse_range(parts.get(2).unwrap_or(&"+0,0"));
    (os, oc, ns, nc)
}

fn parse_range(s: &str) -> (usize, usize) {
    let s = s.trim_start_matches(['-', '+']);
    let mut parts = s.split(',');
    let start: usize = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let count: usize = parts.next().and_then(|p| p.parse().ok()).unwrap_or(1);
    (start, count)
}

/// Compute gutter signs from diff hunks.
pub fn compute_gutter_signs(hunks: &[DiffHunk]) -> Vec<(usize, GutterSign)> {
    let mut signs = Vec::new();
    for hunk in hunks {
        let mut new_line = hunk.new_start;
        for dl in &hunk.lines {
            match dl {
                DiffLine::Added(_) => { signs.push((new_line, GutterSign::Added)); new_line += 1; }
                DiffLine::Removed(_) => { signs.push((new_line, GutterSign::Removed)); }
                DiffLine::Context(_) => { new_line += 1; }
            }
        }
    }
    signs
}

/// Build blame annotations from raw blame output lines.
pub fn parse_blame_output(lines: &[&str]) -> Vec<BlameLine> {
    lines.iter().enumerate().filter_map(|(i, line)| {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 {
            Some(BlameLine {
                commit_hash: parts[0].to_string(),
                author: parts[1].to_string(),
                line_number: i + 1,
                content: parts[2].to_string(),
            })
        } else { None }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_status_variants() {
        assert_ne!(FileStatus::Modified, FileStatus::Added);
        let e = StatusEntry { path: "foo.rs".into(), status: FileStatus::Modified, staged: false };
        assert!(!e.staged);
    }

    #[test]
    fn parse_simple_diff() {
        let diff = "@@ -1,3 +1,4 @@\n hello\n-old\n+new\n+added\n world";
        let hunks = parse_diff_hunks(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[0].new_count, 4);
        assert_eq!(hunks[0].lines.len(), 5);
    }

    #[test]
    fn diff_line_types() {
        let diff = "@@ -1,2 +1,2 @@\n-removed\n+added";
        let hunks = parse_diff_hunks(diff);
        assert!(matches!(hunks[0].lines[0], DiffLine::Removed(_)));
        assert!(matches!(hunks[0].lines[1], DiffLine::Added(_)));
    }

    #[test]
    fn gutter_signs_from_hunks() {
        let diff = "@@ -1,2 +1,3 @@\n context\n+added\n context2";
        let hunks = parse_diff_hunks(diff);
        let signs = compute_gutter_signs(&hunks);
        assert!(signs.iter().any(|(_, s)| *s == GutterSign::Added));
    }

    #[test]
    fn blame_parsing() {
        let lines = vec!["abc123 author1 line content", "def456 author2 other content"];
        let blame = parse_blame_output(&lines);
        assert_eq!(blame.len(), 2);
        assert_eq!(blame[0].commit_hash, "abc123");
        assert_eq!(blame[1].line_number, 2);
    }

    #[test]
    fn multiple_hunks() {
        let diff = "@@ -1,2 +1,2 @@\n-a\n+b\n@@ -10,1 +10,1 @@\n-c\n+d";
        let hunks = parse_diff_hunks(diff);
        assert_eq!(hunks.len(), 2);
        assert_eq!(hunks[1].old_start, 10);
    }

    #[test]
    fn parse_range_single() {
        assert_eq!(parse_range("-5"), (5, 1));
        assert_eq!(parse_range("+3,7"), (3, 7));
    }
}
