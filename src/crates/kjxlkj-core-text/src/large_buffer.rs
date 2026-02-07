//! Large buffer support: loading strategies, line indexing, chunking.

use serde::{Deserialize, Serialize};

/// Threshold: files under 1 MB load fully.
const FULL_THRESHOLD: u64 = 1_024 * 1_024;
/// Threshold: files under 100 MB use chunked loading.
const CHUNKED_THRESHOLD: u64 = 100 * 1_024 * 1_024;

/// Strategy for loading a file into the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadStrategy {
    /// Load entire file into memory at once.
    Full,
    /// Load in fixed-size chunks.
    Chunked,
    /// Stream from disk on demand.
    Streamed,
}

/// Choose the best loading strategy based on file size.
pub fn choose_strategy(file_size: u64) -> LoadStrategy {
    if file_size <= FULL_THRESHOLD {
        LoadStrategy::Full
    } else if file_size <= CHUNKED_THRESHOLD {
        LoadStrategy::Chunked
    } else {
        LoadStrategy::Streamed
    }
}

/// Build an index of byte offsets for each line start.
pub fn build_line_index(content: &str) -> Vec<usize> {
    let mut offsets = vec![0usize];
    for (i, b) in content.bytes().enumerate() {
        if b == b'\n' && i + 1 <= content.len() {
            offsets.push(i + 1);
        }
    }
    offsets
}

/// Compute chunk ranges (offset, length) for a file.
pub fn compute_chunks(file_size: u64, chunk_size: u64) -> Vec<(u64, u64)> {
    if chunk_size == 0 || file_size == 0 {
        return Vec::new();
    }
    let mut chunks = Vec::new();
    let mut offset = 0u64;
    while offset < file_size {
        let len = (file_size - offset).min(chunk_size);
        chunks.push((offset, len));
        offset += len;
    }
    chunks
}

/// Extract lines [start, end) from content using a pre-built line index.
pub fn extract_line_range(content: &str, index: &[usize], start: usize, end: usize) -> String {
    if start >= index.len() || start >= end {
        return String::new();
    }
    let byte_start = index[start];
    let byte_end = if end < index.len() {
        index[end]
    } else {
        content.len()
    };
    content
        .get(byte_start..byte_end)
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_selection() {
        assert_eq!(choose_strategy(500), LoadStrategy::Full);
        assert_eq!(choose_strategy(2 * 1_024 * 1_024), LoadStrategy::Chunked);
        assert_eq!(
            choose_strategy(200 * 1_024 * 1_024),
            LoadStrategy::Streamed
        );
    }

    #[test]
    fn line_index() {
        let text = "abc\ndef\nghi";
        let idx = build_line_index(text);
        assert_eq!(idx, vec![0, 4, 8]);
    }

    #[test]
    fn chunks() {
        let c = compute_chunks(100, 30);
        assert_eq!(c.len(), 4);
        assert_eq!(c[0], (0, 30));
        assert_eq!(c[3], (90, 10));
    }

    #[test]
    fn extract_lines() {
        let text = "line0\nline1\nline2\n";
        let idx = build_line_index(text);
        let extracted = extract_line_range(text, &idx, 1, 2);
        assert_eq!(extracted, "line1\n");
    }
}
