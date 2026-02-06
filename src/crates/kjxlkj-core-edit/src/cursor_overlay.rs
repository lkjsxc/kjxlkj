/// Cursor behavior with overlays, highlights, and boundary interactions.

/// Cursor style overlay priority (higher number wins).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverlayPriority { Base = 0, Selection = 1, Search = 2, Diagnostic = 3, Cursor = 4 }

/// A highlight region that may overlap with the cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightRegion {
    pub start_line: usize, pub start_col: usize,
    pub end_line: usize, pub end_col: usize,
    pub priority: OverlayPriority,
    pub kind: HighlightKind,
}

/// Kind of highlight overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighlightKind { Selection, SearchMatch, CurrentSearch, Diagnostic, MatchParen }

impl HighlightRegion {
    pub fn contains(&self, line: usize, col: usize) -> bool {
        if line < self.start_line || line > self.end_line { return false; }
        if line == self.start_line && col < self.start_col { return false; }
        if line == self.end_line && col > self.end_col { return false; }
        true
    }
}

/// Determine the effective overlay at a cursor position.
pub fn effective_overlay(regions: &[HighlightRegion], line: usize, col: usize) -> Option<&HighlightRegion> {
    regions.iter()
        .filter(|r| r.contains(line, col))
        .max_by_key(|r| r.priority)
}

/// Cursor boundary behavior at line edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryAction { Clamp, Wrap, NoOp }

/// Resolve cursor column after a line change.
pub fn resolve_cursor_col(target_col: usize, line_len: usize, allow_past_end: bool) -> usize {
    if line_len == 0 { return 0; }
    let max = if allow_past_end { line_len } else { line_len.saturating_sub(1) };
    target_col.min(max)
}

/// Resolve boundary action when cursor moves past start/end of line.
pub fn at_line_boundary(col: usize, line_len: usize, moving_right: bool) -> BoundaryAction {
    if moving_right && col >= line_len.saturating_sub(1) { BoundaryAction::Clamp }
    else if !moving_right && col == 0 { BoundaryAction::Clamp }
    else { BoundaryAction::NoOp }
}

/// Check if cursor is visible in the viewport.
pub fn cursor_in_viewport(cursor_line: usize, top_line: usize, visible_lines: usize, scrolloff: usize) -> bool {
    if visible_lines == 0 { return false; }
    let effective_top = top_line + scrolloff;
    let effective_bottom = top_line + visible_lines.saturating_sub(1).saturating_sub(scrolloff);
    cursor_line >= effective_top && cursor_line <= effective_bottom
}

/// Compute the viewport top line to keep cursor visible with scrolloff.
pub fn viewport_top_for_cursor(cursor_line: usize, current_top: usize, visible_lines: usize, scrolloff: usize) -> usize {
    if visible_lines == 0 { return current_top; }
    let so = scrolloff.min(visible_lines / 2);
    if cursor_line < current_top + so {
        cursor_line.saturating_sub(so)
    } else if cursor_line >= current_top + visible_lines.saturating_sub(so) {
        cursor_line.saturating_sub(visible_lines.saturating_sub(1).saturating_sub(so))
    } else {
        current_top
    }
}

/// Match paren: find the matching bracket for a given character.
pub fn matching_bracket(ch: char) -> Option<char> {
    match ch {
        '(' => Some(')'), ')' => Some('('),
        '[' => Some(']'), ']' => Some('['),
        '{' => Some('}'), '}' => Some('{'),
        _ => None,
    }
}

/// Determine if a bracket is an opening bracket.
pub fn is_opening(ch: char) -> bool { matches!(ch, '(' | '[' | '{') }

#[cfg(test)]
mod tests {
    use super::*;

    fn region(sl: usize, sc: usize, el: usize, ec: usize, p: OverlayPriority, k: HighlightKind) -> HighlightRegion {
        HighlightRegion { start_line: sl, start_col: sc, end_line: el, end_col: ec, priority: p, kind: k }
    }

    #[test]
    fn region_contains() {
        let r = region(1, 5, 3, 10, OverlayPriority::Selection, HighlightKind::Selection);
        assert!(r.contains(2, 0));
        assert!(!r.contains(0, 0));
        assert!(!r.contains(3, 11));
    }

    #[test]
    fn effective_overlay_priority() {
        let regions = vec![
            region(0, 0, 0, 10, OverlayPriority::Selection, HighlightKind::Selection),
            region(0, 0, 0, 10, OverlayPriority::Search, HighlightKind::SearchMatch),
        ];
        let eff = effective_overlay(&regions, 0, 5).unwrap();
        assert_eq!(eff.priority, OverlayPriority::Search);
    }

    #[test]
    fn resolve_col_normal() {
        assert_eq!(resolve_cursor_col(5, 10, false), 5);
        assert_eq!(resolve_cursor_col(15, 10, false), 9);
        assert_eq!(resolve_cursor_col(15, 10, true), 10);
        assert_eq!(resolve_cursor_col(5, 0, false), 0);
    }

    #[test]
    fn boundary_actions() {
        assert_eq!(at_line_boundary(9, 10, true), BoundaryAction::Clamp);
        assert_eq!(at_line_boundary(0, 10, false), BoundaryAction::Clamp);
        assert_eq!(at_line_boundary(5, 10, true), BoundaryAction::NoOp);
    }

    #[test]
    fn cursor_visible_with_scrolloff() {
        assert!(cursor_in_viewport(10, 5, 20, 3));
        assert!(!cursor_in_viewport(5, 5, 20, 3));
        assert!(!cursor_in_viewport(22, 5, 20, 3));
    }

    #[test]
    fn viewport_top_follows_cursor() {
        assert_eq!(viewport_top_for_cursor(100, 50, 30, 5), 76);
        assert_eq!(viewport_top_for_cursor(5, 50, 30, 5), 0);
        assert_eq!(viewport_top_for_cursor(60, 50, 30, 5), 50);
    }

    #[test]
    fn match_brackets() {
        assert_eq!(matching_bracket('('), Some(')'));
        assert_eq!(matching_bracket('}'), Some('{'));
        assert_eq!(matching_bracket('x'), None);
    }

    #[test]
    fn opening_check() {
        assert!(is_opening('(')); assert!(is_opening('{')); assert!(!is_opening(')'));
    }
}
