//! Mark storage and retrieval.

use kjxlkj_core_types::{BufferId, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A mark's position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// Buffer the mark is in.
    pub buffer: BufferId,
    /// Position in the buffer.
    pub position: Position,
}

impl Mark {
    /// Creates a new mark.
    pub fn new(buffer: BufferId, position: Position) -> Self {
        Self { buffer, position }
    }
}

/// Mark type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    /// Local mark (a-z) - per buffer.
    Local,
    /// Global mark (A-Z) - across buffers.
    Global,
    /// Special mark ('.^<> etc).
    Special,
}

impl MarkType {
    /// Returns the type for a mark character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' => Some(Self::Local),
            'A'..='Z' => Some(Self::Global),
            '.' | '^' | '\'' | '`' | '<' | '>' | '[' | ']' | '"' => Some(Self::Special),
            _ => None,
        }
    }
}

/// Storage for marks.
#[derive(Debug, Clone, Default)]
pub struct MarkStore {
    /// Local marks per buffer.
    local: HashMap<BufferId, HashMap<char, Position>>,
    /// Global marks.
    global: HashMap<char, Mark>,
    /// Jump list position.
    last_jump: Option<Mark>,
    /// Last change position.
    last_change: Option<Position>,
    /// Last insert position.
    last_insert: Option<Position>,
}

impl MarkStore {
    /// Creates a new mark store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a mark.
    pub fn set(&mut self, c: char, buffer: BufferId, position: Position) {
        match MarkType::from_char(c) {
            Some(MarkType::Local) => {
                self.local
                    .entry(buffer)
                    .or_default()
                    .insert(c, position);
            }
            Some(MarkType::Global) => {
                self.global.insert(c, Mark::new(buffer, position));
            }
            Some(MarkType::Special) => {
                match c {
                    '.' => self.last_change = Some(position),
                    '^' => self.last_insert = Some(position),
                    '\'' | '`' => self.last_jump = Some(Mark::new(buffer, position)),
                    _ => {}
                }
            }
            None => {}
        }
    }

    /// Gets a mark in a specific buffer.
    pub fn get(&self, c: char, buffer: BufferId) -> Option<Mark> {
        match MarkType::from_char(c)? {
            MarkType::Local => {
                let pos = self.local.get(&buffer)?.get(&c)?;
                Some(Mark::new(buffer, *pos))
            }
            MarkType::Global => self.global.get(&c).copied(),
            MarkType::Special => {
                match c {
                    '.' => self.last_change.map(|p| Mark::new(buffer, p)),
                    '^' => self.last_insert.map(|p| Mark::new(buffer, p)),
                    '\'' | '`' => self.last_jump,
                    _ => None,
                }
            }
        }
    }

    /// Deletes a mark.
    pub fn delete(&mut self, c: char, buffer: BufferId) {
        match MarkType::from_char(c) {
            Some(MarkType::Local) => {
                if let Some(marks) = self.local.get_mut(&buffer) {
                    marks.remove(&c);
                }
            }
            Some(MarkType::Global) => {
                self.global.remove(&c);
            }
            _ => {}
        }
    }

    /// Lists all marks for a buffer.
    pub fn list(&self, buffer: BufferId) -> Vec<(char, Mark)> {
        let mut result = Vec::new();
        
        // Local marks
        if let Some(marks) = self.local.get(&buffer) {
            for (c, pos) in marks {
                result.push((*c, Mark::new(buffer, *pos)));
            }
        }
        
        // Global marks
        for (c, mark) in &self.global {
            result.push((*c, *mark));
        }
        
        // Special marks
        if let Some(pos) = self.last_change {
            result.push(('.', Mark::new(buffer, pos)));
        }
        if let Some(pos) = self.last_insert {
            result.push(('^', Mark::new(buffer, pos)));
        }
        if let Some(mark) = self.last_jump {
            result.push(('\'', mark));
        }
        
        result.sort_by_key(|(c, _)| *c);
        result
    }

    /// Updates the last change position.
    pub fn set_last_change(&mut self, buffer: BufferId, position: Position) {
        self.set('.', buffer, position);
    }

    /// Updates the last insert position.
    pub fn set_last_insert(&mut self, buffer: BufferId, position: Position) {
        self.set('^', buffer, position);
    }

    /// Updates the last jump position.
    pub fn set_last_jump(&mut self, buffer: BufferId, position: Position) {
        self.last_jump = Some(Mark::new(buffer, position));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buf() -> BufferId {
        BufferId::new(1)
    }

    #[test]
    fn test_mark_type() {
        assert_eq!(MarkType::from_char('a'), Some(MarkType::Local));
        assert_eq!(MarkType::from_char('Z'), Some(MarkType::Global));
        assert_eq!(MarkType::from_char('.'), Some(MarkType::Special));
        assert_eq!(MarkType::from_char('0'), None);
    }

    #[test]
    fn test_local_mark() {
        let mut store = MarkStore::new();
        let buffer = buf();
        store.set('a', buffer, Position::new(5, 10));
        
        let mark = store.get('a', buffer).unwrap();
        assert_eq!(mark.position.line, 5);
        assert_eq!(mark.position.col, 10);
    }

    #[test]
    fn test_global_mark() {
        let mut store = MarkStore::new();
        let buffer1 = BufferId::new(1);
        let buffer2 = BufferId::new(2);
        store.set('A', buffer1, Position::new(10, 0));
        
        // Should be visible from any buffer
        let mark = store.get('A', buffer2).unwrap();
        assert_eq!(mark.buffer, buffer1);
    }

    #[test]
    fn test_special_marks() {
        let mut store = MarkStore::new();
        let buffer = buf();
        store.set_last_change(buffer, Position::new(3, 5));
        store.set_last_insert(buffer, Position::new(7, 2));
        
        let change = store.get('.', buffer).unwrap();
        assert_eq!(change.position.line, 3);
        
        let insert = store.get('^', buffer).unwrap();
        assert_eq!(insert.position.line, 7);
    }

    #[test]
    fn test_delete_mark() {
        let mut store = MarkStore::new();
        let buffer = buf();
        store.set('b', buffer, Position::new(1, 1));
        assert!(store.get('b', buffer).is_some());
        
        store.delete('b', buffer);
        assert!(store.get('b', buffer).is_none());
    }

    #[test]
    fn test_list_marks() {
        let mut store = MarkStore::new();
        let buffer = buf();
        store.set('a', buffer, Position::new(1, 0));
        store.set('b', buffer, Position::new(2, 0));
        store.set('A', buffer, Position::new(3, 0));
        
        let marks = store.list(buffer);
        assert!(marks.len() >= 3);
    }
}
