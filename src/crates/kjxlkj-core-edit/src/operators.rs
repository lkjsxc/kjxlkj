//! Editing operators that act on ranges within a buffer.

use kjxlkj_core_text::manipulation::{convert_case, reindent, indent_level, CaseKind};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Operator, Position, Range};

/// Result of applying an operator to a range.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    pub deleted_text: Option<String>,
    pub new_cursor: Position,
    pub entered_insert: bool,
}

/// Apply an operator to a range in the buffer.
pub fn apply_operator(buffer: &mut TextBuffer, op: Operator, range: Range) -> OperatorResult {
    let r = range.normalized();
    match op {
        Operator::Delete => {
            let text = delete_range(buffer, r);
            OperatorResult {
                deleted_text: Some(text),
                new_cursor: buffer.clamp_position(r.start),
                entered_insert: false,
            }
        }
        Operator::Yank => {
            let text = yank_range(buffer, r);
            OperatorResult {
                deleted_text: Some(text),
                new_cursor: r.start,
                entered_insert: false,
            }
        }
        Operator::Change => {
            let text = change_range(buffer, r);
            OperatorResult {
                deleted_text: Some(text),
                new_cursor: r.start,
                entered_insert: true,
            }
        }
        Operator::Indent => {
            indent_range(buffer, r, 1);
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
        Operator::Outdent => {
            outdent_range(buffer, r, 1);
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
        Operator::ToggleCase => {
            toggle_case_range(buffer, r);
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
        Operator::UpperCase => {
            upper_case_range(buffer, r);
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
        Operator::LowerCase => {
            lower_case_range(buffer, r);
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
        Operator::Format => {
            OperatorResult { deleted_text: None, new_cursor: r.start, entered_insert: false }
        }
    }
}

/// Delete the given range from the buffer, returning the deleted text.
pub fn delete_range(buffer: &mut TextBuffer, range: Range) -> String {
    let r = range.normalized();
    buffer.delete_range(r.start, r.end)
}

/// Yank (copy) text in the range without modifying the buffer.
pub fn yank_range(buffer: &TextBuffer, range: Range) -> String {
    let r = range.normalized();
    extract_text(buffer, r)
}

/// Delete the range and return the deleted text; caller enters insert mode.
pub fn change_range(buffer: &mut TextBuffer, range: Range) -> String {
    delete_range(buffer, range)
}

/// Indent every line in the range by `amount` levels.
pub fn indent_range(buffer: &mut TextBuffer, range: Range, amount: usize) {
    let r = range.normalized();
    for line_idx in r.start.line..=r.end.line.min(buffer.line_count().saturating_sub(1)) {
        if let Some(content) = buffer.line(line_idx) {
            let level = indent_level(&content, 4) + amount;
            let new_line = reindent(&content, level, false, 4);
            replace_line(buffer, line_idx, &new_line);
        }
    }
}

/// Outdent every line in the range by `amount` levels.
pub fn outdent_range(buffer: &mut TextBuffer, range: Range, amount: usize) {
    let r = range.normalized();
    for line_idx in r.start.line..=r.end.line.min(buffer.line_count().saturating_sub(1)) {
        if let Some(content) = buffer.line(line_idx) {
            let level = indent_level(&content, 4).saturating_sub(amount);
            let new_line = reindent(&content, level, false, 4);
            replace_line(buffer, line_idx, &new_line);
        }
    }
}

/// Toggle case of all characters in the range.
pub fn toggle_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Toggle);
}

/// Convert all characters in the range to upper case.
pub fn upper_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Upper);
}

/// Convert all characters in the range to lower case.
pub fn lower_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Lower);
}

fn apply_case(buffer: &mut TextBuffer, range: Range, kind: CaseKind) {
    let r = range.normalized();
    let text = extract_text(buffer, r);
    let converted = convert_case(&text, kind);
    buffer.delete_range(r.start, r.end);
    buffer.insert_text(r.start, &converted);
}

fn extract_text(buffer: &TextBuffer, r: Range) -> String {
    let mut result = String::new();
    for line_idx in r.start.line..=r.end.line.min(buffer.line_count().saturating_sub(1)) {
        let line = buffer.line(line_idx).unwrap_or_default();
        let start_col = if line_idx == r.start.line { r.start.col } else { 0 };
        let end_col = if line_idx == r.end.line { r.end.col } else { line.len() };
        let end_col = end_col.min(line.len());
        if start_col < end_col {
            result.push_str(&line[start_col..end_col]);
        }
        if line_idx < r.end.line {
            result.push('\n');
        }
    }
    result
}

fn replace_line(buffer: &mut TextBuffer, line_idx: usize, new_content: &str) {
    let old = buffer.line(line_idx).unwrap_or_default();
    let start = Position::new(line_idx, 0);
    let end = Position::new(line_idx, old.len());
    buffer.delete_range(start, end);
    buffer.insert_text(start, new_content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    fn buf(text: &str) -> TextBuffer {
        TextBuffer::from_text(BufferId(1), "test".into(), text)
    }

    #[test]
    fn test_delete_range() {
        let mut b = buf("hello world");
        let deleted = delete_range(&mut b, Range::new(Position::new(0, 5), Position::new(0, 11)));
        assert_eq!(deleted, " world");
        assert_eq!(b.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_yank_range() {
        let b = buf("hello world");
        let text = yank_range(&b, Range::new(Position::new(0, 0), Position::new(0, 5)));
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_toggle_case() {
        let mut b = buf("Hello");
        toggle_case_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
        assert_eq!(b.line(0).unwrap(), "hELLO");
    }
}
