//! Helper types for normal mode parsing.

use kjxlkj_core_types::motion::Direction;

/// Kind of character pending for find/replace.
#[derive(Debug, Clone, Copy)]
pub enum CharPendingKind {
    /// Find character (f/F).
    Find(Direction),
    /// Till character (t/T).
    Till(Direction),
    /// Replace character (r).
    Replace,
}

/// Kind of mark pending.
#[derive(Debug, Clone, Copy)]
pub enum MarkPendingKind {
    /// Set mark (m).
    Set,
    /// Jump to mark line (').
    Jump,
    /// Jump to mark exact position (`).
    JumpColumn,
}
