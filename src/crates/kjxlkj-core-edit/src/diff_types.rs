//! Diff types and hunk definitions.

/// Kind of diff change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    /// Lines added.
    Add,
    /// Lines deleted.
    Delete,
    /// Lines changed.
    Change,
}

/// A diff hunk.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Start line in the old version (1-based).
    pub old_start: usize,
    /// Number of lines in old version.
    pub old_count: usize,
    /// Start line in the new version (1-based).
    pub new_start: usize,
    /// Number of lines in new version.
    pub new_count: usize,
    /// Hunk type.
    pub kind: DiffKind,
}

impl DiffHunk {
    /// Creates a new diff hunk.
    pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
        let kind = if old_count == 0 {
            DiffKind::Add
        } else if new_count == 0 {
            DiffKind::Delete
        } else {
            DiffKind::Change
        };

        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            kind,
        }
    }

    /// Returns the hunk header in unified diff format.
    pub fn header(&self) -> String {
        format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_count, self.new_start, self.new_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_hunk_kind() {
        let add = DiffHunk::new(1, 0, 1, 2);
        assert_eq!(add.kind, DiffKind::Add);

        let delete = DiffHunk::new(1, 2, 1, 0);
        assert_eq!(delete.kind, DiffKind::Delete);

        let change = DiffHunk::new(1, 2, 1, 3);
        assert_eq!(change.kind, DiffKind::Change);
    }

    #[test]
    fn test_diff_hunk_header() {
        let hunk = DiffHunk::new(1, 5, 1, 7);
        assert_eq!(hunk.header(), "@@ -1,5 +1,7 @@");
    }
}
