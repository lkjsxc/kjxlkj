use super::Buffer;

impl Buffer {
    /// Inserts a character at cursor.
    pub fn insert_char(&mut self, c: char) {
        let line = self.cursor_line();
        let col = self.cursor_col();
        let char_idx = self.text.line_to_char(line) + col;
        self.text.insert(char_idx, &c.to_string());
        self.move_cursor(line, col + 1);
        self.modified = true;
    }

    /// Inserts a newline at cursor.
    pub fn insert_newline(&mut self) {
        let line = self.cursor_line();
        let col = self.cursor_col();
        let char_idx = self.text.line_to_char(line) + col;
        self.text.insert(char_idx, "\n");
        self.move_cursor(line + 1, 0);
        self.modified = true;
    }

    /// Deletes character before cursor (backspace).
    pub fn delete_char_before(&mut self) {
        let line = self.cursor_line();
        let col = self.cursor_col();
        if col > 0 {
            let char_idx = self.text.line_to_char(line) + col - 1;
            self.text.remove(char_idx, char_idx + 1);
            self.move_cursor(line, col - 1);
            self.modified = true;
        } else if line > 0 {
            // Join with previous line
            let prev_line_len = self.line_len(line - 1);
            let char_idx = self.text.line_to_char(line) - 1;
            self.text.remove(char_idx, char_idx + 1);
            self.move_cursor(line - 1, prev_line_len);
            self.modified = true;
        }
    }

    /// Deletes character at cursor.
    pub fn delete_char_at(&mut self) {
        let line = self.cursor_line();
        let col = self.cursor_col();
        let line_len = self.line_len(line);
        if col < line_len {
            let char_idx = self.text.line_to_char(line) + col;
            self.text.remove(char_idx, char_idx + 1);
            self.modified = true;
        }
    }

    /// Deletes an entire line.
    pub fn delete_line(&mut self, line: usize) {
        if line >= self.line_count() {
            return;
        }
        let start = self.text.line_to_char(line);
        let end = if line + 1 < self.line_count() {
            self.text.line_to_char(line + 1)
        } else {
            self.text.len_chars()
        };
        self.text.remove(start, end);
        self.modified = true;
        // Adjust cursor if needed
        if self.cursor_line() >= self.line_count() {
            let new_line = self.line_count().saturating_sub(1);
            self.move_cursor(new_line, 0);
        }
    }

    /// Sets or replaces a line's content.
    pub fn set_line(&mut self, line: usize, content: &str) {
        if line >= self.line_count() {
            return;
        }
        let start = self.text.line_to_char(line);
        let end = if line + 1 < self.line_count() {
            // Keep newline, replace content
            self.text.line_to_char(line + 1) - 1
        } else {
            self.text.len_chars()
        };
        self.text.replace(start, end, content);
        self.modified = true;
    }
}
