/// Streaming file IO to avoid full-file copies.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StreamState {
    Idle,
    Reading { bytes_read: u64, total: Option<u64> },
    Complete,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReadChunk {
    pub(crate) data: Vec<u8>,
    pub(crate) offset: u64,
    pub(crate) is_last: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StreamConfig {
    pub(crate) chunk_size: usize,
    pub(crate) max_file_size: u64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: 64 * 1024,          // 64KB
            max_file_size: 1024 * 1024 * 1024, // 1GB
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StreamReader {
    pub(crate) config: StreamConfig,
    pub(crate) state: StreamState,
    pub(crate) chunks_read: usize,
}

impl StreamReader {
    pub(crate) fn new(config: StreamConfig) -> Self {
        Self {
            config,
            state: StreamState::Idle,
            chunks_read: 0,
        }
    }

    pub(crate) fn feed_chunk(&mut self, chunk: ReadChunk) {
        let bytes_read = chunk.offset + chunk.data.len() as u64;
        self.chunks_read += 1;

        if chunk.is_last {
            self.state = StreamState::Complete;
        } else {
            self.state = StreamState::Reading {
                bytes_read,
                total: None,
            };
        }
    }

    pub(crate) fn progress(&self) -> f64 {
        match &self.state {
            StreamState::Idle => 0.0,
            StreamState::Reading { bytes_read, total } => {
                if let Some(t) = total {
                    if *t == 0 { 1.0 } else { *bytes_read as f64 / *t as f64 }
                } else {
                    0.0
                }
            }
            StreamState::Complete => 1.0,
            StreamState::Error(_) => 0.0,
        }
    }

    pub(crate) fn is_complete(&self) -> bool {
        matches!(self.state, StreamState::Complete)
    }

    pub(crate) fn state(&self) -> &StreamState {
        &self.state
    }
}

pub(crate) fn validate_file_size(size: u64, max: u64) -> bool {
    size <= max
}

pub(crate) fn estimate_line_count(bytes: u64) -> usize {
    (bytes / 40) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let cfg = StreamConfig::default();
        assert_eq!(cfg.chunk_size, 64 * 1024);
        assert_eq!(cfg.max_file_size, 1024 * 1024 * 1024);
    }

    #[test]
    fn new_reader() {
        let reader = StreamReader::new(StreamConfig::default());
        assert_eq!(reader.state, StreamState::Idle);
        assert_eq!(reader.chunks_read, 0);
    }

    #[test]
    fn feed_single_chunk() {
        let mut reader = StreamReader::new(StreamConfig::default());
        reader.feed_chunk(ReadChunk { data: vec![0; 100], offset: 0, is_last: true });
        assert!(reader.is_complete());
        assert_eq!(reader.chunks_read, 1);
    }

    #[test]
    fn feed_multiple_chunks() {
        let mut reader = StreamReader::new(StreamConfig::default());
        reader.feed_chunk(ReadChunk { data: vec![0; 100], offset: 0, is_last: false });
        assert!(!reader.is_complete());
        reader.feed_chunk(ReadChunk { data: vec![0; 100], offset: 100, is_last: false });
        assert!(!reader.is_complete());
        reader.feed_chunk(ReadChunk { data: vec![0; 50], offset: 200, is_last: true });
        assert!(reader.is_complete());
        assert_eq!(reader.chunks_read, 3);
    }

    #[test]
    fn progress_tracking() {
        let mut reader = StreamReader::new(StreamConfig::default());
        assert!((reader.progress() - 0.0).abs() < f64::EPSILON);
        reader.feed_chunk(ReadChunk { data: vec![0; 100], offset: 0, is_last: true });
        assert!((reader.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn complete_state() {
        let mut reader = StreamReader::new(StreamConfig::default());
        reader.feed_chunk(ReadChunk { data: vec![0; 10], offset: 0, is_last: true });
        assert!(matches!(reader.state(), StreamState::Complete));
    }

    #[test]
    fn validate_size() {
        assert!(validate_file_size(100, 200));
        assert!(validate_file_size(200, 200));
        assert!(!validate_file_size(201, 200));
    }

    #[test]
    fn estimate_lines() {
        assert_eq!(estimate_line_count(400), 10);
        assert_eq!(estimate_line_count(0), 0);
        assert_eq!(estimate_line_count(80), 2);
    }
}
