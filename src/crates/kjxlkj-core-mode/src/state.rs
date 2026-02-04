//! Mode state aggregation.

use crate::{Intent, NormalModeState};
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
}
