//! Git diff parsing and gutter signs.

use serde::{Deserialize, Serialize};

/// Kind of diff line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}

/// A single line in a diff hunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
}

/// A diff hunk with header info and lines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// Gutter sign for the editor margin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GutterSign {
    Added,
    Modified,
    Removed,
}

/// Parse a hunk header like `@@ -1,5 +1,7 @@`.
pub fn parse_hunk_header(header: &str) -> Option<(usize, usize, usize, usize)> {
    let header = header.trim();
    let at_start = header.find("@@")?;
    let rest = &header[at_start + 2..];
    let at_end = rest.find("@@")?;
    let range_part = rest[..at_end].trim();

    let mut parts = range_part.split_whitespace();
    let old = parts.next()?.strip_prefix('-')?;
    let new = parts.next()?.strip_prefix('+')?;

    let (old_start, old_count) = parse_range_pair(old)?;
    let (new_start, new_count) = parse_range_pair(new)?;
    Some((old_start, old_count, new_start, new_count))
}

fn parse_range_pair(s: &str) -> Option<(usize, usize)> {
    if let Some((a, b)) = s.split_once(',') {
        Some((a.parse().ok()?, b.parse().ok()?))
    } else {
        Some((s.parse().ok()?, 1))
    }
}

/// Parse unified diff text into hunks.
pub fn parse_diff_hunks(diff_text: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut current: Option<DiffHunk> = None;

    for line in diff_text.lines() {
        if line.starts_with("@@") {
            if let Some(hunk) = current.take() {
                hunks.push(hunk);
            }
            if let Some((os, oc, ns, nc)) = parse_hunk_header(line) {
                current = Some(DiffHunk {
                    old_start: os,
                    old_count: oc,
                    new_start: ns,
                    new_count: nc,
                    lines: Vec::new(),
                });
            }
        } else if let Some(ref mut hunk) = current {
            let (kind, content) = if let Some(rest) = line.strip_prefix('+') {
                (DiffLineKind::Added, rest)
            } else if let Some(rest) = line.strip_prefix('-') {
                (DiffLineKind::Removed, rest)
            } else if let Some(rest) = line.strip_prefix(' ') {
                (DiffLineKind::Context, rest)
            } else {
                (DiffLineKind::Context, line)
            };
            hunk.lines.push(DiffLine {
                kind,
                content: content.to_string(),
            });
        }
    }
    if let Some(hunk) = current {
        hunks.push(hunk);
    }
    hunks
}

/// Compute gutter signs from hunks (line number -> sign).
pub fn compute_gutter_signs(hunks: &[DiffHunk]) -> Vec<(usize, GutterSign)> {
    let mut signs = Vec::new();
    for hunk in hunks {
        let mut new_line = hunk.new_start;
        let mut had_remove = false;
        for dl in &hunk.lines {
            match dl.kind {
                DiffLineKind::Added => {
                    signs.push((new_line, GutterSign::Added));
                    new_line += 1;
                }
                DiffLineKind::Removed => {
                    had_remove = true;
                }
                DiffLineKind::Context => {
                    if had_remove {
                        signs.push((new_line, GutterSign::Modified));
                        had_remove = false;
                    }
                    new_line += 1;
                }
            }
        }
        if had_remove && new_line > 0 {
            signs.push((new_line.saturating_sub(1), GutterSign::Removed));
        }
    }
    signs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        let h = parse_hunk_header("@@ -10,5 +12,7 @@ fn main").unwrap();
        assert_eq!(h, (10, 5, 12, 7));
    }

    #[test]
    fn parse_header_no_count() {
        let h = parse_hunk_header("@@ -1 +1 @@").unwrap();
        assert_eq!(h, (1, 1, 1, 1));
    }

    #[test]
    fn parse_hunks_basic() {
        let diff = "@@ -1,3 +1,4 @@\n context\n+added\n context\n context\n";
        let hunks = parse_diff_hunks(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].lines.len(), 4);
    }
}
