//! Large buffer strategies — streaming load and chunked operations.
//!
//! Provides utilities for handling files too large for naive full-buffer
//! operations, using chunk-based processing and streaming reads.

/// Chunk metadata describing a segment of a large file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub offset: usize,
    pub length: usize,
    pub line_start: usize,
    pub line_count: usize,
}

/// Strategy for handling a file based on its size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStrategy {
    /// Small file — load entirely into memory.
    Full,
    /// Medium file — load in chunks, index eagerly.
    Chunked,
    /// Very large file — memory-map, index lazily.
    Streamed,
}

/// Thresholds for load strategy decisions.
pub const FULL_THRESHOLD: usize = 1_048_576;      // 1 MB
pub const CHUNKED_THRESHOLD: usize = 104_857_600; // 100 MB
pub const DEFAULT_CHUNK_SIZE: usize = 65_536;      // 64 KB

/// Determine the load strategy for a given file size.
pub fn choose_strategy(file_size: usize) -> LoadStrategy {
    if file_size <= FULL_THRESHOLD { LoadStrategy::Full }
    else if file_size <= CHUNKED_THRESHOLD { LoadStrategy::Chunked }
    else { LoadStrategy::Streamed }
}

/// Build a line index from text, returning byte offsets of each line start.
pub fn build_line_index(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' && i + 1 <= text.len() {
            offsets.push(i + 1);
        }
    }
    offsets
}

/// Compute chunk boundaries for a file of `total_size` bytes.
pub fn compute_chunks(total_size: usize, chunk_size: usize) -> Vec<Chunk> {
    if total_size == 0 { return vec![]; }
    let cs = if chunk_size == 0 { DEFAULT_CHUNK_SIZE } else { chunk_size };
    let mut chunks = Vec::new();
    let mut offset = 0;
    while offset < total_size {
        let length = cs.min(total_size - offset);
        chunks.push(Chunk { offset, length, line_start: 0, line_count: 0 });
        offset += length;
    }
    chunks
}

/// Annotate chunks with line information from a line index.
pub fn annotate_chunks_with_lines(chunks: &mut [Chunk], line_offsets: &[usize]) {
    for chunk in chunks.iter_mut() {
        let start = chunk.offset;
        let end = start + chunk.length;
        let first = line_offsets.partition_point(|&o| o < start);
        let last = line_offsets.partition_point(|&o| o < end);
        chunk.line_start = first;
        chunk.line_count = last - first;
    }
}

/// Extract text for a specific line range from the full text using a line index.
pub fn extract_line_range<'a>(text: &'a str, line_offsets: &[usize], start_line: usize, count: usize) -> &'a str {
    if start_line >= line_offsets.len() { return ""; }
    let byte_start = line_offsets[start_line];
    let end_line = (start_line + count).min(line_offsets.len());
    let byte_end = if end_line < line_offsets.len() {
        line_offsets[end_line]
    } else {
        text.len()
    };
    &text[byte_start..byte_end]
}

/// Check if a file size exceeds the chunked threshold.
pub fn is_very_large(file_size: usize) -> bool {
    file_size > CHUNKED_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_full() {
        assert_eq!(choose_strategy(1000), LoadStrategy::Full);
        assert_eq!(choose_strategy(FULL_THRESHOLD), LoadStrategy::Full);
    }

    #[test]
    fn strategy_chunked() {
        assert_eq!(choose_strategy(FULL_THRESHOLD + 1), LoadStrategy::Chunked);
        assert_eq!(choose_strategy(CHUNKED_THRESHOLD), LoadStrategy::Chunked);
    }

    #[test]
    fn strategy_streamed() {
        assert_eq!(choose_strategy(CHUNKED_THRESHOLD + 1), LoadStrategy::Streamed);
    }

    #[test]
    fn line_index_building() {
        let idx = build_line_index("hello\nworld\n");
        assert_eq!(idx, vec![0, 6, 12]);
    }

    #[test]
    fn chunk_computation() {
        let chunks = compute_chunks(150, 64);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].offset, 0);
        assert_eq!(chunks[0].length, 64);
        assert_eq!(chunks[2].length, 22);
    }

    #[test]
    fn annotate_chunks() {
        let text = "aaa\nbbb\nccc\nddd\n";
        let idx = build_line_index(text);
        let mut chunks = compute_chunks(text.len(), 8);
        annotate_chunks_with_lines(&mut chunks, &idx);
        assert_eq!(chunks[0].line_start, 0);
        assert!(chunks[0].line_count >= 1);
    }

    #[test]
    fn extract_lines() {
        let text = "line0\nline1\nline2\nline3\n";
        let idx = build_line_index(text);
        let extracted = extract_line_range(text, &idx, 1, 2);
        assert_eq!(extracted, "line1\nline2\n");
    }

    #[test]
    fn is_very_large_check() {
        assert!(!is_very_large(1000));
        assert!(is_very_large(CHUNKED_THRESHOLD + 1));
    }

    #[test]
    fn empty_file_chunks() {
        let chunks = compute_chunks(0, 64);
        assert!(chunks.is_empty());
    }
}
