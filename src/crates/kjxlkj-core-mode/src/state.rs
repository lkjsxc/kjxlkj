//! Modal state management.

use kjxlkj_core_types::{Mode, RegisterName};

/// Mode state for the editor.
#[derive(Debug, Clone)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Count prefix.
    pub count: Option<usize>,
    /// Pending operator.
    pub pending_operator: Option<PendingOperator>,
    /// Active register.
    pub register: RegisterName,
    /// Command line input.
    pub cmdline: String,
    /// Search pattern.
    pub search_pattern: String,
    /// Search direction.
    pub search_forward: bool,
    /// Recording macro register.
    pub recording_macro: Option<char>,
    /// Macro being recorded.
    pub macro_buffer: Vec<kjxlkj_core_types::KeyEvent>,
}

impl Default for ModeState {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            count: None,
            pending_operator: None,
            register: RegisterName::Unnamed,
            cmdline: String::new(),
            search_pattern: String::new(),
            search_forward: true,
            recording_macro: None,
            macro_buffer: Vec::new(),
        }
    }
}

impl ModeState {
    /// Create new mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get effective count (default 1).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Reset to normal mode.
    pub fn reset(&mut self) {
        self.mode = Mode::Normal;
        self.count = None;
        self.pending_operator = None;
        self.register = RegisterName::Unnamed;
    }

    /// Enter insert mode.
    pub fn enter_insert(&mut self) {
        self.mode = Mode::Insert;
        self.count = None;
        self.pending_operator = None;
    }

    /// Enter visual mode.
    pub fn enter_visual(&mut self, visual_mode: VisualModeType) {
        self.mode = match visual_mode {
            VisualModeType::Char => Mode::Visual,
            VisualModeType::Line => Mode::VisualLine,
            VisualModeType::Block => Mode::VisualBlock,
        };
        self.count = None;
        self.pending_operator = None;
    }

    /// Enter command mode.
    pub fn enter_command(&mut self, prefix: char) {
        self.mode = Mode::Command;
        self.cmdline.clear();
        if prefix == '/' {
            self.search_forward = true;
        } else if prefix == '?' {
            self.search_forward = false;
        }
    }

    /// Enter replace mode.
    pub fn enter_replace(&mut self) {
        self.mode = Mode::Replace;
        self.count = None;
        self.pending_operator = None;
    }

    /// Append digit to count.
    pub fn append_count(&mut self, digit: u8) {
        let current = self.count.unwrap_or(0);
        self.count = Some(current * 10 + digit as usize);
    }
}

/// Pending operator state.
#[derive(Debug, Clone)]
pub struct PendingOperator {
    /// The operator type.
    pub operator: kjxlkj_core_types::Operator,
    /// Count prefix for operator.
    pub count: Option<usize>,
}

/// Visual mode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualModeType {
    /// Character-wise.
    Char,
    /// Line-wise.
    Line,
    /// Block.
    Block,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = ModeState::new();
        assert_eq!(state.mode, Mode::Normal);
        assert_eq!(state.count, None);
        assert_eq!(state.effective_count(), 1);
    }

    #[test]
    fn test_enter_insert() {
        let mut state = ModeState::new();
        state.enter_insert();
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn test_append_count() {
        let mut state = ModeState::new();
        state.append_count(2);
        state.append_count(3);
        assert_eq!(state.effective_count(), 23);
    }
}
