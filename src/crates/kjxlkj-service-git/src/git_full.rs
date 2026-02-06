/// Full git integration — diff, log, blame, branch operations.

use std::collections::HashMap;

/// A diff hunk showing changes between two versions.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffHunk {
    pub old_start: usize, pub old_count: usize,
    pub new_start: usize, pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff.
#[derive(Debug, Clone, PartialEq)]
pub enum DiffLine { Context(String), Added(String), Removed(String) }

/// A git log entry.
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub hash: String, pub short_hash: String,
    pub author: String, pub date: String, pub message: String,
}

/// A blame annotation for a line.
#[derive(Debug, Clone, PartialEq)]
pub struct BlameEntry {
    pub hash: String, pub author: String,
    pub date: String, pub line: usize, pub original_line: usize,
}

/// Git branch info.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchInfo { pub name: String, pub is_current: bool, pub tracking: Option<String> }

/// Parse a unified diff into hunks.
pub fn parse_diff(diff_output: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;
    for line in diff_output.lines() {
        if line.starts_with("@@") {
            if let Some(h) = current_hunk.take() { hunks.push(h); }
            let (os, oc, ns, nc) = parse_hunk_header(line);
            current_hunk = Some(DiffHunk { old_start: os, old_count: oc, new_start: ns, new_count: nc, lines: vec![] });
        } else if let Some(ref mut h) = current_hunk {
            if let Some(rest) = line.strip_prefix('+') { h.lines.push(DiffLine::Added(rest.into())); }
            else if let Some(rest) = line.strip_prefix('-') { h.lines.push(DiffLine::Removed(rest.into())); }
            else { h.lines.push(DiffLine::Context(line.strip_prefix(' ').unwrap_or(line).into())); }
        }
    }
    if let Some(h) = current_hunk { hunks.push(h); }
    hunks
}

fn parse_hunk_header(line: &str) -> (usize, usize, usize, usize) {
    // @@ -old_start,old_count +new_start,new_count @@
    let parts: Vec<&str> = line.split_whitespace().collect();
    let old = parts.get(1).unwrap_or(&"-0,0").trim_start_matches('-');
    let new = parts.get(2).unwrap_or(&"+0,0").trim_start_matches('+');
    let (os, oc) = parse_range(old);
    let (ns, nc) = parse_range(new);
    (os, oc, ns, nc)
}

fn parse_range(s: &str) -> (usize, usize) {
    let mut parts = s.split(',');
    let start = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let count = parts.next().and_then(|p| p.parse().ok()).unwrap_or(1);
    (start, count)
}

/// Parse git log output (oneline format).
pub fn parse_log(output: &str) -> Vec<LogEntry> {
    output.lines().filter_map(|line| {
        let (hash, rest) = line.split_once(' ')?;
        Some(LogEntry { hash: hash.into(), short_hash: hash.chars().take(7).collect(),
            author: String::new(), date: String::new(), message: rest.into() })
    }).collect()
}

/// Gutter sign type for diff indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitSign { Added, Modified, Removed, ChangeDelete }

/// Compute gutter signs from diff hunks.
pub fn compute_signs(hunks: &[DiffHunk]) -> HashMap<usize, GitSign> {
    let mut signs = HashMap::new();
    for hunk in hunks {
        for (i, dl) in hunk.lines.iter().enumerate() {
            let line = hunk.new_start + i;
            match dl {
                DiffLine::Added(_) => { signs.insert(line, GitSign::Added); }
                DiffLine::Removed(_) => { signs.insert(line, GitSign::Removed); }
                _ => {}
            }
        }
    }
    signs
}

/// Count changes in a diff.
pub fn count_changes(hunks: &[DiffHunk]) -> (usize, usize) {
    let (mut added, mut removed) = (0, 0);
    for h in hunks { for l in &h.lines { match l {
        DiffLine::Added(_) => added += 1, DiffLine::Removed(_) => removed += 1, _ => {} }}}
    (added, removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_diff_basic() {
        let diff = "@@ -1,3 +1,4 @@\n context\n+added\n-removed\n context2";
        let hunks = parse_diff(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].lines.len(), 4);
    }

    #[test]
    fn diff_line_types() {
        let diff = "@@ -1,2 +1,3 @@\n+new line\n-old line\n same";
        let hunks = parse_diff(diff);
        assert!(matches!(hunks[0].lines[0], DiffLine::Added(_)));
        assert!(matches!(hunks[0].lines[1], DiffLine::Removed(_)));
    }

    #[test]
    fn parse_log_oneline() {
        let log = "abc1234 Initial commit\ndef5678 Add feature";
        let entries = parse_log(log);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].short_hash, "abc1234");
    }

    #[test]
    fn compute_signs_from_hunks() {
        let diff = "@@ -1,1 +1,2 @@\n context\n+added";
        let hunks = parse_diff(diff);
        let signs = compute_signs(&hunks);
        assert!(signs.values().any(|s| *s == GitSign::Added));
    }

    #[test]
    fn count_changes_basic() {
        let diff = "@@ -1,2 +1,3 @@\n+a\n+b\n-c";
        let hunks = parse_diff(diff);
        let (a, r) = count_changes(&hunks);
        assert_eq!(a, 2); assert_eq!(r, 1);
    }

    #[test]
    fn empty_diff() {
        let hunks = parse_diff("");
        assert!(hunks.is_empty());
    }

    #[test]
    fn hunk_header_parse() {
        let (os, oc, ns, nc) = parse_hunk_header("@@ -10,5 +12,7 @@");
        assert_eq!((os, oc, ns, nc), (10, 5, 12, 7));
    }

    #[test]
    fn branch_info() {
        let b = BranchInfo { name: "main".into(), is_current: true, tracking: Some("origin/main".into()) };
        assert!(b.is_current);
    }
}
