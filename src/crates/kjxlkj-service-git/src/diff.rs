//! Git diff parsing.

/// A diff hunk.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Start line in the old version.
    pub old_start: usize,
    /// Number of old lines.
    pub old_count: usize,
    /// Start line in the new version.
    pub new_start: usize,
    /// Number of new lines.
    pub new_count: usize,
    /// Diff lines.
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff.
#[derive(Debug, Clone)]
pub enum DiffLine {
    Context(String),
    Added(String),
    Removed(String),
}

/// Parse unified diff output into hunks.
pub fn parse_unified_diff(diff: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut current: Option<DiffHunk> = None;

    for line in diff.lines() {
        if line.starts_with("@@ ") {
            if let Some(hunk) = current.take() {
                hunks.push(hunk);
            }
            if let Some(hunk) = parse_hunk_header(line) {
                current = Some(hunk);
            }
        } else if let Some(ref mut hunk) = current {
            if let Some(stripped) = line.strip_prefix('+') {
                hunk.lines.push(DiffLine::Added(stripped.to_string()));
            } else if let Some(stripped) = line.strip_prefix('-') {
                hunk.lines.push(DiffLine::Removed(stripped.to_string()));
            } else if let Some(stripped) = line.strip_prefix(' ') {
                hunk.lines.push(DiffLine::Context(stripped.to_string()));
            }
        }
    }

    if let Some(hunk) = current {
        hunks.push(hunk);
    }

    hunks
}

fn parse_hunk_header(line: &str) -> Option<DiffHunk> {
    // Format: @@ -old_start,old_count +new_start,new_count @@
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }

    let old = parts[1].strip_prefix('-')?;
    let new = parts[2].strip_prefix('+')?;

    let (old_start, old_count) = parse_range(old);
    let (new_start, new_count) = parse_range(new);

    Some(DiffHunk {
        old_start,
        old_count,
        new_start,
        new_count,
        lines: Vec::new(),
    })
}

fn parse_range(s: &str) -> (usize, usize) {
    if let Some((start, count)) = s.split_once(',') {
        (start.parse().unwrap_or(0), count.parse().unwrap_or(0))
    } else {
        (s.parse().unwrap_or(0), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_diff() {
        let diff = "@@ -1,3 +1,4 @@\n context\n+added\n context\n context\n";
        let hunks = parse_unified_diff(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[0].new_count, 4);
    }
}
