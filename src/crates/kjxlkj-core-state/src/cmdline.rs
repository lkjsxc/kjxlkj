use kjxlkj_core_ui::CmdlineState;

use crate::cmdline_completion::CompletionState;

/// Command-line handler for ex commands and search.
#[derive(Debug)]
pub struct CmdlineHandler {
    pub prefix: Option<char>,
    pub content: String,
    pub cursor_pos: usize,
    pub active: bool,
    pub history: Vec<String>,
    pub history_idx: Option<usize>,
    pub completion: CompletionState,
}

impl CmdlineHandler {
    pub fn new() -> Self {
        Self {
            prefix: None,
            content: String::new(),
            cursor_pos: 0,
            active: false,
            history: Vec::new(),
            history_idx: None,
            completion: CompletionState::default(),
        }
    }

    pub fn open(&mut self, prefix: char) {
        self.prefix = Some(prefix);
        self.content.clear();
        self.cursor_pos = 0;
        self.active = true;
        self.history_idx = None;
    }

    pub fn close(&mut self) {
        if !self.content.is_empty() {
            self.history.push(self.content.clone());
        }
        self.prefix = None;
        self.content.clear();
        self.cursor_pos = 0;
        self.active = false;
        self.history_idx = None;
    }

    pub fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.content[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.content.remove(prev);
            self.cursor_pos = prev;
        }
    }

    pub fn delete_at_cursor(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.content.remove(self.cursor_pos);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.content[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.cursor_pos += self.content[self.cursor_pos..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_pos = self.content.len();
    }

    pub fn delete_word_backward(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        let before = &self.content[..self.cursor_pos];
        let trimmed = before.trim_end();
        let new_pos = trimmed
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        self.content.drain(new_pos..self.cursor_pos);
        self.cursor_pos = new_pos;
    }

    pub fn delete_to_start(&mut self) {
        self.content.drain(..self.cursor_pos);
        self.cursor_pos = 0;
    }

    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let idx = match self.history_idx {
            Some(0) => return,
            Some(i) => i - 1,
            None => self.history.len() - 1,
        };
        self.history_idx = Some(idx);
        self.content = self.history[idx].clone();
        self.cursor_pos = self.content.len();
    }

    pub fn history_next(&mut self) {
        let idx = match self.history_idx {
            Some(i) => i + 1,
            None => return,
        };
        if idx < self.history.len() {
            self.history_idx = Some(idx);
            self.content = self.history[idx].clone();
            self.cursor_pos = self.content.len();
        } else {
            self.history_idx = None;
            self.content.clear();
            self.cursor_pos = 0;
        }
    }

    pub fn take_content(&mut self) -> String {
        let content = self.content.clone();
        self.close();
        content
    }

    pub fn snapshot(&self) -> CmdlineState {
        CmdlineState {
            prefix: self.prefix,
            content: self.content.clone(),
            cursor_pos: self.cursor_pos,
            active: self.active,
        }
    }
}

impl Default for CmdlineHandler {
    fn default() -> Self {
        Self::new()
    }
}
