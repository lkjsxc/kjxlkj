//! Cursor overlay, bracket matching, and overlay priority resolution.

use kjxlkj_core_types::Position;

/// Priority levels for overlapping highlights (ordered low→high).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OverlayPriority {
    Base,
    Selection,
    Search,
    Diagnostic,
    Cursor,
}

/// A highlighted region with a priority kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightRegion {
    pub start: Position,
    pub end: Position,
    pub kind: OverlayPriority,
}

/// Return the highest-priority overlay that covers `pos`.
pub fn effective_overlay(overlays: &[HighlightRegion], pos: Position) -> Option<OverlayPriority> {
    overlays.iter()
        .filter(|r| pos >= r.start && pos < r.end)
        .map(|r| r.kind)
        .max()
}

/// Boundary action when cursor col exceeds line length.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundaryAction {
    Clamp,
    Wrap,
    NoOp,
}

/// Resolve a cursor column given line length and boundary action.
pub fn resolve_cursor_col(col: usize, line_len: usize, action: BoundaryAction) -> usize {
    if line_len == 0 { return 0; }
    match action {
        BoundaryAction::Clamp => col.min(line_len.saturating_sub(1)),
        BoundaryAction::Wrap => {
            if col >= line_len { 0 } else { col }
        }
        BoundaryAction::NoOp => col,
    }
}

/// Check whether a cursor position is within the visible viewport.
pub fn cursor_in_viewport(
    cursor: Position, top: usize, left: usize, height: usize, width: usize,
) -> bool {
    cursor.line >= top
        && cursor.line < top + height
        && cursor.col >= left
        && cursor.col < left + width
}

/// Find the matching bracket on the same line, if any.
///
/// Supports `()`, `[]`, `{}`.
pub fn matching_bracket(line: &str, col: usize) -> Option<usize> {
    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() { return None; }
    let ch = chars[col];
    let (open, close, forward) = match ch {
        '(' => ('(', ')', true),
        '[' => ('[', ']', true),
        '{' => ('{', '}', true),
        ')' => ('(', ')', false),
        ']' => ('[', ']', false),
        '}' => ('{', '}', false),
        _ => return None,
    };
    let mut depth = 0i32;
    if forward {
        for i in col..chars.len() {
            if chars[i] == open { depth += 1; }
            if chars[i] == close { depth -= 1; }
            if depth == 0 { return Some(i); }
        }
    } else {
        for i in (0..=col).rev() {
            if chars[i] == close { depth += 1; }
            if chars[i] == open { depth -= 1; }
            if depth == 0 { return Some(i); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_highest_priority() {
        let overlays = vec![
            HighlightRegion { start: Position::new(0,0), end: Position::new(0,10), kind: OverlayPriority::Selection },
            HighlightRegion { start: Position::new(0,0), end: Position::new(0,5),  kind: OverlayPriority::Cursor },
        ];
        assert_eq!(effective_overlay(&overlays, Position::new(0,2)), Some(OverlayPriority::Cursor));
        assert_eq!(effective_overlay(&overlays, Position::new(0,7)), Some(OverlayPriority::Selection));
    }

    #[test]
    fn overlay_none() {
        assert_eq!(effective_overlay(&[], Position::new(0,0)), None);
    }

    #[test]
    fn resolve_clamp() {
        assert_eq!(resolve_cursor_col(100, 10, BoundaryAction::Clamp), 9);
        assert_eq!(resolve_cursor_col(5, 10, BoundaryAction::Clamp), 5);
    }

    #[test]
    fn resolve_wrap() {
        assert_eq!(resolve_cursor_col(10, 10, BoundaryAction::Wrap), 0);
        assert_eq!(resolve_cursor_col(3, 10, BoundaryAction::Wrap), 3);
    }

    #[test]
    fn resolve_noop() {
        assert_eq!(resolve_cursor_col(999, 10, BoundaryAction::NoOp), 999);
    }

    #[test]
    fn resolve_empty_line() {
        assert_eq!(resolve_cursor_col(5, 0, BoundaryAction::Clamp), 0);
    }

    #[test]
    fn cursor_viewport() {
        assert!(cursor_in_viewport(Position::new(5,10), 0, 0, 24, 80));
        assert!(!cursor_in_viewport(Position::new(30,0), 0, 0, 24, 80));
    }

    #[test]
    fn matching_bracket_parens() {
        assert_eq!(matching_bracket("(abc)", 0), Some(4));
        assert_eq!(matching_bracket("(abc)", 4), Some(0));
    }

    #[test]
    fn matching_bracket_nested() {
        assert_eq!(matching_bracket("{({})}", 0), Some(5));
    }

    #[test]
    fn matching_bracket_none() {
        assert_eq!(matching_bracket("abc", 0), None);
        assert_eq!(matching_bracket("(abc", 0), None);
    }
}
