//! Mark management: local, global, and special marks.

use kjxlkj_core_types::{BufferId, Position};
use std::collections::HashMap;

/// A single mark entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkEntry {
    pub buffer_id: BufferId,
    pub position: Position,
}

/// Whether a mark is local or global.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkScope {
    Local,
    Global,
}

/// Determine scope from mark character.
pub fn mark_scope(ch: char) -> MarkScope {
    if ch.is_ascii_uppercase() || ch.is_ascii_digit() {
        MarkScope::Global
    } else {
        MarkScope::Local
    }
}

/// Set a mark in a flat map.
pub fn set_mark(
    marks: &mut HashMap<char, MarkEntry>,
    ch: char,
    buffer_id: BufferId,
    pos: Position,
) {
    marks.insert(
        ch,
        MarkEntry {
            buffer_id,
            position: pos,
        },
    );
}

/// Get a mark from a flat map.
pub fn get_mark(marks: &HashMap<char, MarkEntry>, ch: char) -> Option<&MarkEntry> {
    marks.get(&ch)
}

/// Extended mark registry with local and global separation.
#[derive(Debug, Clone)]
pub struct MarkRegistry {
    pub local: HashMap<BufferId, HashMap<char, Position>>,
    pub global: HashMap<char, MarkEntry>,
    pub special: HashMap<char, MarkEntry>,
}

impl MarkRegistry {
    pub fn new() -> Self {
        Self {
            local: HashMap::new(),
            global: HashMap::new(),
            special: HashMap::new(),
        }
    }

    pub fn set(&mut self, ch: char, buffer_id: BufferId, pos: Position) {
        match mark_scope(ch) {
            MarkScope::Global => {
                self.global.insert(
                    ch,
                    MarkEntry {
                        buffer_id,
                        position: pos,
                    },
                );
            }
            MarkScope::Local => {
                self.local.entry(buffer_id).or_default().insert(ch, pos);
            }
        }
    }

    pub fn get(&self, ch: char, current_buffer: BufferId) -> Option<MarkEntry> {
        match mark_scope(ch) {
            MarkScope::Global => self.global.get(&ch).cloned(),
            MarkScope::Local => self
                .local
                .get(&current_buffer)
                .and_then(|m| m.get(&ch))
                .map(|pos| MarkEntry {
                    buffer_id: current_buffer,
                    position: *pos,
                }),
        }
    }

    pub fn set_special(&mut self, ch: char, buffer_id: BufferId, pos: Position) {
        self.special.insert(
            ch,
            MarkEntry {
                buffer_id,
                position: pos,
            },
        );
    }
}

impl Default for MarkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_mark() {
        let mut marks = HashMap::new();
        set_mark(&mut marks, 'a', BufferId(1), Position::new(5, 3));
        let m = get_mark(&marks, 'a').unwrap();
        assert_eq!(m.position, Position::new(5, 3));
    }

    #[test]
    fn mark_scope_test() {
        assert_eq!(mark_scope('a'), MarkScope::Local);
        assert_eq!(mark_scope('A'), MarkScope::Global);
        assert_eq!(mark_scope('0'), MarkScope::Global);
    }

    #[test]
    fn registry_local_global() {
        let mut reg = MarkRegistry::new();
        reg.set('a', BufferId(1), Position::new(1, 0));
        reg.set('A', BufferId(1), Position::new(2, 0));
        assert!(reg.get('a', BufferId(1)).is_some());
        assert!(reg.get('a', BufferId(2)).is_none());
        assert!(reg.get('A', BufferId(2)).is_some());
    }
}
