//! Streaming IO types for large file reading.

use serde::{Deserialize, Serialize};

/// State of a streaming read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum StreamState {
    #[default]
    Idle,
    Reading,
    Complete,
    Error,
}


/// A chunk of data from a streaming read.
#[derive(Debug, Clone)]
pub struct ReadChunk {
    pub data: Vec<u8>,
    pub offset: u64,
    pub is_last: bool,
}

/// Configuration for streaming IO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Chunk size in bytes (default 64 KB).
    pub chunk_size: u64,
    /// Maximum file size in bytes (default 1 GB).
    pub max_file_size: u64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: 64 * 1024,
            max_file_size: 1024 * 1024 * 1024,
        }
    }
}

/// Streaming reader state tracker.
#[derive(Debug, Clone)]
pub struct StreamReader {
    pub config: StreamConfig,
    pub state: StreamState,
    pub chunks: Vec<ReadChunk>,
    pub total_bytes: u64,
    pub bytes_read: u64,
}

impl StreamReader {
    pub fn new(config: StreamConfig) -> Self {
        Self {
            config,
            state: StreamState::Idle,
            chunks: Vec::new(),
            total_bytes: 0,
            bytes_read: 0,
        }
    }

    /// Begin reading a file of the given size.
    pub fn begin(&mut self, size: u64) {
        self.total_bytes = size;
        self.bytes_read = 0;
        self.chunks.clear();
        self.state = StreamState::Reading;
    }

    /// Accept a chunk.
    pub fn push_chunk(&mut self, chunk: ReadChunk) {
        self.bytes_read += chunk.data.len() as u64;
        if chunk.is_last {
            self.state = StreamState::Complete;
        }
        self.chunks.push(chunk);
    }

    /// Progress as a fraction 0.0..=1.0.
    pub fn progress(&self) -> f64 {
        if self.total_bytes == 0 {
            return 1.0;
        }
        self.bytes_read as f64 / self.total_bytes as f64
    }
}

/// Check if a file size is within the configured limits.
pub fn validate_file_size(size: u64, config: &StreamConfig) -> bool {
    size <= config.max_file_size
}

/// Rough estimate of line count from byte count (assume ~60 bytes/line).
pub fn estimate_line_count(byte_count: u64) -> u64 {
    byte_count / 60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_reader_progress() {
        let mut sr = StreamReader::new(StreamConfig::default());
        sr.begin(128);
        assert_eq!(sr.state, StreamState::Reading);
        sr.push_chunk(ReadChunk {
            data: vec![0u8; 64],
            offset: 0,
            is_last: false,
        });
        assert!((sr.progress() - 0.5).abs() < 0.01);
        sr.push_chunk(ReadChunk {
            data: vec![0u8; 64],
            offset: 64,
            is_last: true,
        });
        assert_eq!(sr.state, StreamState::Complete);
    }

    #[test]
    fn validate_size() {
        let cfg = StreamConfig::default();
        assert!(validate_file_size(1000, &cfg));
        assert!(!validate_file_size(2 * 1024 * 1024 * 1024, &cfg));
    }

    #[test]
    fn estimate_lines() {
        assert_eq!(estimate_line_count(600), 10);
    }
}
