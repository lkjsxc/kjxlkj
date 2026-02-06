/// Visual mode selection tracking — character, line, block selection with anchor/cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionKind { Char, Line, Block }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectPos { pub line: usize, pub col: usize }

impl SelectPos { pub fn new(line: usize, col: usize) -> Self { Self { line, col } } }

impl PartialOrd for SelectPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl Ord for SelectPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line).then(self.col.cmp(&other.col))
    }
}

/// A visual selection region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualSelection {
    pub kind: SelectionKind, pub anchor: SelectPos, pub cursor: SelectPos,
}

impl VisualSelection {
    pub fn new(kind: SelectionKind, pos: SelectPos) -> Self { Self { kind, anchor: pos, cursor: pos } }
    pub fn start(&self) -> SelectPos { std::cmp::min(self.anchor, self.cursor) }
    pub fn end(&self) -> SelectPos { std::cmp::max(self.anchor, self.cursor) }
    pub fn swap_ends(&mut self) { std::mem::swap(&mut self.anchor, &mut self.cursor); }
    pub fn move_cursor(&mut self, pos: SelectPos) { self.cursor = pos; }

    /// Check if a given position is within the selection.
    pub fn contains(&self, pos: SelectPos) -> bool {
        match self.kind {
            SelectionKind::Char => {
                let (s, e) = (self.start(), self.end());
                if pos.line < s.line || pos.line > e.line { return false; }
                if pos.line == s.line && pos.col < s.col { return false; }
                if pos.line == e.line && pos.col > e.col { return false; }
                true
            }
            SelectionKind::Line => {
                let (s, e) = (self.start(), self.end());
                pos.line >= s.line && pos.line <= e.line
            }
            SelectionKind::Block => {
                let min_line = self.anchor.line.min(self.cursor.line);
                let max_line = self.anchor.line.max(self.cursor.line);
                let min_col = self.anchor.col.min(self.cursor.col);
                let max_col = self.anchor.col.max(self.cursor.col);
                pos.line >= min_line && pos.line <= max_line
                    && pos.col >= min_col && pos.col <= max_col
            }
        }
    }

    pub fn line_range(&self) -> (usize, usize) { (self.start().line, self.end().line) }
    pub fn block_cols(&self) -> Option<(usize, usize)> {
        if self.kind != SelectionKind::Block { return None; }
        let min_col = self.anchor.col.min(self.cursor.col);
        let max_col = self.anchor.col.max(self.cursor.col);
        Some((min_col, max_col))
    }

    pub fn line_count(&self) -> usize { let (s, e) = self.line_range(); e - s + 1 }
    pub fn switch_kind(&mut self, new_kind: SelectionKind) { self.kind = new_kind; }
}

/// Extract the selected text from buffer lines.
pub fn extract_selection<'a>(sel: &VisualSelection, lines: &'a [&str]) -> Vec<String> {
    let (start, end) = (sel.start(), sel.end());
    match sel.kind {
        SelectionKind::Line => {
            (start.line..=end.line).filter_map(|l| lines.get(l).map(|s| s.to_string())).collect()
        }
        SelectionKind::Char => {
            let mut result = Vec::new();
            for line_idx in start.line..=end.line {
                if let Some(line) = lines.get(line_idx) {
                    let from = if line_idx == start.line { start.col } else { 0 };
                    let to = if line_idx == end.line { (end.col + 1).min(line.len()) } else { line.len() };
                    result.push(line.get(from..to).unwrap_or("").to_string());
                }
            }
            result
        }
        SelectionKind::Block => {
            let (min_col, max_col) = sel.block_cols().unwrap_or((0, 0));
            (start.line..=end.line).filter_map(|l| {
                lines.get(l).map(|s| {
                    let from = min_col.min(s.len());
                    let to = (max_col + 1).min(s.len());
                    s.get(from..to).unwrap_or("").to_string()
                })
            }).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_selection_contains() {
        let sel = VisualSelection { kind: SelectionKind::Char,
            anchor: SelectPos::new(1, 5), cursor: SelectPos::new(3, 2) };
        assert!(sel.contains(SelectPos::new(2, 0)));
        assert!(!sel.contains(SelectPos::new(0, 0)));
        assert!(!sel.contains(SelectPos::new(3, 3)));
    }

    #[test]
    fn line_selection_contains() {
        let sel = VisualSelection { kind: SelectionKind::Line,
            anchor: SelectPos::new(2, 0), cursor: SelectPos::new(4, 0) };
        assert!(sel.contains(SelectPos::new(3, 99)));
        assert!(!sel.contains(SelectPos::new(5, 0)));
    }

    #[test]
    fn block_selection_contains() {
        let sel = VisualSelection { kind: SelectionKind::Block,
            anchor: SelectPos::new(1, 3), cursor: SelectPos::new(3, 7) };
        assert!(sel.contains(SelectPos::new(2, 5)));
        assert!(!sel.contains(SelectPos::new(2, 2)));
    }

    #[test]
    fn swap_ends() {
        let mut sel = VisualSelection::new(SelectionKind::Char, SelectPos::new(0, 0));
        sel.move_cursor(SelectPos::new(5, 3));
        sel.swap_ends();
        assert_eq!(sel.anchor, SelectPos::new(5, 3));
        assert_eq!(sel.cursor, SelectPos::new(0, 0));
    }

    #[test]
    fn extract_char() {
        let lines = vec!["hello world", "foo bar", "baz qux"];
        let sel = VisualSelection { kind: SelectionKind::Char,
            anchor: SelectPos::new(0, 6), cursor: SelectPos::new(1, 2) };
        let text = extract_selection(&sel, &lines);
        assert_eq!(text, vec!["world", "foo"]);
    }

    #[test]
    fn extract_line() {
        let lines = vec!["aaa", "bbb", "ccc"];
        let sel = VisualSelection { kind: SelectionKind::Line,
            anchor: SelectPos::new(0, 0), cursor: SelectPos::new(1, 0) };
        let text = extract_selection(&sel, &lines);
        assert_eq!(text, vec!["aaa", "bbb"]);
    }

    #[test]
    fn extract_block() {
        let lines = vec!["abcdef", "ghijkl", "mnopqr"];
        let sel = VisualSelection { kind: SelectionKind::Block,
            anchor: SelectPos::new(0, 1), cursor: SelectPos::new(2, 3) };
        let text = extract_selection(&sel, &lines);
        assert_eq!(text, vec!["bcd", "hij", "nop"]);
    }

    #[test]
    fn line_count_and_block_cols() {
        let sel = VisualSelection { kind: SelectionKind::Block,
            anchor: SelectPos::new(2, 5), cursor: SelectPos::new(4, 10) };
        assert_eq!(sel.line_count(), 3);
        assert_eq!(sel.block_cols(), Some((5, 10)));
    }

    #[test]
    fn switch_kind() {
        let mut sel = VisualSelection::new(SelectionKind::Char, SelectPos::new(0, 0));
        sel.switch_kind(SelectionKind::Line);
        assert_eq!(sel.kind, SelectionKind::Line);
    }
}
