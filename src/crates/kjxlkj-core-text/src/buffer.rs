//! TextBuffer: core rope-based text storage.

use kjxlkj_core_types::{BufferId, BufferVersion, Position};
use ropey::Rope;

/// The core text storage backed by a rope data structure.
pub struct TextBuffer {
    id: BufferId,
    rope: Rope,
    version: BufferVersion,
    name: String,
    path: Option<String>,
    modified: bool,
    readonly: bool,
}

impl TextBuffer {
    /// Create an empty buffer.
    pub fn new(id: BufferId, name: String) -> Self {
        Self {
            id,
            rope: Rope::new(),
            version: BufferVersion(1),
            name,
            path: None,
            modified: false,
            readonly: false,
        }
    }

    /// Create a buffer pre-filled with text.
    pub fn from_text(id: BufferId, name: String, text: &str) -> Self {
        Self {
            id,
            rope: Rope::from_str(text),
            version: BufferVersion(1),
            name,
            path: None,
            modified: false,
            readonly: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Return line content WITHOUT trailing newline.
    pub fn line(&self, idx: usize) -> Option<String> {
        if idx >= self.rope.len_lines() {
            return None;
        }
        let slice = self.rope.line(idx);
        let mut s = slice.to_string();
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
        Some(s)
    }

    /// Character count of line (excluding trailing newline).
    pub fn line_len(&self, idx: usize) -> usize {
        self.line(idx).map_or(0, |l| l.chars().count())
    }

    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn insert_char(&mut self, pos: Position, ch: char) {
        let idx = self.pos_to_char_idx(pos);
        self.rope.insert_char(idx, ch);
        self.modified = true;
    }

    pub fn insert_text(&mut self, pos: Position, text: &str) {
        let idx = self.pos_to_char_idx(pos);
        self.rope.insert(idx, text);
        self.modified = true;
    }

    /// Delete [start, end) and return the deleted text.
    pub fn delete_range(&mut self, start: Position, end: Position) -> String {
        let s = self.pos_to_char_idx(start);
        let e = self.pos_to_char_idx(end);
        let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
        let hi = hi.min(self.rope.len_chars());
        let deleted: String = self.rope.slice(lo..hi).to_string();
        self.rope.remove(lo..hi);
        self.modified = true;
        deleted
    }

    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    pub fn line_text(&self, line: usize) -> String {
        self.line(line).unwrap_or_default()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn version(&self) -> BufferVersion {
        self.version
    }

    pub fn bump_version(&mut self) {
        self.version = self.version.next();
    }

    pub fn id(&self) -> BufferId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_readonly(&mut self, val: bool) {
        self.readonly = val;
    }

    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Convert a Position (line, col) to a rope char index.
    pub fn pos_to_char_idx(&self, pos: Position) -> usize {
        let line = pos.line.min(self.rope.len_lines().saturating_sub(1));
        let line_start = self.rope.line_to_char(line);
        let line_len = self.line_len(line);
        line_start + pos.col.min(line_len)
    }

    /// Convert a rope char index back to a Position.
    pub fn char_idx_to_pos(&self, idx: usize) -> Position {
        let idx = idx.min(self.rope.len_chars());
        let line = self.rope.char_to_line(idx);
        let line_start = self.rope.line_to_char(line);
        Position::new(line, idx - line_start)
    }

    /// Clamp a position to valid buffer coordinates.
    pub fn clamp_position(&self, pos: Position) -> Position {
        if self.rope.len_chars() == 0 {
            return Position::ZERO;
        }
        let line = pos.line.min(self.rope.len_lines().saturating_sub(1));
        let max_col = self.line_len(line);
        Position::new(line, pos.col.min(max_col))
    }
}

#[cfg(test)]
#[path = "buffer_tests.rs"]
mod tests;
