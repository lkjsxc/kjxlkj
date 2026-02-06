//! Operator implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    OperatorKind, Position, Range, RegisterContent, RegisterType,
};

/// Result of applying an operator.
pub struct OperatorResult {
    pub deleted_text: Option<RegisterContent>,
    pub new_cursor: Position,
    pub enter_insert: bool,
}

/// Apply an operator over a range in the buffer.
pub fn apply_operator(
    buf: &mut TextBuffer,
    op: OperatorKind,
    range: Range,
    is_linewise: bool,
) -> OperatorResult {
    let ordered = range.ordered();
    match op {
        OperatorKind::Delete => apply_delete(buf, ordered, is_linewise),
        OperatorKind::Yank => apply_yank(buf, ordered, is_linewise),
        OperatorKind::Change => apply_change(buf, ordered, is_linewise),
        OperatorKind::Indent => apply_indent(buf, ordered, true),
        OperatorKind::Outdent => apply_indent(buf, ordered, false),
        OperatorKind::Uppercase => apply_case(buf, ordered, CaseAction::Upper),
        OperatorKind::Lowercase => apply_case(buf, ordered, CaseAction::Lower),
        OperatorKind::ToggleCase => apply_case(buf, ordered, CaseAction::Toggle),
        _ => OperatorResult {
            deleted_text: None,
            new_cursor: ordered.start,
            enter_insert: false,
        },
    }
}

fn apply_delete(buf: &mut TextBuffer, range: Range, linewise: bool) -> OperatorResult {
    let (text, new_cursor) = if linewise {
        delete_lines(buf, range)
    } else {
        let text = buf.delete_range(range);
        let cursor = buf.clamp_position(range.start);
        (text, cursor)
    };
    let reg_type = if linewise {
        RegisterType::Linewise
    } else {
        RegisterType::Charwise
    };
    OperatorResult {
        deleted_text: Some(RegisterContent {
            text,
            reg_type,
        }),
        new_cursor,
        enter_insert: false,
    }
}

fn apply_yank(buf: &TextBuffer, range: Range, linewise: bool) -> OperatorResult {
    let text = if linewise {
        yank_lines(buf, range)
    } else {
        let start = buf.pos_to_char_idx(range.start);
        let end = buf.pos_to_char_idx(range.end).min(buf.rope().len_chars());
        buf.rope().slice(start..end).to_string()
    };
    let reg_type = if linewise {
        RegisterType::Linewise
    } else {
        RegisterType::Charwise
    };
    OperatorResult {
        deleted_text: Some(RegisterContent {
            text,
            reg_type,
        }),
        new_cursor: range.start,
        enter_insert: false,
    }
}

fn apply_change(buf: &mut TextBuffer, range: Range, linewise: bool) -> OperatorResult {
    let mut result = apply_delete(buf, range, linewise);
    result.enter_insert = true;
    result
}

fn delete_lines(buf: &mut TextBuffer, range: Range) -> (String, Position) {
    let start_line = range.start.line;
    let end_line = range.end.line.min(buf.line_count().saturating_sub(1));
    let mut deleted = String::new();
    let count = end_line - start_line + 1;
    for _ in 0..count {
        if start_line < buf.line_count() {
            deleted.push_str(&buf.delete_line(start_line));
        }
    }
    let cursor_line = start_line.min(buf.line_count().saturating_sub(1));
    let cursor = Position::new(cursor_line, 0);
    (deleted, buf.clamp_position(cursor))
}

fn yank_lines(buf: &TextBuffer, range: Range) -> String {
    let start_line = range.start.line;
    let end_line = range.end.line.min(buf.line_count().saturating_sub(1));
    let mut text = String::new();
    for i in start_line..=end_line {
        text.push_str(&buf.line_to_string(i));
        text.push('\n');
    }
    text
}

enum CaseAction {
    Upper,
    Lower,
    Toggle,
}

fn apply_case(buf: &mut TextBuffer, range: Range, action: CaseAction) -> OperatorResult {
    let start = buf.pos_to_char_idx(range.start);
    let end = buf.pos_to_char_idx(range.end).min(buf.rope().len_chars());
    let slice: String = buf.rope().slice(start..end).to_string();
    let transformed: String = match action {
        CaseAction::Upper => slice.to_uppercase(),
        CaseAction::Lower => slice.to_lowercase(),
        CaseAction::Toggle => slice
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            })
            .collect(),
    };
    let r = Range::new(range.start, range.end);
    buf.delete_range(r);
    buf.insert_text(range.start, &transformed);
    OperatorResult {
        deleted_text: None,
        new_cursor: range.start,
        enter_insert: false,
    }
}

fn apply_indent(buf: &mut TextBuffer, range: Range, indent: bool) -> OperatorResult {
    let start_line = range.start.line;
    let end_line = range.end.line.min(buf.line_count().saturating_sub(1));
    for line_idx in start_line..=end_line {
        if indent {
            buf.insert_text(Position::new(line_idx, 0), "    ");
        } else {
            let ls = buf.line_to_string(line_idx);
            let spaces = ls.chars().take(4).take_while(|c| *c == ' ').count();
            if spaces > 0 {
                buf.delete_range(Range::new(
                    Position::new(line_idx, 0),
                    Position::new(line_idx, spaces),
                ));
            }
        }
    }
    OperatorResult {
        deleted_text: None,
        new_cursor: range.start,
        enter_insert: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_charwise() {
        let mut buf = TextBuffer::from_text("hello world");
        let result = apply_operator(
            &mut buf,
            OperatorKind::Delete,
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            false,
        );
        assert_eq!(buf.text(), " world");
        assert!(result.deleted_text.is_some());
        assert_eq!(result.deleted_text.unwrap().text, "hello");
    }

    #[test]
    fn yank_charwise() {
        let buf = TextBuffer::from_text("hello world");
        let result = apply_yank(
            &buf,
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            false,
        );
        assert_eq!(result.deleted_text.unwrap().text, "hello");
    }

    #[test]
    fn change_enters_insert() {
        let mut buf = TextBuffer::from_text("hello world");
        let result = apply_operator(
            &mut buf,
            OperatorKind::Change,
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            false,
        );
        assert!(result.enter_insert);
    }

    #[test]
    fn indent_outdent() {
        let mut buf = TextBuffer::from_text("hello\nworld");
        apply_operator(
            &mut buf,
            OperatorKind::Indent,
            Range::new(Position::new(0, 0), Position::new(1, 0)),
            false,
        );
        assert!(buf.text().starts_with("    hello"));
    }

    #[test]
    fn toggle_case() {
        let mut buf = TextBuffer::from_text("Hello");
        apply_operator(
            &mut buf,
            OperatorKind::ToggleCase,
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            false,
        );
        assert_eq!(buf.text(), "hELLO");
    }
}
