//! Extended mark types: global marks, special marks, mark metadata.

use kjxlkj_core_types::{BufferId, Position};
use std::collections::HashMap;

/// Mark scope distinguishing local vs global marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkScope { Local, Global }

/// A stored mark with position and buffer association.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mark {
    pub buffer_id: BufferId,
    pub position: Position,
    pub scope: MarkScope,
}

impl Mark {
    pub fn local(buffer_id: BufferId, pos: Position) -> Self {
        Self { buffer_id, position: pos, scope: MarkScope::Local }
    }
    pub fn global(buffer_id: BufferId, pos: Position) -> Self {
        Self { buffer_id, position: pos, scope: MarkScope::Global }
    }
}

/// Classify a mark character into its scope.
pub fn mark_scope(c: char) -> Option<MarkScope> {
    match c {
        'a'..='z' => Some(MarkScope::Local),
        'A'..='Z' => Some(MarkScope::Global),
        '<' | '>' | '[' | ']' | '.' | '^' | '\'' | '`' => Some(MarkScope::Local),
        _ => None,
    }
}

/// Extended mark registry supporting both local and global marks.
pub struct MarkRegistry {
    local: HashMap<(BufferId, char), Position>,
    global: HashMap<char, (BufferId, Position)>,
}

impl MarkRegistry {
    pub fn new() -> Self { Self { local: HashMap::new(), global: HashMap::new() } }

    pub fn set(&mut self, c: char, buffer_id: BufferId, pos: Position) {
        match mark_scope(c) {
            Some(MarkScope::Local) => { self.local.insert((buffer_id, c), pos); }
            Some(MarkScope::Global) => { self.global.insert(c, (buffer_id, pos)); }
            None => {}
        }
    }

    pub fn get(&self, c: char, current_buf: BufferId) -> Option<Mark> {
        match mark_scope(c) {
            Some(MarkScope::Local) => {
                self.local.get(&(current_buf, c)).map(|p| Mark::local(current_buf, *p))
            }
            Some(MarkScope::Global) => {
                self.global.get(&c).map(|(bid, p)| Mark::global(*bid, *p))
            }
            None => None,
        }
    }

    pub fn delete(&mut self, c: char, current_buf: BufferId) -> bool {
        match mark_scope(c) {
            Some(MarkScope::Local) => self.local.remove(&(current_buf, c)).is_some(),
            Some(MarkScope::Global) => self.global.remove(&c).is_some(),
            None => false,
        }
    }

    /// Set special marks `[` and `]` to bracket a changed range.
    pub fn set_change_range(&mut self, bid: BufferId, start: Position, end: Position) {
        self.local.insert((bid, '['), start);
        self.local.insert((bid, ']'), end);
    }

    /// Set visual marks `<` and `>`.
    pub fn set_visual_range(&mut self, bid: BufferId, start: Position, end: Position) {
        self.local.insert((bid, '<'), start);
        self.local.insert((bid, '>'), end);
    }

    /// List all marks for display (`:marks` command).
    pub fn display(&self, current_buf: BufferId) -> Vec<(char, BufferId, Position)> {
        let mut out = Vec::new();
        for (&(bid, c), &pos) in &self.local {
            if bid == current_buf { out.push((c, bid, pos)); }
        }
        for (&c, &(bid, pos)) in &self.global { out.push((c, bid, pos)); }
        out.sort_by_key(|(c, _, _)| *c);
        out
    }

    pub fn local_count(&self, bid: BufferId) -> usize {
        self.local.keys().filter(|(b, _)| *b == bid).count()
    }
    pub fn global_count(&self) -> usize { self.global.len() }
}

impl Default for MarkRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn bid(n: u64) -> BufferId { BufferId(n) }
    fn pos(l: usize, c: usize) -> Position { Position::new(l, c) }

    #[test]
    fn local_mark() {
        let mut mr = MarkRegistry::new();
        mr.set('a', bid(1), pos(5, 3));
        let m = mr.get('a', bid(1)).unwrap();
        assert_eq!(m.scope, MarkScope::Local);
        assert_eq!(m.position, pos(5, 3));
        assert!(mr.get('a', bid(2)).is_none()); // different buffer
    }

    #[test]
    fn global_mark() {
        let mut mr = MarkRegistry::new();
        mr.set('A', bid(1), pos(10, 0));
        let m = mr.get('A', bid(2)).unwrap(); // any buffer
        assert_eq!(m.scope, MarkScope::Global);
        assert_eq!(m.buffer_id, bid(1));
    }

    #[test]
    fn special_marks() {
        let mut mr = MarkRegistry::new();
        mr.set_visual_range(bid(1), pos(2, 0), pos(5, 10));
        assert_eq!(mr.get('<', bid(1)).unwrap().position, pos(2, 0));
        assert_eq!(mr.get('>', bid(1)).unwrap().position, pos(5, 10));
    }

    #[test]
    fn change_range_marks() {
        let mut mr = MarkRegistry::new();
        mr.set_change_range(bid(1), pos(3, 0), pos(3, 5));
        assert_eq!(mr.get('[', bid(1)).unwrap().position, pos(3, 0));
        assert_eq!(mr.get(']', bid(1)).unwrap().position, pos(3, 5));
    }

    #[test]
    fn delete_mark() {
        let mut mr = MarkRegistry::new();
        mr.set('b', bid(1), pos(0, 0));
        assert!(mr.delete('b', bid(1)));
        assert!(mr.get('b', bid(1)).is_none());
    }

    #[test]
    fn display_lists_sorted() {
        let mut mr = MarkRegistry::new();
        mr.set('c', bid(1), pos(0, 0));
        mr.set('a', bid(1), pos(1, 0));
        mr.set('B', bid(2), pos(5, 0));
        let d = mr.display(bid(1));
        assert!(d.len() >= 2);
        assert!(d[0].0 <= d[1].0);
    }

    #[test]
    fn mark_scope_classification() {
        assert_eq!(mark_scope('a'), Some(MarkScope::Local));
        assert_eq!(mark_scope('Z'), Some(MarkScope::Global));
        assert_eq!(mark_scope('<'), Some(MarkScope::Local));
        assert_eq!(mark_scope('1'), None);
    }

    #[test]
    fn counts() {
        let mut mr = MarkRegistry::new();
        mr.set('a', bid(1), pos(0, 0));
        mr.set('b', bid(1), pos(1, 0));
        mr.set('A', bid(1), pos(2, 0));
        assert_eq!(mr.local_count(bid(1)), 2);
        assert_eq!(mr.global_count(), 1);
    }
}
