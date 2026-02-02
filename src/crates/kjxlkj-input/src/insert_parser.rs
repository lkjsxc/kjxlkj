//! Insert mode input handling.

use crate::key::{Key, KeyEvent};

/// Result of insert mode input.
#[derive(Debug, Clone)]
pub enum InsertResult {
    /// Insert a character.
    Insert(char),
    /// Insert a newline.
    Newline,
    /// Insert a tab.
    Tab,
    /// Delete character before cursor.
    Backspace,
    /// Delete character at cursor.
    Delete,
    /// Delete word before cursor.
    DeleteWord,
    /// Delete line before cursor.
    DeleteLine,
    /// Move cursor left.
    Left,
    /// Move cursor right.
    Right,
    /// Move cursor up.
    Up,
    /// Move cursor down.
    Down,
    /// Move to start of line.
    Home,
    /// Move to end of line.
    End,
    /// Exit insert mode.
    Exit,
    /// Toggle paste mode.
    PasteMode,
    /// Complete word.
    Complete,
    /// Ignore this input.
    Ignore,
}

/// Parser for insert mode.
#[derive(Debug, Default)]
pub struct InsertParser {
    /// Whether in paste mode.
    paste_mode: bool,
}

impl InsertParser {
    /// Creates a new insert parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a key event in insert mode.
    pub fn parse(&mut self, event: KeyEvent) -> InsertResult {
        // In paste mode, just insert characters
        if self.paste_mode {
            return match event.key {
                Key::Char(c) => InsertResult::Insert(c),
                Key::Enter => InsertResult::Newline,
                Key::Escape => {
                    self.paste_mode = false;
                    InsertResult::Exit
                }
                _ => InsertResult::Ignore,
            };
        }

        // Handle control sequences
        if event.modifiers.ctrl {
            return self.parse_ctrl(event);
        }

        match event.key {
            Key::Char(c) => InsertResult::Insert(c),
            Key::Enter => InsertResult::Newline,
            Key::Tab => InsertResult::Tab,
            Key::Backspace => InsertResult::Backspace,
            Key::Delete => InsertResult::Delete,
            Key::Left => InsertResult::Left,
            Key::Right => InsertResult::Right,
            Key::Up => InsertResult::Up,
            Key::Down => InsertResult::Down,
            Key::Home => InsertResult::Home,
            Key::End => InsertResult::End,
            Key::Escape => InsertResult::Exit,
            _ => InsertResult::Ignore,
        }
    }

    fn parse_ctrl(&mut self, event: KeyEvent) -> InsertResult {
        if let Key::Char(c) = event.key {
            match c {
                // Ctrl-[ is Escape
                '[' => InsertResult::Exit,
                // Ctrl-C also exits
                'c' => InsertResult::Exit,
                // Ctrl-H is backspace
                'h' => InsertResult::Backspace,
                // Ctrl-W deletes word
                'w' => InsertResult::DeleteWord,
                // Ctrl-U deletes line
                'u' => InsertResult::DeleteLine,
                // Ctrl-N/Ctrl-P for completion
                'n' | 'p' => InsertResult::Complete,
                // Other Ctrl sequences ignored
                _ => InsertResult::Ignore,
            }
        } else {
            InsertResult::Ignore
        }
    }

    /// Enters paste mode.
    pub fn enter_paste_mode(&mut self) {
        self.paste_mode = true;
    }

    /// Exits paste mode.
    pub fn exit_paste_mode(&mut self) {
        self.paste_mode = false;
    }

    /// Returns whether in paste mode.
    pub fn is_paste_mode(&self) -> bool {
        self.paste_mode
    }
}
