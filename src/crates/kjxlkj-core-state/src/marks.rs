//! Mark system for remembering cursor positions.
//!
//! Supports buffer-local marks (a-z), global marks (A-Z),
//! and special automatic marks.

use std::collections::HashMap;

/// A mark position with timestamp for conflict resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkPosition {
    pub buffer_id: usize,
    pub line: usize,
    pub col: usize,
    /// Monotonic counter for viminfo merge conflict resolution.
    pub timestamp: u64,
}
impl MarkPosition {
    pub fn new(buffer_id: usize, line: usize, col: usize) -> Self { Self { buffer_id, line, col, timestamp: 0 } }
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
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, name: char, pos: MarkPosition) {
        match name {
            'a'..='z' => { self.local.entry(pos.buffer_id).or_default().insert(name, pos); }
            'A'..='Z' | '0'..='9' => { self.global.insert(name, pos); }
            _ => { self.special.insert(name, pos); }
        }
    }

    pub fn get(&self, name: char, buffer_id: usize) -> Option<&MarkPosition> {
        match name {
            'a'..='z' => self.local.get(&buffer_id).and_then(|m| m.get(&name)),
            'A'..='Z' | '0'..='9' => self.global.get(&name),
            _ => self.special.get(&name),
        }
    }

    pub fn delete(&mut self, name: char, buffer_id: usize) -> bool {
        match name {
            'a'..='z' => self.local.get_mut(&buffer_id).map(|m| m.remove(&name).is_some()).unwrap_or(false),
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

    /// Rotate numbered marks: 8→9, 7→8, ..., 0→1, then set 0 to new pos.
    pub fn rotate_numbered(&mut self, pos: MarkPosition) {
        for i in (0..9).rev() {
            let src = char::from(b'0' + i);
            let dst = char::from(b'0' + i + 1);
            if let Some(p) = self.global.get(&src).copied() { self.global.insert(dst, p); }
        }
        self.global.insert('0', pos);
    }

    /// Serialize global+numbered marks to viminfo format (with timestamps).
    pub fn serialize_viminfo(&self) -> String {
        let mut out = String::from("# Viminfo Marks\n");
        for (&name, pos) in &self.global {
            out.push_str(&format!("'{}  {}  {}  {}  {}\n", name, pos.buffer_id, pos.line, pos.col, pos.timestamp));
        }
        out
    }
    /// Load marks from viminfo format string. Newer timestamps win on conflict.
    pub fn load_viminfo(&mut self, input: &str) {
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if line.starts_with('\'') && line.len() >= 3 {
                let ch = line.as_bytes()[1] as char;
                let parts: Vec<&str> = line[2..].split_whitespace().collect();
                if parts.len() >= 3 {
                    if let (Ok(bid), Ok(l), Ok(c)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
                        let ts: u64 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
                        let existing_ts = self.global.get(&ch).map(|p| p.timestamp).unwrap_or(0);
                        if ts >= existing_ts { self.global.insert(ch, MarkPosition { buffer_id: bid, line: l, col: c, timestamp: ts }); }
                    }
                }
            }
        }
    }
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
