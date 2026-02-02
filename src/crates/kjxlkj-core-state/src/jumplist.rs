//! Jump list for navigation history.
//!
//! Tracks positions visited during navigation (searches, jumps, etc.)
//! for Ctrl-O (back) and Ctrl-I (forward) navigation.

use kjxlkj_core_types::{BufferId, Position};

/// Maximum jump list size.
const MAX_JUMPS: usize = 100;

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

/// Jump list for navigation history.
#[derive(Debug, Default)]
pub struct JumpList {
    /// List of jumps.
    jumps: Vec<Jump>,
    /// Current position in the list.
    index: usize,
}

impl JumpList {
    /// Creates a new empty jump list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a jump to the list.
    pub fn push(&mut self, jump: Jump) {
        // Remove duplicates at the current position.
        if let Some(current) = self.jumps.get(self.index) {
            if current.buffer == jump.buffer && current.position == jump.position {
                return;
            }
        }

        // If we're not at the end, remove future entries.
        if self.index < self.jumps.len() {
            self.jumps.truncate(self.index);
        }

        // Add the new jump.
        self.jumps.push(jump);
        self.index = self.jumps.len();

        // Limit size.
        if self.jumps.len() > MAX_JUMPS {
            let excess = self.jumps.len() - MAX_JUMPS;
            self.jumps.drain(0..excess);
            self.index = self.index.saturating_sub(excess);
        }
    }

    /// Goes back in the jump list (Ctrl-O).
    pub fn go_back(&mut self) -> Option<&Jump> {
        if self.index > 0 {
            self.index -= 1;
            self.jumps.get(self.index)
        } else {
            None
        }
    }

    /// Goes forward in the jump list (Ctrl-I).
    pub fn go_forward(&mut self) -> Option<&Jump> {
        if self.index < self.jumps.len() {
            let jump = self.jumps.get(self.index);
            self.index += 1;
            jump
        } else {
            None
        }
    }

    /// Returns the current jump.
    pub fn current(&self) -> Option<&Jump> {
        if self.index > 0 && self.index <= self.jumps.len() {
            self.jumps.get(self.index - 1)
        } else {
            self.jumps.first()
        }
    }

    /// Returns all jumps.
    pub fn jumps(&self) -> &[Jump] {
        &self.jumps
    }

    /// Returns the number of jumps.
    pub fn len(&self) -> usize {
        self.jumps.len()
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.jumps.is_empty()
    }

    /// Clears the jump list.
    pub fn clear(&mut self) {
        self.jumps.clear();
        self.index = 0;
    }
}

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
        if self.changes.len() > MAX_JUMPS {
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
    fn test_jump_new() {
        let j = Jump::new(buf(1), pos(10, 5));
        assert_eq!(j.buffer, buf(1));
        assert_eq!(j.position, pos(10, 5));
    }

    #[test]
    fn test_jumplist_push() {
        let mut jl = JumpList::new();
        jl.push(Jump::new(buf(1), pos(0, 0)));
        jl.push(Jump::new(buf(1), pos(10, 0)));
        assert_eq!(jl.len(), 2);
    }

    #[test]
    fn test_jumplist_go_back() {
        let mut jl = JumpList::new();
        jl.push(Jump::new(buf(1), pos(0, 0)));
        jl.push(Jump::new(buf(1), pos(10, 0)));
        jl.push(Jump::new(buf(1), pos(20, 0)));

        let jump = jl.go_back().unwrap();
        assert_eq!(jump.position, pos(20, 0));
    }

    #[test]
    fn test_jumplist_go_forward() {
        let mut jl = JumpList::new();
        jl.push(Jump::new(buf(1), pos(0, 0)));
        jl.push(Jump::new(buf(1), pos(10, 0)));
        jl.push(Jump::new(buf(1), pos(20, 0)));

        jl.go_back();
        jl.go_back();
        let jump = jl.go_forward().unwrap();
        assert_eq!(jump.position, pos(10, 0));
    }

    #[test]
    fn test_jumplist_truncate_on_new_jump() {
        let mut jl = JumpList::new();
        jl.push(Jump::new(buf(1), pos(0, 0)));
        jl.push(Jump::new(buf(1), pos(10, 0)));
        jl.push(Jump::new(buf(1), pos(20, 0)));

        jl.go_back();
        jl.go_back();
        jl.push(Jump::new(buf(1), pos(5, 0)));
        assert_eq!(jl.len(), 2);
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
}
