//! Mode tracking state.

use kjxlkj_core_types::Mode;

/// Tracks the current mode and history of transitions.
pub struct ModeState {
    current: Mode,
    previous: Mode,
}

impl ModeState {
    pub fn new() -> Self {
        Self {
            current: Mode::Normal,
            previous: Mode::Normal,
        }
    }

    pub fn current(&self) -> Mode {
        self.current
    }

    pub fn previous(&self) -> Mode {
        self.previous
    }

    pub fn transition(&mut self, mode: Mode) {
        self.previous = self.current;
        self.current = mode;
    }

    pub fn is_normal(&self) -> bool {
        self.current == Mode::Normal
    }

    pub fn is_insert(&self) -> bool {
        self.current == Mode::Insert
    }

    pub fn is_visual(&self) -> bool {
        self.current.is_visual()
    }

    pub fn is_command(&self) -> bool {
        self.current == Mode::Command
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
    fn initial_mode_is_normal() {
        let s = ModeState::new();
        assert_eq!(s.current(), Mode::Normal);
        assert!(s.is_normal());
    }

    #[test]
    fn transition_tracks_previous() {
        let mut s = ModeState::new();
        s.transition(Mode::Insert);
        assert_eq!(s.current(), Mode::Insert);
        assert_eq!(s.previous(), Mode::Normal);
    }

    #[test]
    fn escape_returns_to_normal() {
        let mut s = ModeState::new();
        s.transition(Mode::Insert);
        s.transition(Mode::Normal);
        assert!(s.is_normal());
        assert_eq!(s.previous(), Mode::Insert);
    }
}
