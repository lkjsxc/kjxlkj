//! Git hunk types.

/// Kind of hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkKind {
    /// Added lines.
    Add,
    /// Removed lines.
    Remove,
    /// Changed lines.
    Change,
}

/// A diff hunk.
#[derive(Debug, Clone)]
pub struct Hunk {
    /// Kind.
    pub kind: HunkKind,
    /// Start line.
    pub start: usize,
    /// Number of lines.
    pub count: usize,
}

impl Hunk {
    /// Creates a new hunk.
    pub fn new(kind: HunkKind, start: usize, count: usize) -> Self {
        Self { kind, start, count }
    }

    /// Creates an add hunk.
    pub fn add(start: usize, count: usize) -> Self {
        Self::new(HunkKind::Add, start, count)
    }

    /// Creates a remove hunk.
    pub fn remove(start: usize, count: usize) -> Self {
        Self::new(HunkKind::Remove, start, count)
    }

    /// Creates a change hunk.
    pub fn change(start: usize, count: usize) -> Self {
        Self::new(HunkKind::Change, start, count)
    }
}
