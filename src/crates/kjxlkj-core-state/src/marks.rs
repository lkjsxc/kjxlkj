//! Mark types and storage for kjxlkj editor.

use kjxlkj_core_types::{BufferId, MarkId, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A mark location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// Buffer the mark is in.
    pub buffer_id: BufferId,
    /// Position of the mark.
    pub position: Position,
}

impl Mark {
    /// Creates a new mark.
    pub fn new(buffer_id: BufferId, position: Position) -> Self {
        Self {
            buffer_id,
            position,
        }
    }
}

/// Storage for marks.
#[derive(Debug, Default)]
pub struct MarkStore {
    /// Local marks (a-z) per buffer.
    local: HashMap<BufferId, HashMap<char, Position>>,
    /// Global marks (A-Z).
    global: HashMap<char, Mark>,
    /// Special marks.
    special: SpecialMarks,
}

/// Special automatic marks.
#[derive(Debug, Default)]
pub struct SpecialMarks {
    /// Last jump position (used by '' and ``).
    pub last_jump: Option<Mark>,
    /// Last change position ('.').
    pub last_change: Option<Mark>,
    /// Last insert position ('^').
    pub last_insert: Option<Mark>,
    /// Start of last visual selection ('<').
    pub visual_start: Option<Mark>,
    /// End of last visual selection ('>').
    pub visual_end: Option<Mark>,
    /// Start of last change/yank ('[').
    pub change_start: Option<Mark>,
    /// End of last change/yank (']').
    pub change_end: Option<Mark>,
}

impl MarkStore {
    /// Creates a new mark store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a local mark in a buffer.
    pub fn set_local(&mut self, buffer_id: BufferId, name: char, position: Position) {
        if name.is_ascii_lowercase() {
            self.local
                .entry(buffer_id)
                .or_default()
                .insert(name, position);
        }
    }

    /// Gets a local mark from a buffer.
    pub fn get_local(&self, buffer_id: BufferId, name: char) -> Option<Position> {
        self.local
            .get(&buffer_id)
            .and_then(|m| m.get(&name))
            .copied()
    }

    /// Sets a global mark.
    pub fn set_global(&mut self, name: char, mark: Mark) {
        if name.is_ascii_uppercase() {
            self.global.insert(name, mark);
        }
    }

    /// Gets a global mark.
    pub fn get_global(&self, name: char) -> Option<Mark> {
        self.global.get(&name).copied()
    }

    /// Gets a mark by ID.
    pub fn get(&self, buffer_id: BufferId, mark_id: MarkId) -> Option<Mark> {
        match mark_id {
            MarkId::Local(c) => self
                .get_local(buffer_id, c)
                .map(|pos| Mark::new(buffer_id, pos)),
            MarkId::Global(c) => self.get_global(c),
            MarkId::LastJump => self.special.last_jump,
            MarkId::LastChange => self.special.last_change,
            MarkId::LastInsert => self.special.last_insert,
            MarkId::VisualStart => self.special.visual_start,
            MarkId::VisualEnd => self.special.visual_end,
            MarkId::ChangeStart => self.special.change_start,
            MarkId::ChangeEnd => self.special.change_end,
        }
    }

    /// Sets the last jump position.
    pub fn set_last_jump(&mut self, mark: Mark) {
        self.special.last_jump = Some(mark);
    }

    /// Sets the last change position.
    pub fn set_last_change(&mut self, mark: Mark) {
        self.special.last_change = Some(mark);
    }

    /// Sets the visual selection marks.
    pub fn set_visual_range(&mut self, start: Mark, end: Mark) {
        self.special.visual_start = Some(start);
        self.special.visual_end = Some(end);
    }

    /// Sets the change/yank range marks.
    pub fn set_change_range(&mut self, start: Mark, end: Mark) {
        self.special.change_start = Some(start);
        self.special.change_end = Some(end);
    }

    /// Removes all local marks for a buffer.
    pub fn clear_buffer(&mut self, buffer_id: BufferId) {
        self.local.remove(&buffer_id);
    }

    /// Returns all local marks for a buffer.
    pub fn local_marks(&self, buffer_id: BufferId) -> Vec<(char, Position)> {
        self.local
            .get(&buffer_id)
            .map(|m| m.iter().map(|(&c, &p)| (c, p)).collect())
            .unwrap_or_default()
    }

    /// Returns all global marks.
    pub fn global_marks(&self) -> Vec<(char, Mark)> {
        self.global.iter().map(|(&c, &m)| (c, m)).collect()
    }
}
