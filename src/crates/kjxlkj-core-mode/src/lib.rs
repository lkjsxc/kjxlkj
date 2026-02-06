//! Modal state machines — mode transitions and key dispatch.

use kjxlkj_core_types::Mode;

/// Describes a transition between editor modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeTransition {
    pub from: Mode,
    pub to: Mode,
}

impl ModeTransition {
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }
}

/// Tracks the current mode and validates transitions.
pub struct ModeManager {
    current: Mode,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current: Mode::Normal,
        }
    }

    /// Return the current mode.
    pub fn current(&self) -> Mode {
        self.current
    }

    /// Attempt a transition to the given mode.
    /// Returns the resulting `ModeTransition`.
    pub fn transition_to(&mut self, target: Mode) -> ModeTransition {
        let from = self.current;
        self.current = target;
        ModeTransition::new(from, target)
    }

    /// Reset to Normal mode.
    pub fn reset(&mut self) -> ModeTransition {
        self.transition_to(Mode::Normal)
    }
}

impl Default for ModeManager {
    fn default() -> Self {
        Self::new()
    }
}
