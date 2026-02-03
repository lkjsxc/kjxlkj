//! Rope-based text storage.

use ropey::Rope;

use kjxlkj_core_types::{BufferVersion, LineCol};

/// Rope-based text content with versioning.
#[derive(Debug, Clone)]
pub struct RopeText {
    rope: Rope,
    version: BufferVersion,
}

impl RopeText {
    /// Creates a new empty text.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: BufferVersion::default(),
        }
    }

    /// Creates text from a string.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
            version: BufferVersion::default(),
        }
    }

    /// Returns the current version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Increments the version.
    fn bump_version(&mut self) {
        self.version = self.version.next();
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns the total number of characters.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns true if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Returns the entire text as a string.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Returns a specific line as a string (without trailing newline).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        let line = self.rope.line(line_idx);
        let s = line.to_string();
        Some(s.trim_end_matches('\n').to_string())
    }

    /// Returns the length of a line in characters.
    pub fn line_len(&self, line_idx: usize) -> Option<usize> {
        self.line(line_idx).map(|s| s.chars().count())
    }

    /// Converts a LineCol to a char offset.
    pub fn line_col_to_char(&self, pos: LineCol) -> Option<usize> {
        if pos.line as usize >= self.line_count() {
            return None;
        }
        let line_start = self.rope.line_to_char(pos.line as usize);
        Some(line_start + pos.col as usize)
    }

    /// Converts a char offset to LineCol.
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<LineCol> {
        if char_idx > self.char_count() {
            return None;
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        Some(LineCol::new(line as u32, col as u32))
    }

    /// Inserts a character at the given position.
    pub fn insert_char(&mut self, pos: LineCol, ch: char) -> bool {
        if let Some(char_idx) = self.line_col_to_char(pos) {
            self.rope.insert_char(char_idx, ch);
            self.bump_version();
            true
        } else if pos.line as usize == self.line_count() && pos.col == 0 {
            let idx = self.char_count();
            self.rope.insert_char(idx, ch);
            self.bump_version();
            true
        } else {
            false
        }
    }

    /// Inserts a string at the given position.
    pub fn insert(&mut self, pos: LineCol, s: &str) -> bool {
        if let Some(char_idx) = self.line_col_to_char(pos) {
            self.rope.insert(char_idx, s);
            self.bump_version();
            true
        } else if pos.line as usize == self.line_count() && pos.col == 0 {
            let idx = self.char_count();
            self.rope.insert(idx, s);
            self.bump_version();
            true
        } else {
            false
        }
    }

    /// Deletes a range of characters.
    pub fn delete_range(&mut self, start: usize, end: usize) -> bool {
        if start <= end && end <= self.char_count() {
            self.rope.remove(start..end);
            self.bump_version();
            true
        } else {
            false
        }
    }

    /// Deletes a single character at the given position.
    pub fn delete_char(&mut self, pos: LineCol) -> bool {
        if let Some(char_idx) = self.line_col_to_char(pos) {
            if char_idx < self.char_count() {
                self.rope.remove(char_idx..char_idx + 1);
                self.bump_version();
                return true;
            }
        }
        false
    }

    /// Deletes a line and returns its content.
    pub fn delete_line(&mut self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        let start = self.rope.line_to_char(line_idx);
        let end = if line_idx + 1 < self.line_count() {
            self.rope.line_to_char(line_idx + 1)
        } else {
            self.char_count()
        };
        let content = self.rope.slice(start..end).to_string();
        self.rope.remove(start..end);
        self.bump_version();
        Some(content)
    }

    /// Returns a slice of the text.
    pub fn slice(&self, start: usize, end: usize) -> Option<String> {
        if start <= end && end <= self.char_count() {
            Some(self.rope.slice(start..end).to_string())
        } else {
            None
        }
    }
}

impl Default for RopeText {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let t = RopeText::new();
        assert!(t.is_empty());
        assert_eq!(t.line_count(), 1);
    }

    #[test]
    fn from_string() {
        let t = RopeText::from_str("hello\nworld");
        assert_eq!(t.line_count(), 2);
        assert_eq!(t.line(0), Some("hello".to_string()));
        assert_eq!(t.line(1), Some("world".to_string()));
    }

    #[test]
    fn insert_char() {
        let mut t = RopeText::from_str("hllo");
        t.insert_char(LineCol::new(0, 1), 'e');
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn delete_char() {
        let mut t = RopeText::from_str("helllo");
        t.delete_char(LineCol::new(0, 3));
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn version_increments() {
        let mut t = RopeText::new();
        let v0 = t.version();
        t.insert_char(LineCol::new(0, 0), 'a');
        let v1 = t.version();
        assert!(v1 > v0);
    }
}
