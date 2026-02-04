//! Line information.

/// Information about a line in a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineInfo {
    /// Zero-based line index.
    pub line_idx: usize,
    /// Byte offset of line start.
    pub start_byte: usize,
    /// Length in bytes (including newline if present).
    pub len_bytes: usize,
    /// Number of grapheme clusters.
    pub grapheme_count: usize,
}
