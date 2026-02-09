//! Mark system for remembering cursor positions.
//!
//! Supports buffer-local marks (a-z), global marks (A-Z),
//! and special automatic marks.

use std::collections::HashMap;

/// A mark position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkPosition {
    /// Buffer ID this mark belongs to.
    pub buffer_id: usize,
    /// Line number (0-indexed).
    pub line: usize,
    /// Column (0-indexed).
    pub col: usize,
}

/// The mark file storing all marks.
#[derive(Debug, Clone, Default)]
pub struct MarkFile {
    /// Buffer-local marks (a-z) per buffer.
    local: HashMap<usize, HashMap<char, MarkPosition>>,
    /// Global marks (A-Z, 0-9).
    global: HashMap<char, MarkPosition>,
    /// Special marks.
    special: HashMap<char, MarkPosition>,
}

impl MarkFile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a mark.
    pub fn set(&mut self, name: char, pos: MarkPosition) {
        match name {
            'a'..='z' => {
                self.local
                    .entry(pos.buffer_id)
                    .or_default()
                    .insert(name, pos);
            }
            'A'..='Z' | '0'..='9' => {
                self.global.insert(name, pos);
            }
            _ => {
                self.special.insert(name, pos);
            }
        }
    }

    /// Get a mark. For local marks, requires buffer_id context.
    pub fn get(&self, name: char, buffer_id: usize) -> Option<&MarkPosition> {
        match name {
            'a'..='z' => self.local.get(&buffer_id).and_then(|m| m.get(&name)),
            'A'..='Z' | '0'..='9' => self.global.get(&name),
            _ => self.special.get(&name),
        }
    }

    /// Delete a mark.
    pub fn delete(&mut self, name: char, buffer_id: usize) -> bool {
        match name {
            'a'..='z' => self
                .local
                .get_mut(&buffer_id)
                .map(|m| m.remove(&name).is_some())
                .unwrap_or(false),
            'A'..='Z' | '0'..='9' => self.global.remove(&name).is_some(),
            _ => self.special.remove(&name).is_some(),
        }
    }

    /// Set the last-change mark.
    pub fn set_last_change(&mut self, pos: MarkPosition) {
        self.special.insert('.', pos);
    }

    /// Set the jump-from mark.
    pub fn set_jump_from(&mut self, pos: MarkPosition) {
        self.special.insert('\'', pos);
    }

    /// Set the last insert position.
    pub fn set_last_insert(&mut self, pos: MarkPosition) {
        self.special.insert('^', pos);
    }

    /// Set visual selection start/end marks.
    pub fn set_visual_start(&mut self, pos: MarkPosition) {
        self.special.insert('<', pos);
    }

    pub fn set_visual_end(&mut self, pos: MarkPosition) {
        self.special.insert('>', pos);
    }

    /// Set start of last changed/yanked text.
    pub fn set_change_start(&mut self, pos: MarkPosition) {
        self.special.insert('[', pos);
    }

    /// Set end of last changed/yanked text.
    pub fn set_change_end(&mut self, pos: MarkPosition) {
        self.special.insert(']', pos);
    }

    /// List all marks for a buffer.
    pub fn list_for_buffer(&self, buffer_id: usize) -> Vec<(char, &MarkPosition)> {
        let mut result = Vec::new();

        // Local marks
        if let Some(locals) = self.local.get(&buffer_id) {
            let mut marks: Vec<_> = locals.iter().map(|(&c, p)| (c, p)).collect();
            marks.sort_by_key(|(c, _)| *c);
            result.extend(marks);
        }

        // Global marks
        let mut globals: Vec<_> = self.global.iter().map(|(&c, p)| (c, p)).collect();
        globals.sort_by_key(|(c, _)| *c);
        result.extend(globals);

        // Special marks
        let mut specials: Vec<_> = self.special.iter().map(|(&c, p)| (c, p)).collect();
        specials.sort_by_key(|(c, _)| *c);
        result.extend(specials);

        result
    }

    /// Clear all local marks for a buffer.
    pub fn clear_buffer(&mut self, buffer_id: usize) {
        self.local.remove(&buffer_id);
    }

    /// Adjust marks after lines are inserted or deleted in a buffer.
    pub fn adjust_for_edit(&mut self, buffer_id: usize, start_line: usize, lines_added: isize) {
        // Adjust local marks
        if let Some(locals) = self.local.get_mut(&buffer_id) {
            for pos in locals.values_mut() {
                if pos.line >= start_line {
                    if lines_added < 0 {
                        let removed = (-lines_added) as usize;
                        if pos.line < start_line + removed {
                            pos.line = start_line;
                            pos.col = 0;
                        } else {
                            pos.line -= removed;
                        }
                    } else {
                        pos.line += lines_added as usize;
                    }
                }
            }
        }

        // Adjust global marks in same buffer
        for pos in self.global.values_mut() {
            if pos.buffer_id == buffer_id && pos.line >= start_line {
                if lines_added < 0 {
                    let removed = (-lines_added) as usize;
                    if pos.line < start_line + removed {
                        pos.line = start_line;
                        pos.col = 0;
                    } else {
                        pos.line -= removed;
                    }
                } else {
                    pos.line += lines_added as usize;
                }
            }
        }
    }
}
