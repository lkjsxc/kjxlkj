//! Mode state management.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

use crate::{CommandState, InsertState, NormalState, VisualState};

/// Aggregate mode state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Normal mode state.
    pub normal: NormalState,
    /// Insert mode state.
    pub insert: InsertState,
    /// Visual mode state.
    pub visual: VisualState,
    /// Command mode state.
    pub command: CommandState,
}

impl ModeState {
    /// Creates a new mode state in normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            normal: NormalState::new(),
            insert: InsertState::new(),
            visual: VisualState::new(),
            command: CommandState::new(),
        }
    }

    /// Transitions to a new mode.
    pub fn transition(&mut self, mode: Mode) {
        self.mode = mode;
        match mode {
            Mode::Normal => self.normal.reset(),
            Mode::Insert => self.insert.reset(),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {}
            Mode::Command => self.command.reset(),
            Mode::Replace => {}
            Mode::OperatorPending => {}
        }
    }

    /// Returns the mode name for display.
    pub fn mode_name(&self) -> &'static str {
        self.mode.name()
    }

    /// Returns true if in any visual mode.
    pub fn is_visual(&self) -> bool {
        self.mode.is_visual()
    }

    /// Returns true if in insert or replace mode.
    pub fn is_insert_like(&self) -> bool {
        self.mode.is_insert_like()
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_state_new() {
        let state = ModeState::new();
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn test_transition_to_insert() {
        let mut state = ModeState::new();
        state.transition(Mode::Insert);
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn test_transition_to_visual() {
        let mut state = ModeState::new();
        state.transition(Mode::Visual);
        assert!(state.is_visual());
    }

    #[test]
    fn test_transition_to_command() {
        let mut state = ModeState::new();
        state.transition(Mode::Command);
        assert_eq!(state.mode, Mode::Command);
        assert_eq!(state.mode_name(), "COMMAND");
    }

    #[test]
    fn test_is_insert_like() {
        let mut state = ModeState::new();
        state.transition(Mode::Insert);
        assert!(state.is_insert_like());
        
        state.transition(Mode::Replace);
        assert!(state.is_insert_like());
        
        state.transition(Mode::Normal);
        assert!(!state.is_insert_like());
    }
}
