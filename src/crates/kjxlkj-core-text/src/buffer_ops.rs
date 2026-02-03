//! Buffer operations - insert, delete, line manipulation.

use kjxlkj_core_types::{EditorError, EditorResult, Position, Range};

use super::buffer::TextBuffer;

impl TextBuffer {
    /// Convert a position to a char offset (for rope operations).
    pub fn pos_to_offset(&self, pos: Position) -> EditorResult<usize> {
        let line_idx = pos.line as usize;
        if line_idx >= self.line_count() {
            return Err(EditorError::InvalidPosition {
                line: pos.line,
                col: pos.col,
            });
        }

        let line_start = self.rope().line_to_char(line_idx);
        let line_text = self.line(line_idx).unwrap_or_default();
        let char_offset = super::grapheme_to_char_offset(&line_text, pos.col as usize);

        Ok(line_start + char_offset)
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) -> EditorResult<()> {
        let offset = self.pos_to_offset(pos)?;
        self.rope_mut().insert(offset, text);
        self.mark_modified();
        Ok(())
    }

    /// Delete a range of text.
    pub fn delete(&mut self, range: Range) -> EditorResult<String> {
        let norm = range.normalized();
        let start = self.pos_to_offset(norm.start)?;
        let end = self.pos_to_offset(norm.end)?;

        if end > start {
            let deleted: String = self.rope().slice(start..end).to_string();
            self.rope_mut().remove(start..end);
            self.mark_modified();
            Ok(deleted)
        } else {
            Ok(String::new())
        }
    }

    /// Delete a single line.
    pub fn delete_line(&mut self, line_idx: usize) -> EditorResult<String> {
        if line_idx >= self.line_count() {
            return Err(EditorError::InvalidPosition {
                line: line_idx as u32,
                col: 0,
            });
        }

        let start = self.rope().line_to_char(line_idx);
        let end = if line_idx + 1 < self.line_count() {
            self.rope().line_to_char(line_idx + 1)
        } else {
            self.rope().len_chars()
        };

        let deleted: String = self.rope().slice(start..end).to_string();
        self.rope_mut().remove(start..end);
        self.mark_modified();
        Ok(deleted)
    }

    /// Insert a new line at the given line index.
    pub fn insert_line(&mut self, line_idx: usize, text: &str) -> EditorResult<()> {
        let offset = if line_idx >= self.line_count() {
            self.rope().len_chars()
        } else {
            self.rope().line_to_char(line_idx)
        };

        let line_with_ending = if text.ends_with('\n') {
            text.to_string()
        } else {
            format!("{}{}", text, self.line_ending().as_str())
        };

        self.rope_mut().insert(offset, &line_with_ending);
        self.mark_modified();
        Ok(())
    }
}
