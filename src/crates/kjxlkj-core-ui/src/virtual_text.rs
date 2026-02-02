//! Virtual text state management.

use crate::virtual_text_types::{pos_key, VirtualText, VirtualTextPos};
use std::collections::HashMap;

/// Virtual text state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct VirtualTextState {
    /// Virtual texts by ID.
    texts: HashMap<usize, VirtualText>,
    /// Virtual texts by line.
    by_line: HashMap<usize, Vec<usize>>,
    /// Next ID.
    next_id: usize,
}

impl VirtualTextState {
    /// Creates new virtual text state.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Adds virtual text.
    pub fn add(&mut self, mut vt: VirtualText) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        vt.id = id;

        let line = vt.line;
        self.texts.insert(id, vt);
        self.by_line.entry(line).or_default().push(id);

        id
    }

    /// Removes virtual text by ID.
    pub fn remove(&mut self, id: usize) -> bool {
        if let Some(vt) = self.texts.remove(&id) {
            if let Some(ids) = self.by_line.get_mut(&vt.line) {
                ids.retain(|&i| i != id);
                if ids.is_empty() {
                    self.by_line.remove(&vt.line);
                }
            }
            true
        } else {
            false
        }
    }

    /// Gets virtual text by ID.
    pub fn get(&self, id: usize) -> Option<&VirtualText> {
        self.texts.get(&id)
    }

    /// Gets all virtual texts at a line.
    pub fn at_line(&self, line: usize) -> Vec<&VirtualText> {
        self.by_line
            .get(&line)
            .map(|ids| ids.iter().filter_map(|id| self.texts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets all virtual texts at a line in deterministic render order.
    pub fn ordered_at_line(&self, line: usize) -> Vec<&VirtualText> {
        let mut vts = self.at_line(line);
        vts.sort_by_key(|vt| (pos_key(vt.pos), vt.id));
        vts
    }

    /// Clears all virtual texts.
    pub fn clear(&mut self) {
        self.texts.clear();
        self.by_line.clear();
    }

    /// Returns count of virtual texts.
    pub fn len(&self) -> usize {
        self.texts.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_text_state_add() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));

        assert!(state.get(id).is_some());
        assert_eq!(state.at_line(10).len(), 1);
    }

    #[test]
    fn test_virtual_text_state_remove() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));

        assert!(state.remove(id));
        assert!(state.at_line(10).is_empty());
    }

    #[test]
    fn test_virtual_text_state_clear() {
        let mut state = VirtualTextState::new();
        state.add(VirtualText::eol(0, 10, "hint", "Hint"));
        state.add(VirtualText::eol(0, 20, "hint2", "Hint"));

        state.clear();
        assert!(state.is_empty());
    }

    #[test]
    fn test_virtual_text_state_remove_cleans_line() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));
        assert_eq!(state.at_line(10).len(), 1);
        assert!(state.remove(id));
        assert!(state.at_line(10).is_empty());
        assert!(state.by_line.get(&10).is_none());
    }

    #[test]
    fn test_virtual_text_state_ordered() {
        let mut state = VirtualTextState::new();
        state.add(VirtualText::inline(0, 10, 20, "b", "B"));
        state.add(VirtualText::inline(0, 10, 5, "a", "A"));
        state.add(VirtualText::eol(0, 10, "e", "E"));

        let ordered = state.ordered_at_line(10);
        assert_eq!(ordered.len(), 3);
        assert_eq!(ordered[0].pos, VirtualTextPos::Inline(5));
        assert_eq!(ordered[1].pos, VirtualTextPos::Inline(20));
        assert_eq!(ordered[2].pos, VirtualTextPos::EndOfLine);
    }

    #[test]
    fn test_virtual_text_state_len() {
        let mut state = VirtualTextState::new();
        assert_eq!(state.len(), 0);
        state.add(VirtualText::eol(0, 1, "a", "A"));
        assert_eq!(state.len(), 1);
    }
}
