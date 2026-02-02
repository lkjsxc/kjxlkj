//! Jump location type.

use kjxlkj_core_types::{BufferId, Position};

/// A jump location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jump {
    /// Buffer ID.
    pub buffer: BufferId,
    /// Position in the buffer.
    pub position: Position,
}

impl Jump {
    /// Creates a new jump.
    pub fn new(buffer: BufferId, position: Position) -> Self {
        Self { buffer, position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buf(id: u64) -> BufferId {
        BufferId::new(id)
    }

    fn pos(line: usize, col: usize) -> Position {
        Position::new(line, col)
    }

    #[test]
    fn test_jump_new() {
        let j = Jump::new(buf(1), pos(10, 5));
        assert_eq!(j.buffer, buf(1));
        assert_eq!(j.position, pos(10, 5));
    }

    #[test]
    fn test_jump_clone() {
        let j1 = Jump::new(buf(1), pos(10, 5));
        let j2 = j1.clone();
        assert_eq!(j1, j2);
    }
}
