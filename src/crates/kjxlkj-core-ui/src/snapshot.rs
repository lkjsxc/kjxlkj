//! Editor snapshots for rendering.

use kjxlkj_core_types::{BufferId, BufferVersion, Cursor, Mode};

use crate::Viewport;

/// Monotonically increasing sequence for snapshot ordering.
/// Prevents "one-key lag" by allowing renderers to discard stale snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SnapshotSeq(u64);

impl SnapshotSeq {
    /// Creates a new sequence with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the next sequence value.
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    /// Returns the raw sequence value.
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// A snapshot of a buffer for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub version: BufferVersion,
    pub name: String,
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub modified: bool,
}

/// Status line content.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    pub mode: Mode,
    pub file_name: String,
    pub modified: bool,
    pub cursor_line: u32,
    pub cursor_col: u32,
    pub line_count: usize,
    pub message: Option<String>,
}

/// A complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Monotonic sequence for render ordering.
    pub seq: SnapshotSeq,
    pub buffer: BufferSnapshot,
    pub status: StatusLine,
    pub command_line: Option<String>,
    pub terminal_size: (u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_line_default() {
        let status = StatusLine::default();
        assert_eq!(status.mode, Mode::Normal);
    }

    #[test]
    fn snapshot_seq_ordering() {
        let seq1 = SnapshotSeq::new(1);
        let seq2 = seq1.next();
        assert!(seq2 > seq1);
        assert_eq!(seq2.value(), 2);
    }

    #[test]
    fn snapshot_seq_monotonic() {
        let mut seq = SnapshotSeq::default();
        for i in 0..100 {
            let next = seq.next();
            assert!(next > seq);
            assert_eq!(next.value(), i + 1);
            seq = next;
        }
    }
}
