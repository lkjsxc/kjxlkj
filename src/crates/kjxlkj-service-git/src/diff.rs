//! Git diff parsing.

/// Type of diff hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkType {
    /// Added lines.
    Add,
    /// Removed lines.
    Remove,
    /// Changed lines (combination).
    Change,
}

/// A diff hunk.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Start line in the old file.
    pub old_start: usize,
    /// Number of lines in the old file.
    pub old_count: usize,
    /// Start line in the new file.
    pub new_start: usize,
    /// Number of lines in the new file.
    pub new_count: usize,
    /// The hunk type.
    pub hunk_type: HunkType,
    /// The diff lines.
    pub lines: Vec<DiffLine>,
}

/// A line in a diff.
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line type.
    pub line_type: DiffLineType,
    /// Line content.
    pub content: String,
}

/// Type of diff line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    /// Context (unchanged).
    Context,
    /// Added.
    Add,
    /// Removed.
    Remove,
}

impl DiffHunk {
    /// Parses hunks from a unified diff.
    pub fn parse_unified(diff: &str) -> Vec<Self> {
        let mut hunks = Vec::new();
        let lines: Vec<&str> = diff.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].starts_with("@@") {
                if let Some((hunk, consumed)) = Self::parse_hunk(&lines[i..]) {
                    hunks.push(hunk);
                    i += consumed;
                    continue;
                }
            }
            i += 1;
        }

        hunks
    }

    fn parse_hunk(lines: &[&str]) -> Option<(Self, usize)> {
        let header = lines.first()?;
        if !header.starts_with("@@") {
            return None;
        }

        // Parse @@ -old_start,old_count +new_start,new_count @@
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let (old_start, old_count) = Self::parse_range(parts[1].trim_start_matches('-'))?;
        let (new_start, new_count) = Self::parse_range(parts[2].trim_start_matches('+'))?;

        let mut diff_lines = Vec::new();
        let mut consumed = 1;

        for line in &lines[1..] {
            if line.starts_with("@@") || line.starts_with("diff ") {
                break;
            }

            let (line_type, content) = if let Some(rest) = line.strip_prefix('+') {
                (DiffLineType::Add, rest.to_string())
            } else if let Some(rest) = line.strip_prefix('-') {
                (DiffLineType::Remove, rest.to_string())
            } else if let Some(rest) = line.strip_prefix(' ') {
                (DiffLineType::Context, rest.to_string())
            } else {
                (DiffLineType::Context, line.to_string())
            };

            diff_lines.push(DiffLine { line_type, content });
            consumed += 1;
        }

        let hunk_type = if old_count == 0 {
            HunkType::Add
        } else if new_count == 0 {
            HunkType::Remove
        } else {
            HunkType::Change
        };

        Some((
            Self {
                old_start,
                old_count,
                new_start,
                new_count,
                hunk_type,
                lines: diff_lines,
            },
            consumed,
        ))
    }

    fn parse_range(s: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = s.split(',').collect();
        let start = parts.first()?.parse().ok()?;
        let count = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
        Some((start, count))
    }
}
