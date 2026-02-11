//! Jump list and change list navigation.
//!
//! See /docs/spec/features/navigation/jumplist.md
//! and /docs/spec/features/navigation/changelist.md.

/// Position entry: line and column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

/// Cursor position history for jumplist and changelist.
#[derive(Debug, Clone)]
pub struct PositionList {
    entries: Vec<Position>,
    index: usize,
    cap: usize,
}

impl PositionList {
    pub fn new(cap: usize) -> Self {
        Self { entries: Vec::new(), index: 0, cap }
    }

    /// Push a new position. Entries after current index are discarded.
    pub fn push(&mut self, pos: Position) {
        // Avoid duplicate consecutive entries.
        if self.entries.last() == Some(&pos) && self.index == self.entries.len() {
            return;
        }
        // Truncate future entries if we're not at the end.
        if self.index < self.entries.len() {
            self.entries.truncate(self.index);
        }
        self.entries.push(pos);
        if self.entries.len() > self.cap {
            self.entries.remove(0);
        }
        self.index = self.entries.len();
    }

    /// Move to older entry. Returns the position if available.
    pub fn go_older(&mut self) -> Option<Position> {
        if self.index == 0 { return None; }
        self.index -= 1;
        self.entries.get(self.index).copied()
    }

    /// Move to newer entry. Returns the position if available.
    pub fn go_newer(&mut self) -> Option<Position> {
        if self.index + 1 >= self.entries.len() { return None; }
        self.index += 1;
        self.entries.get(self.index).copied()
    }

    /// Number of entries.
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    /// Current index in the list.
    pub fn current_index(&self) -> usize { self.index }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn push_and_go_older() {
        let mut jl = PositionList::new(100);
        jl.push(Position { line: 0, col: 0 });
        jl.push(Position { line: 10, col: 5 });
        jl.push(Position { line: 20, col: 3 });
        assert_eq!(jl.len(), 3);
        let p = jl.go_older().unwrap();
        assert_eq!(p, Position { line: 20, col: 3 });
        let p = jl.go_older().unwrap();
        assert_eq!(p, Position { line: 10, col: 5 });
        let p = jl.go_older().unwrap();
        assert_eq!(p, Position { line: 0, col: 0 });
        assert!(jl.go_older().is_none());
    }

    #[test] fn go_newer_after_older() {
        let mut jl = PositionList::new(100);
        jl.push(Position { line: 0, col: 0 });
        jl.push(Position { line: 10, col: 5 });
        jl.go_older();
        jl.go_older();
        let p = jl.go_newer().unwrap();
        assert_eq!(p, Position { line: 10, col: 5 });
        assert!(jl.go_newer().is_none());
    }

    #[test] fn push_truncates_future() {
        let mut jl = PositionList::new(100);
        jl.push(Position { line: 0, col: 0 });
        jl.push(Position { line: 10, col: 0 });
        jl.push(Position { line: 20, col: 0 });
        jl.go_older(); // index=2 (line 20)
        jl.go_older(); // index=1 (line 10)
        jl.push(Position { line: 30, col: 0 });
        assert_eq!(jl.len(), 2); // [0, 30]
        assert!(jl.go_newer().is_none());
    }

    #[test] fn capacity_cap() {
        let mut jl = PositionList::new(3);
        for i in 0..5 {
            jl.push(Position { line: i, col: 0 });
        }
        assert_eq!(jl.len(), 3);
        let p = jl.go_older().unwrap();
        assert_eq!(p.line, 4);
    }

    #[test] fn duplicate_consecutive_ignored() {
        let mut jl = PositionList::new(100);
        jl.push(Position { line: 5, col: 3 });
        jl.push(Position { line: 5, col: 3 });
        assert_eq!(jl.len(), 1);
    }

    #[test] fn empty_list_returns_none() {
        let mut jl = PositionList::new(100);
        assert!(jl.go_older().is_none());
        assert!(jl.go_newer().is_none());
    }
}
