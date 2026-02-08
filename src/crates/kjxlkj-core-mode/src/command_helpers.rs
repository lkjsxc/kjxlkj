//! Command-line mode helper utilities: history
//! navigation, word deletion, char boundary.

use kjxlkj_core_types::Direction;

use crate::command::CommandModeState;

impl CommandModeState {
    pub(crate) fn prev_char_boundary(
        &self,
    ) -> usize {
        let mut idx =
            self.cursor.saturating_sub(1);
        while idx > 0
            && !self.buffer.is_char_boundary(idx)
        {
            idx -= 1;
        }
        idx
    }

    pub(crate) fn next_char_boundary(
        &self,
    ) -> usize {
        let mut idx = self.cursor + 1;
        while idx < self.buffer.len()
            && !self.buffer.is_char_boundary(idx)
        {
            idx += 1;
        }
        idx.min(self.buffer.len())
    }

    pub(crate) fn navigate_history(
        &mut self,
        direction: Direction,
    ) {
        if self.history.is_empty() {
            return;
        }
        match direction {
            Direction::Up => {
                let new_idx =
                    match self.history_index {
                        None => {
                            self.saved_input =
                                self.buffer.clone();
                            self.history.len() - 1
                        }
                        Some(0) => return,
                        Some(i) => i - 1,
                    };
                self.history_index = Some(new_idx);
                self.buffer =
                    self.history[new_idx].clone();
                self.cursor = self.buffer.len();
            }
            Direction::Down => {
                match self.history_index {
                    None => {}
                    Some(i)
                        if i + 1
                            >= self.history.len() =>
                    {
                        self.history_index = None;
                        self.buffer =
                            self.saved_input.clone();
                        self.cursor =
                            self.buffer.len();
                    }
                    Some(i) => {
                        self.history_index =
                            Some(i + 1);
                        self.buffer = self.history
                            [i + 1]
                            .clone();
                        self.cursor =
                            self.buffer.len();
                    }
                }
            }
            _ => {}
        }
    }

    pub(crate) fn delete_word_backward(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let bytes = self.buffer.as_bytes();
        let mut pos = self.cursor;
        while pos > 0 && bytes[pos - 1] == b' ' {
            pos -= 1;
        }
        while pos > 0 && bytes[pos - 1] != b' ' {
            pos -= 1;
        }
        self.buffer.drain(pos..self.cursor);
        self.cursor = pos;
    }

    /// Get the current buffer content.
    pub fn content(&self) -> &str {
        &self.buffer
    }
}
