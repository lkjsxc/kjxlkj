//! Jump list: tracks jump positions for Ctrl-O / Ctrl-I navigation.

use kjxlkj_core_types::{BufferId, Position};

/// A single entry in the jump list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpEntry {
    pub buffer_id: BufferId,
    pub position: Position,
}

/// The jump list with forward/backward navigation.
#[derive(Debug, Clone)]
pub struct JumpList {
    entries: Vec<JumpEntry>,
    current: usize,
}

impl JumpList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

    /// Add a new jump entry, truncating any forward history.
    pub fn push(&mut self, entry: JumpEntry) {
        self.entries.truncate(self.current);
        self.entries.push(entry);
        self.current = self.entries.len();
    }

    /// Jump backward in the list (Ctrl-O).
    pub fn jump_back(&mut self) -> Option<&JumpEntry> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        self.entries.get(self.current)
    }

    /// Jump forward in the list (Ctrl-I).
    pub fn jump_forward(&mut self) -> Option<&JumpEntry> {
        if self.current >= self.entries.len() {
            return None;
        }
        let entry = self.entries.get(self.current);
        self.current += 1;
        entry
    }

    pub fn entries(&self) -> &[JumpEntry] {
        &self.entries
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for JumpList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_back() {
        let mut jl = JumpList::new();
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(0, 0),
        });
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(5, 0),
        });
        let e = jl.jump_back().unwrap();
        assert_eq!(e.position, Position::new(5, 0));
    }

    #[test]
    fn forward_after_back() {
        let mut jl = JumpList::new();
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(0, 0),
        });
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(5, 0),
        });
        jl.jump_back();
        let e = jl.jump_forward().unwrap();
        assert_eq!(e.position, Position::new(5, 0));
    }

    #[test]
    fn truncates_forward() {
        let mut jl = JumpList::new();
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(0, 0),
        });
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(5, 0),
        });
        jl.jump_back();
        jl.push(JumpEntry {
            buffer_id: BufferId(1),
            position: Position::new(10, 0),
        });
        assert_eq!(jl.len(), 2);
    }
}
