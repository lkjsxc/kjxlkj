//! Git sign column model.
//!
//! Gutter signs for added/changed/deleted lines compared to the index or HEAD.
//! See /docs/spec/features/git/gitsigns.md

/// Sign type for a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignType {
    /// Lines added (not in base).
    Add,
    /// Lines changed (differ from base).
    Change,
    /// Lines deleted below this position.
    Delete,
    /// Lines deleted above this position.
    TopDelete,
    /// Lines changed and some deleted.
    ChangeDelete,
}

impl SignType {
    /// Display character for this sign type.
    pub fn char(self) -> char {
        match self {
            Self::Add => '│',
            Self::Change => '│',
            Self::Delete => '_',
            Self::TopDelete => '‾',
            Self::ChangeDelete => '~',
        }
    }

    /// Highlight group name for this sign type.
    pub fn highlight(self) -> &'static str {
        match self {
            Self::Add => "GitSignsAdd",
            Self::Change => "GitSignsChange",
            Self::Delete => "GitSignsDelete",
            Self::TopDelete => "GitSignsTopDelete",
            Self::ChangeDelete => "GitSignsChangedelete",
        }
    }
}

/// A contiguous range of changed lines (a hunk).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// 0-based start line in the buffer.
    pub start: usize,
    /// Number of lines in this hunk.
    pub count: usize,
    /// Type of change.
    pub sign: SignType,
}

/// Base comparison target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitBase {
    /// Compare against staged (index).
    Index,
    /// Compare against HEAD commit.
    Head,
}

/// Per-buffer git sign state.
#[derive(Debug, Clone)]
pub struct GitSignState {
    /// File path this state belongs to.
    pub file: String,
    /// Computed hunks in line order.
    pub hunks: Vec<Hunk>,
    /// What we compare against.
    pub base: GitBase,
    /// Current branch name (if known).
    pub branch: Option<String>,
    /// Counts: added, modified, removed lines.
    pub added: usize,
    pub modified: usize,
    pub removed: usize,
}

impl GitSignState {
    /// Create empty state for a file.
    pub fn new(file: String, base: GitBase) -> Self {
        Self {
            file,
            hunks: Vec::new(),
            base,
            branch: None,
            added: 0,
            modified: 0,
            removed: 0,
        }
    }

    /// Replace hunks and recompute counts.
    pub fn set_hunks(&mut self, hunks: Vec<Hunk>) {
        self.added = 0;
        self.modified = 0;
        self.removed = 0;
        for h in &hunks {
            match h.sign {
                SignType::Add => self.added += h.count,
                SignType::Change | SignType::ChangeDelete => self.modified += h.count,
                SignType::Delete | SignType::TopDelete => self.removed += h.count,
            }
        }
        self.hunks = hunks;
    }

    /// Sign type at a given buffer line (0-based). Returns None if clean.
    pub fn sign_at(&self, line: usize) -> Option<SignType> {
        for h in &self.hunks {
            if line >= h.start && line < h.start + h.count {
                return Some(h.sign);
            }
        }
        None
    }

    /// Next hunk start line after `line`. Wraps around.
    pub fn next_hunk(&self, line: usize) -> Option<usize> {
        if self.hunks.is_empty() {
            return None;
        }
        // Find first hunk starting after line.
        for h in &self.hunks {
            if h.start > line {
                return Some(h.start);
            }
        }
        // Wrap to first hunk.
        Some(self.hunks[0].start)
    }

    /// Previous hunk start line before `line`. Wraps around.
    pub fn prev_hunk(&self, line: usize) -> Option<usize> {
        if self.hunks.is_empty() {
            return None;
        }
        // Find last hunk starting before line.
        for h in self.hunks.iter().rev() {
            if h.start < line {
                return Some(h.start);
            }
        }
        // Wrap to last hunk.
        Some(self.hunks.last().unwrap().start)
    }

    /// Summary string like "+2 ~1 -0".
    pub fn summary(&self) -> String {
        format!("+{} ~{} -{}", self.added, self.modified, self.removed)
    }
}

#[cfg(test)]
#[path = "gitsigns_tests.rs"]
mod tests;
