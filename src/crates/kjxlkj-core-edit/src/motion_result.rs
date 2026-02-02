//! Motion result types.

use kjxlkj_core_types::Position;

/// Result of a motion execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionResult {
    /// New position.
    pub position: Position,
    /// Whether motion found a valid target.
    pub found: bool,
}

impl MotionResult {
    /// Creates a successful result.
    pub fn ok(position: Position) -> Self {
        Self {
            position,
            found: true,
        }
    }

    /// Creates a failed result (position unchanged).
    pub fn fail(position: Position) -> Self {
        Self {
            position,
            found: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_result_ok() {
        let r = MotionResult::ok(Position::new(5, 10));
        assert!(r.found);
        assert_eq!(r.position, Position::new(5, 10));
    }

    #[test]
    fn test_motion_result_fail() {
        let r = MotionResult::fail(Position::new(3, 4));
        assert!(!r.found);
        assert_eq!(r.position, Position::new(3, 4));
    }
}
