//! Buffer: core-owned, single-writer text content.
//!
//! See /docs/spec/editor/buffers.md for full requirements.

use kjxlkj_core_types::{BufferId, WindowId};
use ropey::Rope;
use std::path::PathBuf;

use crate::grapheme::{grapheme_count, grapheme_to_byte_offset};

/// Character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
}

/// A text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: BufferId,
    pub name: String,
    pub path: Option<PathBuf>,
    pub content: Rope,
    pub modified: bool,
    pub version: u64,
    pub encoding: Encoding,
    pub line_ending: LineEnding,
    pub readonly: bool,
}

impl Buffer {
    /// Create a new empty scratch buffer.
    pub fn new_scratch(id: BufferId) -> Self {
        Self {
            id,
            name: "[No Name]".to_string(),
            path: None,
            content: Rope::new(),
            modified: false,
            version: 0,
            encoding: Encoding::Utf8,
            line_ending: LineEnding::Lf,
            readonly: false,
        }
    }

    /// Create a buffer from file contents.
    pub fn from_text(id: BufferId, name: &str, text: &str) -> Self {
        let line_ending = if text.contains("\r\n") {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        Self {
            id,
            name: name.to_string(),
            path: None,
            content: Rope::from_str(text),
            modified: false,
            version: 0,
            encoding: Encoding::Utf8,
            line_ending,
            readonly: false,
        }
    }

    /// Insert text at the given line and grapheme offset.
    pub fn insert(
        &mut self,
        line: usize,
        grapheme_offset: usize,
        text: &str,
    ) -> Result<(), &'static str> {
        if self.readonly {
            return Err("buffer is readonly");
        }
        if line >= self.content.len_lines() {
            return Err("line out of bounds");
        }
        let line_slice = self.content.line(line);
        let byte_in_line =
            grapheme_to_byte_offset(line_slice, grapheme_offset)
                .ok_or("grapheme offset out of bounds")?;
        let line_byte_start = self.content.line_to_byte(line);
        let byte_offset = line_byte_start + byte_in_line;
        self.content.insert(byte_offset, text);
        self.version += 1;
        self.modified = true;
        Ok(())
    }

    /// Delete a range from (start_line, start_grapheme) to
    /// (end_line, end_grapheme).
    pub fn delete(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
    ) -> Result<(), &'static str> {
        if self.readonly {
            return Err("buffer is readonly");
        }
        let start_byte = self.grapheme_to_absolute_byte(
            start_line,
            start_grapheme,
        )?;
        let end_byte =
            self.grapheme_to_absolute_byte(end_line, end_grapheme)?;
        if start_byte >= end_byte {
            return Err("invalid range");
        }
        self.content.remove(start_byte..end_byte);
        self.version += 1;
        self.modified = true;
        Ok(())
    }

    /// Number of lines in the buffer.
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get line content as a string (0-based).
    pub fn line(&self, idx: usize) -> Option<String> {
        if idx >= self.content.len_lines() {
            return None;
        }
        Some(self.content.line(idx).to_string())
    }

    /// Count graphemes on a line.
    pub fn line_grapheme_count(&self, line: usize) -> usize {
        if line >= self.content.len_lines() {
            return 0;
        }
        grapheme_count(self.content.line(line))
    }

    /// Convert line + grapheme offset to absolute byte position.
    fn grapheme_to_absolute_byte(
        &self,
        line: usize,
        grapheme_offset: usize,
    ) -> Result<usize, &'static str> {
        if line >= self.content.len_lines() {
            return Err("line out of bounds");
        }
        let line_slice = self.content.line(line);
        let byte_in_line =
            grapheme_to_byte_offset(line_slice, grapheme_offset)
                .ok_or("grapheme offset out of bounds")?;
        Ok(self.content.line_to_byte(line) + byte_in_line)
    }

    /// Clone rope for snapshot (cheap via structural sharing).
    pub fn snapshot_rope(&self) -> Rope {
        self.content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratch_buffer_defaults() {
        let b = Buffer::new_scratch(BufferId(0));
        assert_eq!(b.name, "[No Name]");
        assert!(!b.modified);
        assert_eq!(b.version, 0);
        assert_eq!(b.line_count(), 1);
    }

    #[test]
    fn insert_and_version() {
        let mut b = Buffer::from_text(BufferId(0), "test", "hello");
        assert_eq!(b.version, 0);
        b.insert(0, 5, " world").unwrap();
        assert_eq!(b.version, 1);
        assert!(b.modified);
        assert_eq!(b.line(0).unwrap(), "hello world");
    }

    #[test]
    fn delete_basic() {
        let mut b =
            Buffer::from_text(BufferId(0), "test", "hello world");
        b.delete(0, 5, 0, 11).unwrap();
        assert_eq!(b.line(0).unwrap(), "hello");
    }

    #[test]
    fn readonly_blocks_edit() {
        let mut b = Buffer::new_scratch(BufferId(0));
        b.readonly = true;
        assert!(b.insert(0, 0, "x").is_err());
    }
}
