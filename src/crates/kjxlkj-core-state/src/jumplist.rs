//! Jump list for navigation history.
//!
//! Tracks positions visited during navigation (searches, jumps, etc.)
//! for Ctrl-O (back) and Ctrl-I (forward) navigation.

use crate::jump::Jump;

/// Maximum jump list size.
const MAX_JUMPS: usize = 100;

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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, Position};

    fn buf(id: u64) -> BufferId {
        BufferId::new(id)
    }

    fn pos(line: usize, col: usize) -> Position {
        Position::new(line, col)
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
    fn test_jumplist_clear() {
        let mut jl = JumpList::new();
        jl.push(Jump::new(buf(1), pos(0, 0)));
        jl.clear();
        assert!(jl.is_empty());
    }
}
