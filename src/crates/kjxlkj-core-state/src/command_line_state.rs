//! Command-line input state and history.

use kjxlkj_core_types::{KeyCode, KeyEvent, Modifiers};

/// Command-line history with navigation.
#[derive(Debug, Clone)]
pub struct CommandHistory {
    entries: Vec<String>,
    index: usize,
    max_size: usize,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            index: 0,
            max_size,
        }
    }

    /// Add a new entry to history.
    pub fn push(&mut self, entry: String) {
        if entry.is_empty() {
            return;
        }
        // Remove duplicate if exists
        self.entries.retain(|e| e != &entry);
        self.entries.push(entry);
        if self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
        self.index = self.entries.len();
    }

    /// Navigate to previous history entry.
    pub fn prev(&mut self) -> Option<&String> {
        if self.index == 0 || self.entries.is_empty() {
            return None;
        }
        self.index -= 1;
        self.entries.get(self.index)
    }

    /// Navigate to next history entry.
    pub fn next(&mut self) -> Option<&String> {
        if self.index >= self.entries.len() {
            return None;
        }
        self.index += 1;
        self.entries.get(self.index)
    }

    /// Search for an entry matching a prefix.
    pub fn search_prefix(&self, prefix: &str) -> Option<&String> {
        self.entries.iter().rev().find(|e| e.starts_with(prefix))
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Reset index to end of history.
    pub fn reset_index(&mut self) {
        self.index = self.entries.len();
    }
}

/// Active command-line input state.
#[derive(Debug, Clone)]
pub struct CommandLineState {
    pub prefix: String,
    pub content: String,
    pub cursor_pos: usize,
    pub history: CommandHistory,
    pub active: bool,
}

impl CommandLineState {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            content: String::new(),
            cursor_pos: 0,
            history: CommandHistory::new(200),
            active: false,
        }
    }

    /// Activate command line with given prefix (e.g. ":", "/", "?").
    pub fn activate(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
        self.content.clear();
        self.cursor_pos = 0;
        self.active = true;
        self.history.reset_index();
    }

    /// Deactivate command line.
    pub fn deactivate(&mut self) {
        self.active = false;
        self.content.clear();
        self.cursor_pos = 0;
    }

    /// Get full display text (prefix + content).
    pub fn full_text(&self) -> String {
        format!("{}{}", self.prefix, self.content)
    }
}

impl Default for CommandLineState {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions for command-line editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineAction {
    InsertChar(char),
    DeleteBack,
    DeleteWord,
    DeleteToStart,
    MoveLeft,
    MoveRight,
    MoveStart,
    MoveEnd,
    HistoryPrev,
    HistoryNext,
    Accept,
    Cancel,
    Complete,
    PasteRegister,
}

/// Map a key event to a command-line action.
pub fn map_cmdline_key(key: &KeyEvent) -> Option<CmdlineAction> {
    if key.modifiers.contains(Modifiers::CTRL) {
        return match &key.code {
            KeyCode::Char('h') => Some(CmdlineAction::DeleteBack),
            KeyCode::Char('w') => Some(CmdlineAction::DeleteWord),
            KeyCode::Char('u') => Some(CmdlineAction::DeleteToStart),
            KeyCode::Char('a') => Some(CmdlineAction::MoveStart),
            KeyCode::Char('e') => Some(CmdlineAction::MoveEnd),
            KeyCode::Char('r') => Some(CmdlineAction::PasteRegister),
            _ => None,
        };
    }
    match &key.code {
        KeyCode::Char(c) => Some(CmdlineAction::InsertChar(*c)),
        KeyCode::Backspace => Some(CmdlineAction::DeleteBack),
        KeyCode::Left => Some(CmdlineAction::MoveLeft),
        KeyCode::Right => Some(CmdlineAction::MoveRight),
        KeyCode::Up => Some(CmdlineAction::HistoryPrev),
        KeyCode::Down => Some(CmdlineAction::HistoryNext),
        KeyCode::Enter => Some(CmdlineAction::Accept),
        KeyCode::Escape => Some(CmdlineAction::Cancel),
        KeyCode::Tab => Some(CmdlineAction::Complete),
        KeyCode::Home => Some(CmdlineAction::MoveStart),
        KeyCode::End => Some(CmdlineAction::MoveEnd),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_push_and_nav() {
        let mut h = CommandHistory::new(10);
        h.push("q".into());
        h.push("w".into());
        let prev = h.prev().unwrap();
        assert_eq!(prev, "w");
        let prev = h.prev().unwrap();
        assert_eq!(prev, "q");
    }

    #[test]
    fn cmdline_activate_deactivate() {
        let mut cl = CommandLineState::new();
        cl.activate(":");
        assert!(cl.active);
        assert_eq!(cl.prefix, ":");
        cl.deactivate();
        assert!(!cl.active);
    }

    #[test]
    fn map_keys() {
        let esc = KeyEvent::plain(KeyCode::Escape);
        assert_eq!(map_cmdline_key(&esc), Some(CmdlineAction::Cancel));
        let enter = KeyEvent::plain(KeyCode::Enter);
        assert_eq!(map_cmdline_key(&enter), Some(CmdlineAction::Accept));
    }
}
