//! Multiple cursor support.
//!
//! Provides simultaneous editing at multiple positions.

use std::collections::BTreeSet;

/// A cursor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorPosition {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed).
    pub col: usize,
}

impl CursorPosition {
    /// Create a new position.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Get byte offset in text.
    pub fn to_offset(&self, line_offsets: &[usize]) -> usize {
        if self.line >= line_offsets.len() {
            return line_offsets.last().copied().unwrap_or(0);
        }
        line_offsets[self.line] + self.col
    }
}

/// Selection range for a cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Anchor position (start of selection).
    pub anchor: CursorPosition,
    /// Head position (cursor position).
    pub head: CursorPosition,
}

impl Selection {
    /// Create a new selection.
    pub fn new(anchor: CursorPosition, head: CursorPosition) -> Self {
        Self { anchor, head }
    }

    /// Create a point selection (no range).
    pub fn point(pos: CursorPosition) -> Self {
        Self { anchor: pos, head: pos }
    }

    /// Get normalized range (start <= end).
    pub fn range(&self) -> (CursorPosition, CursorPosition) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    /// Check if selection is empty (point).
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// Check if selections overlap.
    pub fn overlaps(&self, other: &Selection) -> bool {
        let (s1, e1) = self.range();
        let (s2, e2) = other.range();
        !(e1 < s2 || e2 < s1)
    }

    /// Merge with another selection (assumes overlap).
    pub fn merge(&self, other: &Selection) -> Self {
        let (s1, e1) = self.range();
        let (s2, e2) = other.range();
        let start = std::cmp::min(s1, s2);
        let end = std::cmp::max(e1, e2);
        Selection::new(start, end)
    }
}

/// A cursor with optional selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    /// Cursor position.
    pub position: CursorPosition,
    /// Selection (None = no selection).
    pub selection: Option<Selection>,
    /// Is this the primary cursor?
    pub is_primary: bool,
}

impl Cursor {
    /// Create a new cursor.
    pub fn new(position: CursorPosition) -> Self {
        Self {
            position,
            selection: None,
            is_primary: false,
        }
    }

    /// Create a primary cursor.
    pub fn primary(position: CursorPosition) -> Self {
        Self {
            position,
            selection: None,
            is_primary: true,
        }
    }

    /// Set selection anchor at current position.
    pub fn start_selection(&mut self) {
        self.selection = Some(Selection::point(self.position));
    }

    /// Extend selection to current position.
    pub fn extend_selection(&mut self) {
        if let Some(ref mut sel) = self.selection {
            sel.head = self.position;
        } else {
            self.start_selection();
        }
    }

    /// Clear selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Get selection range if any.
    pub fn selection_range(&self) -> Option<(CursorPosition, CursorPosition)> {
        self.selection.as_ref().map(|s| s.range())
    }
}

/// Edit operation to apply at each cursor.
#[derive(Debug, Clone)]
pub enum CursorEdit {
    /// Insert text at cursor.
    Insert(String),
    /// Delete selection or count chars.
    Delete { count: usize },
    /// Delete backwards.
    Backspace { count: usize },
    /// Replace selection with text.
    Replace(String),
}

/// Result of applying an edit to multiple cursors.
#[derive(Debug, Clone)]
pub struct MultiEditResult {
    /// Edits to apply, sorted by position (reverse order for safety).
    pub edits: Vec<(usize, usize, String)>, // (start, end, replacement)
    /// New cursor positions after edit.
    pub new_cursors: Vec<CursorPosition>,
}

/// Multiple cursor state.
#[derive(Debug, Clone)]
pub struct MultiCursor {
    /// All cursors (sorted by position).
    cursors: Vec<Cursor>,
    /// Primary cursor index.
    primary_index: usize,
}

impl Default for MultiCursor {
    fn default() -> Self {
        Self::new(CursorPosition::new(0, 0))
    }
}

impl MultiCursor {
    /// Create with a single primary cursor.
    pub fn new(position: CursorPosition) -> Self {
        Self {
            cursors: vec![Cursor::primary(position)],
            primary_index: 0,
        }
    }

    /// Get primary cursor.
    pub fn primary(&self) -> &Cursor {
        &self.cursors[self.primary_index]
    }

    /// Get mutable primary cursor.
    pub fn primary_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[self.primary_index]
    }

    /// Get all cursors.
    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }

    /// Get cursor count.
    pub fn count(&self) -> usize {
        self.cursors.len()
    }

    /// Whether there are multiple cursors.
    pub fn is_multi(&self) -> bool {
        self.cursors.len() > 1
    }

    /// Add a cursor at position.
    pub fn add(&mut self, position: CursorPosition) {
        // Check for duplicate
        if self.cursors.iter().any(|c| c.position == position) {
            return;
        }

        self.cursors.push(Cursor::new(position));
        self.sort_cursors();
    }

    /// Add cursor at next occurrence of selection.
    pub fn add_next_match(&mut self, text: &str, search: &str) -> bool {
        if search.is_empty() {
            return false;
        }

        // Find all occurrences
        let mut occurrences: BTreeSet<usize> = BTreeSet::new();
        for (offset, _) in text.match_indices(search) {
            occurrences.insert(offset);
        }

        if occurrences.is_empty() {
            return false;
        }

        // Find current cursor offset
        let line_offsets = Self::compute_line_offsets(text);
        let current_offset = self.primary().position.to_offset(&line_offsets);

        // Find next occurrence after current cursor
        let next = occurrences
            .iter()
            .find(|&&off| off > current_offset)
            .or_else(|| occurrences.first())
            .copied();

        if let Some(offset) = next {
            let pos = Self::offset_to_position(offset, &line_offsets);
            self.add(pos);
            return true;
        }

        false
    }

    /// Skip current match and find next.
    pub fn skip_next_match(&mut self, text: &str, search: &str) -> bool {
        // Move primary forward, then add next match
        if let Some(new_primary) = self.cursors.iter().position(|c| !c.is_primary) {
            self.cursors.remove(self.primary_index);
            self.primary_index = new_primary.min(self.cursors.len() - 1);
            self.cursors[self.primary_index].is_primary = true;
            return self.add_next_match(text, search);
        }
        false
    }

    /// Add cursors at all occurrences.
    pub fn add_all_matches(&mut self, text: &str, search: &str) {
        if search.is_empty() {
            return;
        }

        let line_offsets = Self::compute_line_offsets(text);

        for (offset, _) in text.match_indices(search) {
            let pos = Self::offset_to_position(offset, &line_offsets);
            self.add(pos);
        }
    }

    /// Add cursor above.
    pub fn add_cursor_above(&mut self) {
        let primary = self.primary().position;
        if primary.line > 0 {
            let pos = CursorPosition::new(primary.line - 1, primary.col);
            self.add(pos);
        }
    }

    /// Add cursor below.
    pub fn add_cursor_below(&mut self, max_line: usize) {
        let primary = self.primary().position;
        if primary.line < max_line {
            let pos = CursorPosition::new(primary.line + 1, primary.col);
            self.add(pos);
        }
    }

    /// Remove cursor at position.
    pub fn remove(&mut self, position: CursorPosition) -> bool {
        // Can't remove last cursor
        if self.cursors.len() <= 1 {
            return false;
        }

        if let Some(idx) = self.cursors.iter().position(|c| c.position == position) {
            let was_primary = self.cursors[idx].is_primary;
            self.cursors.remove(idx);

            if was_primary {
                self.primary_index = 0;
                self.cursors[0].is_primary = true;
            } else if idx <= self.primary_index && self.primary_index > 0 {
                self.primary_index -= 1;
            }

            return true;
        }

        false
    }

    /// Clear all secondary cursors.
    pub fn clear_secondary(&mut self) {
        let primary = self.cursors.remove(self.primary_index);
        self.cursors = vec![primary];
        self.primary_index = 0;
    }

    /// Move all cursors.
    pub fn move_all<F>(&mut self, f: F)
    where
        F: Fn(&CursorPosition) -> CursorPosition,
    {
        for cursor in &mut self.cursors {
            cursor.position = f(&cursor.position);
            if let Some(ref mut sel) = cursor.selection {
                sel.head = cursor.position;
            }
        }
        self.merge_overlapping();
    }

    /// Start selection on all cursors.
    pub fn start_selection_all(&mut self) {
        for cursor in &mut self.cursors {
            cursor.start_selection();
        }
    }

    /// Clear selection on all cursors.
    pub fn clear_selection_all(&mut self) {
        for cursor in &mut self.cursors {
            cursor.clear_selection();
        }
    }

    /// Prepare edit operation for all cursors.
    pub fn prepare_edit(
        &self,
        edit: &CursorEdit,
        text: &str,
    ) -> MultiEditResult {
        let line_offsets = Self::compute_line_offsets(text);
        let mut edits: Vec<(usize, usize, String)> = Vec::new();
        let mut new_positions: Vec<CursorPosition> = Vec::new();

        // Process cursors in reverse order (back to front)
        for cursor in self.cursors.iter().rev() {
            let offset = cursor.position.to_offset(&line_offsets);

            match edit {
                CursorEdit::Insert(s) => {
                    edits.push((offset, offset, s.clone()));
                    new_positions.push(CursorPosition::new(
                        cursor.position.line,
                        cursor.position.col + s.len(),
                    ));
                }
                CursorEdit::Delete { count } => {
                    let end = (offset + count).min(text.len());
                    edits.push((offset, end, String::new()));
                    new_positions.push(cursor.position);
                }
                CursorEdit::Backspace { count } => {
                    let start = offset.saturating_sub(*count);
                    edits.push((start, offset, String::new()));
                    let new_col = cursor.position.col.saturating_sub(*count);
                    new_positions.push(CursorPosition::new(cursor.position.line, new_col));
                }
                CursorEdit::Replace(s) => {
                    if let Some((start, end)) = cursor.selection_range() {
                        let start_off = start.to_offset(&line_offsets);
                        let end_off = end.to_offset(&line_offsets);
                        edits.push((start_off, end_off, s.clone()));
                        new_positions.push(CursorPosition::new(start.line, start.col + s.len()));
                    } else {
                        edits.push((offset, offset, s.clone()));
                        new_positions.push(CursorPosition::new(
                            cursor.position.line,
                            cursor.position.col + s.len(),
                        ));
                    }
                }
            }
        }

        // Reverse to get correct order for new_cursors
        new_positions.reverse();

        MultiEditResult {
            edits,
            new_cursors: new_positions,
        }
    }

    /// Apply edit result positions.
    pub fn apply_positions(&mut self, positions: Vec<CursorPosition>) {
        for (cursor, pos) in self.cursors.iter_mut().zip(positions) {
            cursor.position = pos;
            cursor.selection = None;
        }
        self.merge_overlapping();
    }

    fn sort_cursors(&mut self) {
        // Remember primary
        let primary_pos = self.primary().position;

        // Sort
        self.cursors.sort_by(|a, b| a.position.cmp(&b.position));

        // Find new primary index
        self.primary_index = self
            .cursors
            .iter()
            .position(|c| c.position == primary_pos || c.is_primary)
            .unwrap_or(0);
    }

    fn merge_overlapping(&mut self) {
        if self.cursors.len() <= 1 {
            return;
        }

        // Sort first
        self.sort_cursors();

        // Merge overlapping
        let mut merged = Vec::new();
        let mut i = 0;

        while i < self.cursors.len() {
            let mut current = self.cursors[i].clone();
            let mut j = i + 1;

            while j < self.cursors.len() {
                if current.position == self.cursors[j].position {
                    // Merge: keep primary if either is primary
                    if self.cursors[j].is_primary {
                        current.is_primary = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }

            merged.push(current);
            i = j;
        }

        self.cursors = merged;

        // Ensure primary exists
        if !self.cursors.iter().any(|c| c.is_primary) {
            self.cursors[0].is_primary = true;
        }
        self.primary_index = self.cursors.iter().position(|c| c.is_primary).unwrap_or(0);
    }

    fn compute_line_offsets(text: &str) -> Vec<usize> {
        let mut offsets = vec![0];
        for (i, c) in text.char_indices() {
            if c == '\n' {
                offsets.push(i + 1);
            }
        }
        offsets
    }

    fn offset_to_position(offset: usize, line_offsets: &[usize]) -> CursorPosition {
        let line = line_offsets
            .iter()
            .rposition(|&off| off <= offset)
            .unwrap_or(0);
        let col = offset - line_offsets.get(line).unwrap_or(&0);
        CursorPosition::new(line, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_position_new() {
        let pos = CursorPosition::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_selection_point() {
        let pos = CursorPosition::new(0, 0);
        let sel = Selection::point(pos);
        assert!(sel.is_empty());
    }

    #[test]
    fn test_selection_range() {
        let sel = Selection::new(
            CursorPosition::new(1, 5),
            CursorPosition::new(0, 3),
        );
        let (start, end) = sel.range();
        assert_eq!(start, CursorPosition::new(0, 3));
        assert_eq!(end, CursorPosition::new(1, 5));
    }

    #[test]
    fn test_selection_overlaps() {
        let s1 = Selection::new(CursorPosition::new(0, 0), CursorPosition::new(0, 10));
        let s2 = Selection::new(CursorPosition::new(0, 5), CursorPosition::new(0, 15));
        assert!(s1.overlaps(&s2));

        let s3 = Selection::new(CursorPosition::new(0, 20), CursorPosition::new(0, 25));
        assert!(!s1.overlaps(&s3));
    }

    #[test]
    fn test_cursor_new() {
        let c = Cursor::new(CursorPosition::new(1, 2));
        assert!(!c.is_primary);
        assert!(c.selection.is_none());
    }

    #[test]
    fn test_cursor_selection() {
        let mut c = Cursor::new(CursorPosition::new(0, 0));
        c.start_selection();
        c.position = CursorPosition::new(0, 5);
        c.extend_selection();

        let (start, end) = c.selection_range().unwrap();
        assert_eq!(end, CursorPosition::new(0, 5));
        assert_eq!(start, CursorPosition::new(0, 0));
    }

    #[test]
    fn test_multicursor_new() {
        let mc = MultiCursor::new(CursorPosition::new(0, 0));
        assert_eq!(mc.count(), 1);
        assert!(mc.primary().is_primary);
    }

    #[test]
    fn test_multicursor_add() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        mc.add(CursorPosition::new(1, 0));
        mc.add(CursorPosition::new(2, 0));
        assert_eq!(mc.count(), 3);
    }

    #[test]
    fn test_multicursor_add_duplicate() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        mc.add(CursorPosition::new(0, 0));
        assert_eq!(mc.count(), 1);
    }

    #[test]
    fn test_multicursor_remove() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        mc.add(CursorPosition::new(1, 0));
        assert!(mc.remove(CursorPosition::new(1, 0)));
        assert_eq!(mc.count(), 1);
    }

    #[test]
    fn test_multicursor_remove_last() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        assert!(!mc.remove(CursorPosition::new(0, 0)));
        assert_eq!(mc.count(), 1);
    }

    #[test]
    fn test_multicursor_clear_secondary() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        mc.add(CursorPosition::new(1, 0));
        mc.add(CursorPosition::new(2, 0));
        mc.clear_secondary();
        assert_eq!(mc.count(), 1);
    }

    #[test]
    fn test_multicursor_add_next_match() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        let text = "hello world hello";
        assert!(mc.add_next_match(text, "hello"));
        assert_eq!(mc.count(), 2);
    }

    #[test]
    fn test_multicursor_add_all_matches() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        let text = "hello world hello hello";
        mc.add_all_matches(text, "hello");
        assert_eq!(mc.count(), 3);
    }

    #[test]
    fn test_multicursor_move_all() {
        let mut mc = MultiCursor::new(CursorPosition::new(0, 0));
        mc.add(CursorPosition::new(1, 0));
        mc.move_all(|p| CursorPosition::new(p.line, p.col + 1));
        assert_eq!(mc.primary().position.col, 1);
    }

    #[test]
    fn test_multicursor_prepare_insert() {
        let mc = MultiCursor::new(CursorPosition::new(0, 0));
        let result = mc.prepare_edit(&CursorEdit::Insert("x".to_string()), "hello");
        assert_eq!(result.edits.len(), 1);
        assert_eq!(result.edits[0], (0, 0, "x".to_string()));
    }

    #[test]
    fn test_offset_to_position() {
        let text = "hello\nworld\n";
        let offsets = MultiCursor::compute_line_offsets(text);
        let pos = MultiCursor::offset_to_position(7, &offsets);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.col, 1);
    }
}
