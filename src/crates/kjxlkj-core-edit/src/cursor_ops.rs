//! Cursor movement operations.

use kjxlkj_core_types::Cursor;

/// Trait for cursor movement operations.
pub trait CursorOps {
    /// Returns the line count.
    fn line_count(&self) -> usize;
    /// Returns the length of a line.
    fn line_len(&self, line: usize) -> Option<usize>;
    /// Returns the content of a line.
    fn line_content(&self, line: usize) -> Option<String>;
    /// Returns the cursor.
    fn cursor(&self) -> Cursor;
    /// Returns a mutable reference to the cursor.
    fn cursor_mut(&mut self) -> &mut Cursor;

    /// Moves the cursor left.
    fn move_left(&mut self) {
        let cursor = self.cursor_mut();
        if cursor.position.col > 0 {
            cursor.position.col -= 1;
        }
        cursor.clear_preferred_col();
    }

    /// Moves the cursor right.
    fn move_right(&mut self) {
        let cursor = self.cursor();
        let line_len = self
            .line_len(cursor.position.line as usize)
            .unwrap_or(0);
        let max_col = if line_len > 0 { line_len - 1 } else { 0 };
        let cursor = self.cursor_mut();
        if (cursor.position.col as usize) < max_col {
            cursor.position.col += 1;
        }
        cursor.clear_preferred_col();
    }

    /// Moves the cursor up.
    fn move_up(&mut self) {
        let cur = self.cursor();
        if cur.position.line > 0 {
            let pref = cur.preferred_col.unwrap_or(cur.position.col);
            let new_line = cur.position.line - 1;
            let new_len = self.line_len(new_line as usize).unwrap_or(0);
            let max_col = if new_len > 0 { new_len - 1 } else { 0 };
            let cursor = self.cursor_mut();
            cursor.position.line = new_line;
            cursor.position.col = pref.min(max_col as u32);
            cursor.set_preferred_col(pref);
        }
    }

    /// Moves the cursor down.
    fn move_down(&mut self) {
        let line_count = self.line_count();
        let cur = self.cursor();
        if (cur.position.line as usize) < line_count.saturating_sub(1) {
            let pref = cur.preferred_col.unwrap_or(cur.position.col);
            let new_line = cur.position.line + 1;
            let new_len = self.line_len(new_line as usize).unwrap_or(0);
            let max_col = if new_len > 0 { new_len - 1 } else { 0 };
            let cursor = self.cursor_mut();
            cursor.position.line = new_line;
            cursor.position.col = pref.min(max_col as u32);
            cursor.set_preferred_col(pref);
        }
    }

    /// Moves the cursor to the start of the line (column 0).
    fn move_line_start(&mut self) {
        self.cursor_mut().position.col = 0;
        self.cursor_mut().clear_preferred_col();
    }

    /// Moves the cursor to the end of the line.
    fn move_line_end(&mut self) {
        let cursor = self.cursor();
        let line_len = self
            .line_len(cursor.position.line as usize)
            .unwrap_or(0);
        let max_col = if line_len > 0 { line_len - 1 } else { 0 };
        self.cursor_mut().position.col = max_col as u32;
        self.cursor_mut().clear_preferred_col();
    }

    /// Moves the cursor to the first non-blank character on the line (^).
    fn move_first_non_blank(&mut self) {
        let cursor = self.cursor();
        if let Some(line) = self.line_content(cursor.position.line as usize) {
            let first_non_blank = line
                .char_indices()
                .find(|(_, c)| !c.is_whitespace())
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.cursor_mut().position.col = first_non_blank as u32;
            self.cursor_mut().clear_preferred_col();
        }
    }

    /// Moves the cursor to the start of the next word (w).
    fn move_word_forward(&mut self) {
        let cursor = self.cursor();
        let line = cursor.position.line as usize;
        let col = cursor.position.col as usize;
        let line_count = self.line_count();

        if let Some(content) = self.line_content(line) {
            let chars: Vec<char> = content.chars().collect();
            
            // Find the next word start on current line
            if let Some(new_col) = find_word_start_forward(&chars, col) {
                self.cursor_mut().position.col = new_col as u32;
                self.cursor_mut().clear_preferred_col();
                return;
            }
        }

        // Move to next line
        if line + 1 < line_count {
            self.cursor_mut().position.line = (line + 1) as u32;
            self.cursor_mut().position.col = 0;
            self.move_first_non_blank();
        }
    }

    /// Moves the cursor to the start of the previous word (b).
    fn move_word_backward(&mut self) {
        let cursor = self.cursor();
        let line = cursor.position.line as usize;
        let col = cursor.position.col as usize;

        if let Some(content) = self.line_content(line) {
            let chars: Vec<char> = content.chars().collect();
            
            // Find the previous word start on current line
            if let Some(new_col) = find_word_start_backward(&chars, col) {
                self.cursor_mut().position.col = new_col as u32;
                self.cursor_mut().clear_preferred_col();
                return;
            }
        }

        // Move to previous line end
        if line > 0 {
            self.cursor_mut().position.line = (line - 1) as u32;
            self.move_line_end();
        }
    }

    /// Moves the cursor to the end of the current/next word (e).
    fn move_word_end(&mut self) {
        let cursor = self.cursor();
        let line = cursor.position.line as usize;
        let col = cursor.position.col as usize;
        let line_count = self.line_count();

        if let Some(content) = self.line_content(line) {
            let chars: Vec<char> = content.chars().collect();
            
            // Find the word end on current line
            if let Some(new_col) = find_word_end_forward(&chars, col) {
                self.cursor_mut().position.col = new_col as u32;
                self.cursor_mut().clear_preferred_col();
                return;
            }
        }

        // Move to next line
        if line + 1 < line_count {
            self.cursor_mut().position.line = (line + 1) as u32;
            self.cursor_mut().position.col = 0;
            self.move_word_end();
        }
    }

    /// Moves the cursor to the start of the file (gg).
    fn move_file_start(&mut self) {
        self.cursor_mut().position.line = 0;
        self.cursor_mut().position.col = 0;
        self.move_first_non_blank();
    }

    /// Moves the cursor to the end of the file (G).
    fn move_file_end(&mut self) {
        let line_count = self.line_count();
        if line_count > 0 {
            self.cursor_mut().position.line = (line_count - 1) as u32;
            self.cursor_mut().position.col = 0;
            self.move_first_non_blank();
        }
    }
}

/// Determines if a character is a "word" character (alphanumeric or underscore).
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Find the start of the next word from the given position.
fn find_word_start_forward(chars: &[char], start: usize) -> Option<usize> {
    if chars.is_empty() || start >= chars.len() {
        return None;
    }

    let mut i = start;
    let start_is_word = is_word_char(chars[i]);
    let start_is_space = chars[i].is_whitespace();

    // Skip current word/non-word sequence
    if start_is_space {
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
    } else if start_is_word {
        // Skip word characters
        while i < chars.len() && is_word_char(chars[i]) {
            i += 1;
        }
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
    } else {
        // Skip non-word non-space characters
        while i < chars.len() && !is_word_char(chars[i]) && !chars[i].is_whitespace() {
            i += 1;
        }
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
    }

    if i < chars.len() && i > start {
        Some(i)
    } else {
        None
    }
}

/// Find the start of the previous word from the given position.
fn find_word_start_backward(chars: &[char], start: usize) -> Option<usize> {
    if chars.is_empty() || start == 0 {
        return None;
    }

    let mut i = start.saturating_sub(1);

    // Skip whitespace backwards
    while i > 0 && chars[i].is_whitespace() {
        i -= 1;
    }

    if chars[i].is_whitespace() && i == 0 {
        return Some(0);
    }

    let is_word = is_word_char(chars[i]);

    // Find start of word/non-word sequence
    if is_word {
        while i > 0 && is_word_char(chars[i - 1]) {
            i -= 1;
        }
    } else {
        while i > 0 && !is_word_char(chars[i - 1]) && !chars[i - 1].is_whitespace() {
            i -= 1;
        }
    }

    if i < start {
        Some(i)
    } else {
        None
    }
}

/// Find the end of the current/next word from the given position.
fn find_word_end_forward(chars: &[char], start: usize) -> Option<usize> {
    if chars.is_empty() {
        return None;
    }

    let mut i = start;

    // Move past current position if not at start
    if i < chars.len() {
        i += 1;
    }

    // Skip whitespace
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    if i >= chars.len() {
        return None;
    }

    let is_word = is_word_char(chars[i]);

    // Find end of word/non-word sequence
    if is_word {
        while i + 1 < chars.len() && is_word_char(chars[i + 1]) {
            i += 1;
        }
    } else {
        while i + 1 < chars.len() && !is_word_char(chars[i + 1]) && !chars[i + 1].is_whitespace() {
            i += 1;
        }
    }

    Some(i)
}

use crate::Buffer;

impl CursorOps for Buffer {
    fn line_count(&self) -> usize {
        Buffer::line_count(self)
    }

    fn line_len(&self, line: usize) -> Option<usize> {
        Buffer::line_len(self, line)
    }

    fn line_content(&self, line: usize) -> Option<String> {
        Buffer::line(self, line)
    }

    fn cursor(&self) -> Cursor {
        Buffer::cursor(self)
    }

    fn cursor_mut(&mut self) -> &mut Cursor {
        Buffer::cursor_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, LineCol};

    #[test]
    fn move_down_clamps() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "short\nlongerline",
        );
        buf.cursor_mut().position = LineCol::new(1, 9);
        buf.move_up();
        assert_eq!(buf.cursor().position.col, 4);
    }

    #[test]
    fn word_forward() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "hello world foo",
        );
        buf.cursor_mut().position = LineCol::new(0, 0);
        buf.move_word_forward();
        assert_eq!(buf.cursor().position.col, 6); // "world"
        buf.move_word_forward();
        assert_eq!(buf.cursor().position.col, 12); // "foo"
    }

    #[test]
    fn word_backward() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "hello world foo",
        );
        buf.cursor_mut().position = LineCol::new(0, 12); // at "foo"
        buf.move_word_backward();
        assert_eq!(buf.cursor().position.col, 6); // "world"
        buf.move_word_backward();
        assert_eq!(buf.cursor().position.col, 0); // "hello"
    }

    #[test]
    fn word_end() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "hello world foo",
        );
        buf.cursor_mut().position = LineCol::new(0, 0);
        buf.move_word_end();
        assert_eq!(buf.cursor().position.col, 4); // end of "hello"
        buf.move_word_end();
        assert_eq!(buf.cursor().position.col, 10); // end of "world"
    }

    #[test]
    fn first_non_blank() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "   indented",
        );
        buf.cursor_mut().position = LineCol::new(0, 0);
        buf.move_first_non_blank();
        assert_eq!(buf.cursor().position.col, 3);
    }

    #[test]
    fn file_start_end() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "line1\nline2\nline3",
        );
        buf.cursor_mut().position = LineCol::new(1, 2);
        buf.move_file_start();
        assert_eq!(buf.cursor().position.line, 0);
        
        buf.move_file_end();
        assert_eq!(buf.cursor().position.line, 2);
    }
}
