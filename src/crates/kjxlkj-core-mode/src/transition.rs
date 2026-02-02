//! Mode transitions.

use kjxlkj_core_types::mode::Mode;

/// Represents a mode transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeTransition {
    /// The source mode.
    pub from: Mode,
    /// The target mode.
    pub to: Mode,
}

impl ModeTransition {
    /// Creates a new mode transition.
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }

    /// Returns true if this is entering insert mode from normal.
    pub fn is_enter_insert(&self) -> bool {
        matches!(
            (&self.from, &self.to),
            (Mode::Normal, Mode::Insert) | (Mode::Visual(_), Mode::Insert)
        )
    }

    /// Returns true if this is exiting insert mode.
    pub fn is_exit_insert(&self) -> bool {
        matches!(
            (&self.from, &self.to),
            (Mode::Insert, Mode::Normal)
        )
    }

    /// Returns true if this is entering visual mode.
    pub fn is_enter_visual(&self) -> bool {
        matches!(
            (&self.from, &self.to),
            (Mode::Normal, Mode::Visual(_))
        )
    }

    /// Returns true if this is exiting visual mode.
    pub fn is_exit_visual(&self) -> bool {
        matches!(
            (&self.from, &self.to),
            (Mode::Visual(_), Mode::Normal)
        )
    }

    /// Returns true if this is a visual mode toggle.
    pub fn is_visual_toggle(&self) -> bool {
        matches!(
            (&self.from, &self.to),
            (Mode::Visual(_), Mode::Visual(_))
        )
    }
}
