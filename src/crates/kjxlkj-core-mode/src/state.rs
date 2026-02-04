//! Mode state aggregation.

use crate::NormalModeState;
use kjxlkj_core_types::Mode;

/// Aggregated mode state.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Normal mode state machine.
    pub normal: NormalModeState,
    /// Command line buffer.
    pub command_line: String,
    /// Search pattern.
    pub search_pattern: String,
    /// Search direction (true = forward).
    pub search_forward: bool,
    /// Whether currently recording a macro.
    pub recording_macro: Option<char>,
}

impl ModeState {
    /// Create a new mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset to normal mode.
    pub fn reset(&mut self) {
        self.mode = Mode::Normal;
        self.normal.reset();
        self.command_line.clear();
    }

    /// Set the mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        if mode == Mode::Normal {
            self.normal.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_state_default() {
        let state = ModeState::new();
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn mode_state_set_mode() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn mode_state_reset() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        state.command_line = "test".to_string();
        state.reset();
        assert_eq!(state.mode, Mode::Normal);
        assert!(state.command_line.is_empty());
    }

    #[test]
    fn normal_to_insert_transition() {
        let mut state = ModeState::new();
        assert_eq!(state.mode, Mode::Normal);
        state.set_mode(Mode::Insert);
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn normal_to_visual_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Visual);
        assert_eq!(state.mode, Mode::Visual);
    }

    #[test]
    fn normal_to_command_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Command);
        assert_eq!(state.mode, Mode::Command);
    }

    #[test]
    fn normal_to_replace_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Replace);
        assert_eq!(state.mode, Mode::Replace);
    }

    #[test]
    fn insert_to_normal_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        state.set_mode(Mode::Normal);
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn visual_to_normal_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Visual);
        state.set_mode(Mode::Normal);
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn command_to_normal_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Command);
        state.set_mode(Mode::Normal);
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn normal_resets_normal_state() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        state.set_mode(Mode::Normal);
        // Setting to Normal should reset the normal mode state
        assert_eq!(state.normal.get_count(), 1); // Default count is 1
    }
}
