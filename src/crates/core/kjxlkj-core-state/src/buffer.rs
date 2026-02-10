//! Buffer state management.

use crate::word_nav;
use kjxlkj_core_text::{empty_rope, line_content, line_grapheme_count, Rope};
use kjxlkj_core_types::{BufferId, BufferMeta, BufferVersion, CursorPosition};
use kjxlkj_core_undo::UndoHistory;
use std::path::PathBuf;

/// Buffer state.
#[derive(Debug)]
pub struct Buffer {
    /// Buffer metadata.
    pub meta: BufferMeta,
    /// Rope content.
    pub content: Rope,
    /// Undo history.
    pub undo: UndoHistory,
}

impl Buffer {
    /// Create a new scratch buffer.
    pub fn scratch(id: BufferId) -> Self {
        Self {
            meta: BufferMeta::scratch(id),
            content: empty_rope(),
            undo: UndoHistory::new(),
        }
    }

    /// Create a buffer from file path.
    pub fn from_path(id: BufferId, path: PathBuf, content: String) -> Self {
        Self {
            meta: BufferMeta::from_path(id, path),
            content: Rope::from_str(&content),
            undo: UndoHistory::new(),
        }
    }

    /// Get line count.
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get line content.
    pub fn line(&self, idx: usize) -> String {
        line_content(&self.content, idx)
    }

    /// Get grapheme count for a line.
    pub fn line_grapheme_count(&self, idx: usize) -> usize {
        line_grapheme_count(&self.content, idx)
    }

    /// Insert text at position.
    pub fn insert(&mut self, pos: CursorPosition, text: &str) {
        let byte_offset =
            kjxlkj_core_text::position_to_byte(&self.content, pos.line, pos.grapheme);
        self.content.insert(byte_offset, text);
        self.meta.version.increment();
        self.meta.modified = true;
    }

    /// Delete range.
    pub fn delete(&mut self, start: CursorPosition, end: CursorPosition) {
        kjxlkj_core_text::delete_range(
            &mut self.content,
            start.line,
            start.grapheme,
            end.line,
            end.grapheme,
        );
        self.meta.version.increment();
        self.meta.modified = true;
    }

    /// Mark as saved.
    pub fn mark_saved(&mut self) {
        self.meta.modified = false;
    }

    /// Get buffer version.
    pub fn version(&self) -> BufferVersion {
        self.meta.version
    }

    /// Get first non-blank grapheme on a line.
    pub fn first_non_blank(&self, line: usize) -> usize {
        let content = self.line(line);
        content
            .char_indices()
            .find(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| kjxlkj_core_text::byte_to_grapheme_offset(&content, i))
            .unwrap_or(0)
    }

    /// Find next word start.
    pub fn next_word_start(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::next_word_start(&self.content, pos, false)
    }

    /// Find previous word start.
    pub fn prev_word_start(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::prev_word_start(&self.content, pos, false)
    }

    /// Find next word end.
    pub fn next_word_end(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::next_word_end(&self.content, pos, false)
    }

    /// Find previous word end.
    pub fn prev_word_end(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::prev_word_end(&self.content, pos, false)
    }

    /// Find next big word start.
    pub fn next_big_word_start(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::next_word_start(&self.content, pos, true)
    }

    /// Find previous big word start.
    pub fn prev_big_word_start(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::prev_word_start(&self.content, pos, true)
    }

    /// Find next big word end.
    pub fn next_big_word_end(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::next_word_end(&self.content, pos, true)
    }

    /// Find previous big word end.
    pub fn prev_big_word_end(&self, pos: CursorPosition) -> CursorPosition {
        word_nav::prev_word_end(&self.content, pos, true)
    }
}
