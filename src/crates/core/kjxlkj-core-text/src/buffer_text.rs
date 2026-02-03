use kjxlkj_core_types::{CursorPos, TextRange};
use ropey::Rope;

use crate::TextError;

#[derive(Clone, Debug, Default)]
pub struct BufferText {
    rope: Rope,
}

impl BufferText {
    pub fn from_text(s: &str) -> Self {
        Self { rope: Rope::from_str(s) }
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, line_idx: usize) -> Option<String> {
        self.rope.get_line(line_idx).map(|l| l.to_string())
    }

    pub fn line_len_chars(&self, line_idx: usize) -> Option<usize> {
        self.rope.get_line(line_idx).map(|l| l.len_chars())
    }

    pub fn line_len_chars_no_nl(&self, line_idx: usize) -> Option<usize> {
        let line = self.rope.get_line(line_idx)?;
        let len = line.len_chars();
        if len == 0 {
            return Some(0);
        }
        match line.chars().last() {
            Some('\n') => Some(len.saturating_sub(1)),
            _ => Some(len),
        }
    }

    pub fn clamp_cursor(&self, cursor: CursorPos) -> CursorPos {
        let last_line = self.line_count().saturating_sub(1);
        let line = cursor.line.min(last_line);
        let max_col = self.line_len_chars_no_nl(line).unwrap_or(0);
        let col = cursor.col.min(max_col);
        CursorPos { line, col }
    }

    pub fn cursor_to_char(&self, cursor: CursorPos) -> Result<usize, TextError> {
        let cursor = self.clamp_cursor(cursor);
        if cursor.line >= self.line_count() {
            return Err(TextError::IndexOutOfBounds);
        }
        let line_start = self.rope.line_to_char(cursor.line);
        Ok(line_start.saturating_add(cursor.col))
    }

    pub fn char_to_cursor(&self, char_idx: usize) -> Result<CursorPos, TextError> {
        if char_idx > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        Ok(self.clamp_cursor(CursorPos { line, col: char_idx.saturating_sub(line_start) }))
    }

    pub fn char_at(&self, char_idx: usize) -> Option<char> {
        self.rope.get_char(char_idx)
    }

    pub fn slice(&self, range: TextRange) -> Result<String, TextError> {
        if range.end > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        Ok(self.rope.slice(range.start..range.end).to_string())
    }

    pub fn line_char_range(&self, line_idx: usize) -> Option<TextRange> {
        if line_idx >= self.line_count() {
            return None;
        }
        let start = self.rope.line_to_char(line_idx);
        let line_len = self.rope.get_line(line_idx)?.len_chars();
        Some(TextRange { start, end: start.saturating_add(line_len) })
    }

    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), TextError> {
        if char_idx > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        self.rope.insert(char_idx, text);
        Ok(())
    }

    pub fn remove(&mut self, range: TextRange) -> Result<String, TextError> {
        if range.end > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        let removed = self.rope.slice(range.start..range.end).to_string();
        self.rope.remove(range.start..range.end);
        Ok(removed)
    }
}

impl std::fmt::Display for BufferText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl From<&str> for BufferText {
    fn from(value: &str) -> Self {
        Self::from_text(value)
    }
}

impl From<String> for BufferText {
    fn from(value: String) -> Self {
        Self::from_text(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_remove_roundtrip() {
        let mut t = BufferText::from_text("abc");
        t.insert(1, "Z").unwrap();
        assert_eq!(t.to_string(), "aZbc");

        let removed = t.remove(TextRange { start: 1, end: 2 }).unwrap();
        assert_eq!(removed, "Z");
        assert_eq!(t.to_string(), "abc");
    }

    #[test]
    fn cursor_to_char_clamps_column() {
        let t = BufferText::from_text("ab\nc");
        let idx = t.cursor_to_char(CursorPos { line: 0, col: 99 }).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn line_char_range_includes_newline_when_present() {
        let t = BufferText::from_text("ab\nc");
        let r0 = t.line_char_range(0).unwrap();
        assert_eq!(t.rope.slice(r0.start..r0.end).to_string(), "ab\n");
    }

    #[test]
    fn char_to_cursor_roundtrips_cursor_to_char() {
        let t = BufferText::from_text("ab\nc");
        let c = CursorPos { line: 1, col: 1 };
        let idx = t.cursor_to_char(c).unwrap();
        assert_eq!(t.char_to_cursor(idx).unwrap(), c);
    }
}
