//! Mode transitions and transition result types.
//!
//! Per /docs/spec/modes/transitions.md, transitions are deterministic.

use kjxlkj_core_types::Mode;

/// The outcome of a mode transition request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionResult {
    /// Stay in the current mode.
    Stay,
    /// Switch to a new mode.
    Switch(Mode),
}

/// Mode transition logic: compute the next mode given a current mode
/// and requested target mode.
pub struct ModeTransition;

impl ModeTransition {
    /// Determine the resulting mode for a transition request.
    ///
    /// Some transitions are invalid / no-ops (e.g., Normal → Normal stays).
    pub fn compute(current: Mode, target: Mode) -> TransitionResult {
        if current == target {
            return TransitionResult::Stay;
        }

        match (&current, &target) {
            // Insert → InsertNormal (Ctrl-O in Insert mode).
            (Mode::Insert, Mode::InsertNormal) => {
                TransitionResult::Switch(Mode::InsertNormal)
            }

            // InsertNormal → Insert (after single Normal cmd completed).
            (Mode::InsertNormal, Mode::Insert) => {
                TransitionResult::Switch(Mode::Insert)
            }

            // Visual → Visual (toggle sub-kind).
            (Mode::Visual(_), Mode::Visual(_)) => {
                TransitionResult::Switch(target)
            }

            // Visual → OperatorPending.
            (Mode::Visual(_), Mode::OperatorPending(_)) => {
                TransitionResult::Switch(target)
            }

            // Replace → Insert (when pressing Insert key in Replace).
            (Mode::Replace, Mode::Insert) => {
                TransitionResult::Switch(Mode::Insert)
            }

            // Any mode → Normal.
            (_, Mode::Normal) => TransitionResult::Switch(Mode::Normal),

            // Normal → Insert.
            (Mode::Normal, Mode::Insert) => {
                TransitionResult::Switch(Mode::Insert)
            }

            // Normal → Replace.
            (Mode::Normal, Mode::Replace) => {
                TransitionResult::Switch(Mode::Replace)
            }

            // Normal → Visual (any sub-kind).
            (Mode::Normal, Mode::Visual(_)) => {
                TransitionResult::Switch(target)
            }

            // Normal → Command.
            (Mode::Normal, Mode::Command(_)) => {
                TransitionResult::Switch(target)
            }

            // Normal → OperatorPending.
            (Mode::Normal, Mode::OperatorPending(_)) => {
                TransitionResult::Switch(target)
            }

            // Normal → TerminalInsert (when focusing terminal buffer).
            (Mode::Normal, Mode::TerminalInsert) => {
                TransitionResult::Switch(Mode::TerminalInsert)
            }

            // Default: allow the transition.
            (_, _) => TransitionResult::Switch(target),
        }
    }

    /// Should cursor clamp after transition?
    /// Insert → Normal requires clamping cursor one left.
    pub fn should_clamp_cursor(from: Mode, to: Mode) -> bool {
        matches!(
            (from, to),
            (Mode::Insert, Mode::Normal) | (Mode::Replace, Mode::Normal)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::VisualKind;

    #[test]
    fn normal_to_insert() {
        assert_eq!(
            ModeTransition::compute(Mode::Normal, Mode::Insert),
            TransitionResult::Switch(Mode::Insert),
        );
    }

    #[test]
    fn same_mode_stays() {
        assert_eq!(
            ModeTransition::compute(Mode::Normal, Mode::Normal),
            TransitionResult::Stay,
        );
    }

    #[test]
    fn insert_to_normal_clamp() {
        assert!(ModeTransition::should_clamp_cursor(
            Mode::Insert,
            Mode::Normal,
        ));
    }

    #[test]
    fn normal_to_visual() {
        assert_eq!(
            ModeTransition::compute(
                Mode::Normal,
                Mode::Visual(VisualKind::Char)
            ),
            TransitionResult::Switch(Mode::Visual(VisualKind::Char)),
        );
    }

    #[test]
    fn visual_to_visual_toggle() {
        assert_eq!(
            ModeTransition::compute(
                Mode::Visual(VisualKind::Char),
                Mode::Visual(VisualKind::Line)
            ),
            TransitionResult::Switch(Mode::Visual(VisualKind::Line)),
        );
    }

    #[test]
    fn no_clamp_normal_to_insert() {
        assert!(!ModeTransition::should_clamp_cursor(
            Mode::Normal,
            Mode::Insert,
        ));
    }
}
