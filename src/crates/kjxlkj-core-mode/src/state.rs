//! Mode state management.

use kjxlkj_core_types::Mode;

/// Mode state with transition tracking.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    /// Current mode.
    mode: Mode,
    /// Previous mode (for returning after Command mode).
    previous: Option<Mode>,
}

impl ModeState {
    /// Create a new mode state in Normal mode.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Transition to a new mode.
    pub fn transition(&mut self, new_mode: Mode) {
        if new_mode == Mode::Command {
            self.previous = Some(self.mode);
        }
        self.mode = new_mode;
    }

    /// Return to Normal mode.
    pub fn to_normal(&mut self) {
        self.mode = Mode::Normal;
        self.previous = None;
    }

    /// Return to previous mode (from Command mode).
    pub fn to_previous(&mut self) {
        if let Some(prev) = self.previous.take() {
            self.mode = prev;
        } else {
            self.mode = Mode::Normal;
        }
    }

    /// Check if in a visual mode.
    pub fn is_visual(&self) -> bool {
        self.mode.is_visual()
    }

    /// Check if in insert mode.
    pub fn is_insert(&self) -> bool {
        self.mode.is_insert()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_transitions() {
        let mut state = ModeState::new();
        assert_eq!(state.mode(), Mode::Normal);

        state.transition(Mode::Insert);
        assert_eq!(state.mode(), Mode::Insert);

        state.to_normal();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn test_command_mode_previous() {
        let mut state = ModeState::new();
        state.transition(Mode::Visual);
        state.transition(Mode::Command);
        assert_eq!(state.mode(), Mode::Command);

        state.to_previous();
        assert_eq!(state.mode(), Mode::Visual);
    }
}
