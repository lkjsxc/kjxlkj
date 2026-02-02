//! Change list for tracking edit positions.

use crate::jump::Jump;
use kjxlkj_core_types::{BufferId, Position};

/// Maximum change list size.
const MAX_CHANGES: usize = 100;

/// Change list for tracking edit positions.
#[derive(Debug, Default)]
pub struct ChangeList {
    /// List of change positions.
    changes: Vec<Jump>,
    /// Current position.
    index: usize,
}

impl ChangeList {
    /// Creates a new change list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a change at the given position.
    pub fn record(&mut self, buffer: BufferId, position: Position) {
        let jump = Jump::new(buffer, position);

        // Avoid duplicating recent changes.
        if let Some(last) = self.changes.last() {
            if last.buffer == jump.buffer && last.position == jump.position {
                return;
            }
        }

        self.changes.push(jump);
        self.index = self.changes.len();

        // Limit size.
        if self.changes.len() > MAX_CHANGES {
            self.changes.remove(0);
            self.index = self.changes.len();
        }
    }

    /// Goes to older change (g;).
    pub fn go_older(&mut self) -> Option<&Jump> {
        if self.index > 0 {
            self.index -= 1;
            self.changes.get(self.index)
        } else {
            None
        }
    }

    /// Goes to newer change (g,).
    pub fn go_newer(&mut self) -> Option<&Jump> {
        if self.index < self.changes.len() {
            let jump = self.changes.get(self.index);
            self.index += 1;
            jump
        } else {
            None
        }
    }

    /// Returns the most recent change.
    pub fn last(&self) -> Option<&Jump> {
        self.changes.last()
    }

    /// Returns all changes.
    pub fn changes(&self) -> &[Jump] {
        &self.changes
    }

    /// Clears the change list.
    pub fn clear(&mut self) {
        self.changes.clear();
        self.index = 0;
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
    fn test_changelist_record() {
        let mut cl = ChangeList::new();
        cl.record(buf(1), pos(5, 0));
        cl.record(buf(1), pos(10, 0));
        assert_eq!(cl.changes().len(), 2);
    }

    #[test]
    fn test_changelist_go_older() {
        let mut cl = ChangeList::new();
        cl.record(buf(1), pos(5, 0));
        cl.record(buf(1), pos(10, 0));

        let c = cl.go_older().unwrap();
        assert_eq!(c.position, pos(10, 0));
    }

    #[test]
    fn test_changelist_go_newer() {
        let mut cl = ChangeList::new();
        cl.record(buf(1), pos(5, 0));
        cl.record(buf(1), pos(10, 0));

        cl.go_older();
        cl.go_older();
        let c = cl.go_newer().unwrap();
        assert_eq!(c.position, pos(5, 0));
    }

    #[test]
    fn test_changelist_clear() {
        let mut cl = ChangeList::new();
        cl.record(buf(1), pos(5, 0));
        cl.clear();
        assert!(cl.changes().is_empty());
    }

    #[test]
    fn test_changelist_no_duplicate() {
        let mut cl = ChangeList::new();
        cl.record(buf(1), pos(5, 0));
        cl.record(buf(1), pos(5, 0));
        assert_eq!(cl.changes().len(), 1);
    }
}
