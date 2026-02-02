//! Command-line input buffer.

use crate::cmdline_history::CmdHistory;

/// A buffer for command-line input.
#[derive(Debug, Clone, Default)]
pub struct CommandLine {
    input: String,
    cursor: usize,
    prompt: char,
    history: CmdHistory,
}

impl CommandLine {
    /// Creates a new command line.
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens the command line with a prompt.
    pub fn open(&mut self, prompt: char) {
        self.input.clear();
        self.cursor = 0;
        self.prompt = prompt;
        self.history.reset_index();
    }

    /// Closes the command line and returns the input.
    pub fn close(&mut self) -> String {
        let input = std::mem::take(&mut self.input);
        self.cursor = 0;
        self.prompt = '\0';  // Reset prompt to indicate closed
        input
    }

    /// Returns true if the command line is open.
    pub fn is_open(&self) -> bool {
        self.prompt != '\0'
    }

    /// Returns the current input.
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Returns the prompt.
    pub fn prompt(&self) -> char {
        self.prompt
    }

    /// Returns cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the display line (prompt + input), or None if closed.
    pub fn display(&self) -> Option<String> {
        if self.is_open() {
            Some(format!("{}{}", self.prompt, self.input))
        } else {
            None
        }
    }

    /// Inserts a character at cursor.
    pub fn insert(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Deletes the character before cursor.
    pub fn backspace(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let prev = self.input[..self.cursor]
            .chars()
            .last()
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        self.cursor -= prev;
        self.input.remove(self.cursor);
        true
    }

    /// Deletes the character at cursor.
    pub fn delete(&mut self) -> bool {
        if self.cursor >= self.input.len() {
            return false;
        }
        self.input.remove(self.cursor);
        true
    }

    /// Moves cursor left.
    pub fn move_left(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let prev = self.input[..self.cursor]
            .chars()
            .last()
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        self.cursor -= prev;
        true
    }

    /// Moves cursor right.
    pub fn move_right(&mut self) -> bool {
        if self.cursor >= self.input.len() {
            return false;
        }
        let curr = self.input[self.cursor..]
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        self.cursor += curr;
        true
    }

    /// Moves cursor to start.
    pub fn move_start(&mut self) {
        self.cursor = 0;
    }

    /// Moves cursor to end.
    pub fn move_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// Adds current input to history.
    pub fn add_to_history(&mut self) {
        self.history.add(self.input.clone());
    }

    /// Goes to previous history entry.
    pub fn history_prev(&mut self) -> bool {
        if let Some(entry) = self.history.prev() {
            self.input = entry.to_string();
            self.cursor = self.input.len();
            true
        } else {
            false
        }
    }

    /// Goes to next history entry.
    pub fn history_next(&mut self) -> bool {
        if let Some(entry) = self.history.next() {
            self.input = entry.to_string();
            self.cursor = self.input.len();
            true
        } else {
            self.input.clear();
            self.cursor = 0;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_close() {
        let mut cmd = CommandLine::new();
        cmd.open(':');
        assert_eq!(cmd.prompt(), ':');
        cmd.insert('w');
        assert_eq!(cmd.close(), "w");
    }

    #[test]
    fn test_insert_backspace() {
        let mut cmd = CommandLine::new();
        cmd.open(':');
        cmd.insert('a');
        cmd.insert('b');
        assert_eq!(cmd.input(), "ab");
        cmd.backspace();
        assert_eq!(cmd.input(), "a");
    }

    #[test]
    fn test_cursor_movement() {
        let mut cmd = CommandLine::new();
        cmd.open(':');
        cmd.insert('a');
        cmd.insert('b');
        cmd.insert('c');
        cmd.move_start();
        assert_eq!(cmd.cursor(), 0);
        cmd.move_right();
        assert_eq!(cmd.cursor(), 1);
        cmd.move_end();
        assert_eq!(cmd.cursor(), 3);
    }
}
