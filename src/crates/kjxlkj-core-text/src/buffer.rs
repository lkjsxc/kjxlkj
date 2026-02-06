//! Core text buffer backed by ropey Rope.

use kjxlkj_core_types::{BufferId, BufferVersion, Position, Range};
use ropey::Rope;

/// A text buffer backed by a rope data structure.
pub struct TextBuffer {
    id: BufferId,
    rope: Rope,
    modified: bool,
    version: BufferVersion,
    name: String,
    file_path: Option<std::path::PathBuf>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            id: BufferId::next(),
            rope: Rope::new(),
            modified: false,
            version: BufferVersion(0),
            name: String::from("[No Name]"),
            file_path: None,
        }
    }

    pub fn from_text(text: &str) -> Self {
        Self {
            id: BufferId::next(),
            rope: Rope::from_str(text),
            modified: false,
            version: BufferVersion(0),
            name: String::from("[No Name]"),
            file_path: None,
        }
    }

    pub fn id(&self) -> BufferId { self.id }
    pub fn version(&self) -> BufferVersion { self.version }
    pub fn is_modified(&self) -> bool { self.modified }
    pub fn name(&self) -> &str { &self.name }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn file_path(&self) -> Option<&std::path::Path> {
        self.file_path.as_deref()
    }

    pub fn set_file_path(&mut self, path: impl Into<std::path::PathBuf>) {
        let p: std::path::PathBuf = path.into();
        if let Some(fname) = p.file_name() {
            self.name = fname.to_string_lossy().into();
        }
        self.file_path = Some(p);
    }

    pub fn set_modified(&mut self, val: bool) { self.modified = val; }
    pub fn rope(&self) -> &Rope { &self.rope }
    pub fn text(&self) -> String { self.rope.to_string() }

    pub fn line_count(&self) -> usize { self.rope.len_lines() }

    pub fn line(&self, idx: usize) -> Option<ropey::RopeSlice<'_>> {
        if idx < self.rope.len_lines() {
            Some(self.rope.line(idx))
        } else {
            None
        }
    }

    /// Characters on a line, excluding trailing newline.
    pub fn line_len(&self, line_idx: usize) -> usize {
        if line_idx >= self.line_count() { return 0; }
        let line = self.rope.line(line_idx);
        let len = line.len_chars();
        if len > 0 && line.char(len - 1) == '\n' {
            len - 1
        } else {
            len
        }
    }

    pub fn char_at(&self, pos: Position) -> Option<char> {
        let idx = self.pos_to_char_idx(pos);
        if idx < self.rope.len_chars() {
            Some(self.rope.char(idx))
        } else {
            None
        }
    }

    pub fn pos_to_char_idx(&self, pos: Position) -> usize {
        if pos.line >= self.line_count() {
            return self.rope.len_chars();
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.line_len(pos.line);
        line_start + pos.col.min(line_len)
    }

    pub fn char_idx_to_pos(&self, idx: usize) -> Position {
        let clamped = idx.min(self.rope.len_chars());
        let line = self.rope.char_to_line(clamped);
        let col = clamped - self.rope.line_to_char(line);
        Position::new(line, col)
    }

    pub fn insert_char(&mut self, pos: Position, ch: char) {
        let idx = self.pos_to_char_idx(pos);
        self.rope.insert_char(idx, ch);
        self.modified = true;
        self.version = self.version.next();
    }

    pub fn insert_text(&mut self, pos: Position, text: &str) {
        let idx = self.pos_to_char_idx(pos);
        self.rope.insert(idx, text);
        self.modified = true;
        self.version = self.version.next();
    }

    pub fn delete_range(&mut self, range: Range) -> String {
        let s = self.pos_to_char_idx(range.start);
        let e = self.pos_to_char_idx(range.end);
        let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
        if lo == hi { return String::new(); }
        let hi = hi.min(self.rope.len_chars());
        let deleted: String = self.rope.slice(lo..hi).to_string();
        self.rope.remove(lo..hi);
        self.modified = true;
        self.version = self.version.next();
        deleted
    }

    /// Delete a single line (including its newline).
    pub fn delete_line(&mut self, line_idx: usize) -> String {
        if line_idx >= self.line_count() { return String::new(); }
        let start = self.rope.line_to_char(line_idx);
        let end = if line_idx + 1 < self.line_count() {
            self.rope.line_to_char(line_idx + 1)
        } else {
            self.rope.len_chars()
        };
        let deleted: String = self.rope.slice(start..end).to_string();
        self.rope.remove(start..end);
        self.modified = true;
        self.version = self.version.next();
        deleted
    }

    pub fn clamp_position(&self, pos: Position) -> Position {
        let max_line = self.line_count().saturating_sub(1);
        let line = pos.line.min(max_line);
        let max_col = self.line_len(line).saturating_sub(1).max(0);
        Position::new(line, pos.col.min(max_col))
    }

    /// Clamp for insert mode (allows cursor after last char).
    pub fn clamp_position_insert(&self, pos: Position) -> Position {
        let max_line = self.line_count().saturating_sub(1);
        let line = pos.line.min(max_line);
        let max_col = self.line_len(line);
        Position::new(line, pos.col.min(max_col))
    }

    pub fn line_to_string(&self, line_idx: usize) -> String {
        self.line(line_idx)
            .map(|s| {
                let t = s.to_string();
                t.trim_end_matches('\n').to_string()
            })
            .unwrap_or_default()
    }

    /// Extract text between two positions.
    pub fn text_in_range(&self, start: Position, end: Position) -> String {
        let s = self.pos_to_char_idx(start);
        let e = self.pos_to_char_idx(end);
        let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
        let hi = hi.min(self.rope.len_chars());
        self.rope.slice(lo..hi).to_string()
    }

    /// Delete between two positions (convenience wrapper).
    pub fn delete_between(&mut self, start: Position, end: Position) -> String {
        self.delete_range(Range::new(start, end))
    }
}

impl Default for TextBuffer {
    fn default() -> Self { Self::new() }
}
