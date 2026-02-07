//! Mode transition validation based on the spec transition table.

use kjxlkj_core_types::Mode;
use thiserror::Error;

/// Error for invalid mode transitions.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransitionError {
    #[error("invalid transition from {from} to {to}")]
    InvalidTransition { from: Mode, to: Mode },
}

/// Static transition table: (from, to) pairs that are valid.
pub static TRANSITION_TABLE: &[(Mode, Mode)] = &[
    // From Normal
    (Mode::Normal, Mode::Insert),
    (Mode::Normal, Mode::Visual),
    (Mode::Normal, Mode::VisualLine),
    (Mode::Normal, Mode::VisualBlock),
    (Mode::Normal, Mode::Command),
    (Mode::Normal, Mode::Replace),
    (Mode::Normal, Mode::OperatorPending),
    (Mode::Normal, Mode::Terminal),
    // From Insert
    (Mode::Insert, Mode::Normal),
    // From Visual
    (Mode::Visual, Mode::Normal),
    (Mode::Visual, Mode::VisualLine),
    (Mode::Visual, Mode::VisualBlock),
    (Mode::Visual, Mode::Command),
    // From VisualLine
    (Mode::VisualLine, Mode::Normal),
    (Mode::VisualLine, Mode::Visual),
    (Mode::VisualLine, Mode::VisualBlock),
    (Mode::VisualLine, Mode::Command),
    // From VisualBlock
    (Mode::VisualBlock, Mode::Normal),
    (Mode::VisualBlock, Mode::Visual),
    (Mode::VisualBlock, Mode::VisualLine),
    (Mode::VisualBlock, Mode::Command),
    // From Command
    (Mode::Command, Mode::Normal),
    // From Replace
    (Mode::Replace, Mode::Normal),
    // From OperatorPending
    (Mode::OperatorPending, Mode::Normal),
    // From Terminal
    (Mode::Terminal, Mode::Normal),
    // Self-transitions (stay in same mode)
    (Mode::Normal, Mode::Normal),
];

/// Check whether transitioning from `from` to `to` is valid.
pub fn can_transition(from: Mode, to: Mode) -> bool {
    TRANSITION_TABLE.iter().any(|&(f, t)| f == from && t == to)
}

/// Validate a transition, returning an error if invalid.
pub fn validate_transition(from: Mode, to: Mode) -> Result<(), TransitionError> {
    if can_transition(from, to) {
        Ok(())
    } else {
        Err(TransitionError::InvalidTransition { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_to_insert_valid() {
        assert!(can_transition(Mode::Normal, Mode::Insert));
    }

    #[test]
    fn insert_to_visual_invalid() {
        assert!(!can_transition(Mode::Insert, Mode::Visual));
    }

    #[test]
    fn escape_from_all_modes() {
        let modes = [
            Mode::Insert,
            Mode::Visual,
            Mode::VisualLine,
            Mode::VisualBlock,
            Mode::Command,
            Mode::Replace,
            Mode::OperatorPending,
            Mode::Terminal,
        ];
        for m in modes {
            assert!(
                can_transition(m, Mode::Normal),
                "{m} -> Normal should be valid"
            );
        }
    }

    #[test]
    fn validate_returns_error() {
        let err = validate_transition(Mode::Insert, Mode::Replace);
        assert!(err.is_err());
        let e = err.unwrap_err();
        assert!(matches!(e, TransitionError::InvalidTransition { .. }));
    }

    #[test]
    fn visual_modes_toggle() {
        assert!(can_transition(Mode::Visual, Mode::VisualLine));
        assert!(can_transition(Mode::VisualLine, Mode::VisualBlock));
        assert!(can_transition(Mode::VisualBlock, Mode::Visual));
    }
}
