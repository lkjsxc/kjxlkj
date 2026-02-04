//! Command-line mode state.

/// State for command-line (Ex) mode.
#[derive(Debug, Default)]
pub struct CommandState {
    /// The command line buffer.
    pub line: String,
    /// Cursor position in the command line.
    pub cursor: usize,
    /// Command history.
    pub history: Vec<String>,
    /// Current history index.
    pub history_idx: Option<usize>,
}

impl CommandState {
    /// Create a new empty command state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear the command line.
    pub fn clear(&mut self) {
        self.line.clear();
        self.cursor = 0;
        self.history_idx = None;
    }

    /// Insert a character at cursor.
    pub fn insert(&mut self, c: char) {
        self.line.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Delete character before cursor.
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            let prev = self.line[..self.cursor]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor -= prev;
            self.line.remove(self.cursor);
        }
    }

    /// Move cursor left.
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            let prev = self.line[..self.cursor]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor -= prev;
        }
    }

    /// Move cursor right.
    pub fn move_right(&mut self) {
        if self.cursor < self.line.len() {
            let next = self.line[self.cursor..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor += next;
        }
    }

    /// Get the current command.
    pub fn command(&self) -> &str {
        &self.line
    }

    /// Submit the command and add to history.
    pub fn submit(&mut self) -> String {
        let cmd = std::mem::take(&mut self.line);
        if !cmd.is_empty() {
            self.history.push(cmd.clone());
        }
        self.cursor = 0;
        self.history_idx = None;
        cmd
    }

    /// Navigate to previous history entry.
    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let new_idx = match self.history_idx {
            None => self.history.len().saturating_sub(1),
            Some(i) => i.saturating_sub(1),
        };
        self.history_idx = Some(new_idx);
        self.line = self.history[new_idx].clone();
        self.cursor = self.line.len();
    }

    /// Navigate to next history entry.
    pub fn history_next(&mut self) {
        match self.history_idx {
            None => {}
            Some(i) => {
                if i + 1 < self.history.len() {
                    self.history_idx = Some(i + 1);
                    self.line = self.history[i + 1].clone();
                } else {
                    self.history_idx = None;
                    self.line.clear();
                }
                self.cursor = self.line.len();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut state = CommandState::new();
        state.insert('w');
        state.insert('q');
        assert_eq!(state.command(), "wq");
    }

    #[test]
    fn test_backspace() {
        let mut state = CommandState::new();
        state.insert('a');
        state.insert('b');
        state.backspace();
        assert_eq!(state.command(), "a");
    }
}
