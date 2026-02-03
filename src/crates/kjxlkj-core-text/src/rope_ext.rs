//! Rope extension utilities.

use ropey::Rope;

/// Extension trait for Rope operations.
pub trait RopeExt {
    /// Get the byte length of the rope.
    fn byte_len(&self) -> usize;

    /// Check if the rope is empty.
    fn is_empty(&self) -> bool;

    /// Get the last line index (0-indexed).
    fn last_line_idx(&self) -> usize;
}

impl RopeExt for Rope {
    fn byte_len(&self) -> usize {
        self.len_bytes()
    }

    fn is_empty(&self) -> bool {
        self.len_chars() == 0
    }

    fn last_line_idx(&self) -> usize {
        self.len_lines().saturating_sub(1)
    }
}
