//! Edit operators (delete, yank, change, etc.).

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// An editing operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Delete text.
    Delete,
    /// Yank (copy) text.
    Yank,
    /// Change text (delete and enter insert mode).
    Change,
    /// Uppercase.
    Uppercase,
    /// Lowercase.
    Lowercase,
    /// Toggle case.
    ToggleCase,
    /// Indent.
    Indent,
    /// Outdent.
    Outdent,
}

/// Result of applying an operator.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    /// Text that was affected (for register storage).
    pub text: String,
    /// Whether the operation was linewise.
    pub linewise: bool,
    /// New cursor position.
    pub cursor: Position,
}

/// Apply an operator over a range.
pub fn apply_operator(
    buffer: &mut TextBuffer,
    operator: Operator,
    start: Position,
    end: Position,
    linewise: bool,
) -> OperatorResult {
    match operator {
        Operator::Delete => apply_delete(buffer, start, end, linewise),
        Operator::Yank => apply_yank(buffer, start, end, linewise),
        Operator::Change => apply_delete(buffer, start, end, linewise),
        Operator::Uppercase => apply_case(buffer, start, end, linewise, CaseOp::Upper),
        Operator::Lowercase => apply_case(buffer, start, end, linewise, CaseOp::Lower),
        Operator::ToggleCase => apply_case(buffer, start, end, linewise, CaseOp::Toggle),
        Operator::Indent => apply_indent(buffer, start, end, true),
        Operator::Outdent => apply_indent(buffer, start, end, false),
    }
}

fn apply_delete(
    buffer: &mut TextBuffer,
    start: Position,
    end: Position,
    linewise: bool,
) -> OperatorResult {
    let (start, end) = normalize_range(start, end);

    if linewise {
        // Delete entire lines
        let mut deleted = String::new();
        for line in start.line..=end.line {
            if let Some(slice) = buffer.line(start.line) {
                deleted.push_str(slice.as_str().unwrap_or(""));
            }
            buffer.remove_line(start.line);
        }
        OperatorResult {
            text: deleted,
            linewise: true,
            cursor: Position::new(start.line, 0),
        }
    } else {
        // Delete character range
        let start_idx = buffer.line_to_char(start.line) + start.col;
        let end_idx = buffer.line_to_char(end.line) + end.col + 1;
        let text = buffer.rope().slice(start_idx..end_idx).to_string();
        buffer.remove(start_idx, end_idx);
        OperatorResult {
            text,
            linewise: false,
            cursor: start,
        }
    }
}

fn apply_yank(
    buffer: &TextBuffer,
    start: Position,
    end: Position,
    linewise: bool,
) -> OperatorResult {
    let (start, end) = normalize_range(start, end);

    if linewise {
        let mut yanked = String::new();
        for line in start.line..=end.line {
            if let Some(slice) = buffer.line(line) {
                yanked.push_str(slice.as_str().unwrap_or(""));
            }
        }
        OperatorResult {
            text: yanked,
            linewise: true,
            cursor: start,
        }
    } else {
        let start_idx = buffer.line_to_char(start.line) + start.col;
        let end_idx = buffer.line_to_char(end.line) + end.col + 1;
        let end_idx = end_idx.min(buffer.char_count());
        let text = buffer.rope().slice(start_idx..end_idx).to_string();
        OperatorResult {
            text,
            linewise: false,
            cursor: start,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CaseOp {
    Upper,
    Lower,
    Toggle,
}

fn apply_case(
    buffer: &mut TextBuffer,
    start: Position,
    end: Position,
    linewise: bool,
    op: CaseOp,
) -> OperatorResult {
    let (start, end) = normalize_range(start, end);

    let (start_idx, end_idx) = if linewise {
        let start_idx = buffer.line_to_char(start.line);
        let end_idx = if end.line + 1 < buffer.line_count() {
            buffer.line_to_char(end.line + 1)
        } else {
            buffer.char_count()
        };
        (start_idx, end_idx)
    } else {
        let start_idx = buffer.line_to_char(start.line) + start.col;
        let end_idx = buffer.line_to_char(end.line) + end.col + 1;
        (start_idx, end_idx.min(buffer.char_count()))
    };

    let text = buffer.rope().slice(start_idx..end_idx).to_string();
    let transformed: String = text
        .chars()
        .map(|c| match op {
            CaseOp::Upper => c.to_uppercase().next().unwrap_or(c),
            CaseOp::Lower => c.to_lowercase().next().unwrap_or(c),
            CaseOp::Toggle => {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            }
        })
        .collect();

    buffer.remove(start_idx, end_idx);
    buffer.insert(start_idx, &transformed);

    OperatorResult {
        text: transformed,
        linewise,
        cursor: start,
    }
}

fn apply_indent(
    buffer: &mut TextBuffer,
    start: Position,
    end: Position,
    indent: bool,
) -> OperatorResult {
    let (start, end) = normalize_range(start, end);
    let indent_str = "    "; // 4 spaces

    for line in start.line..=end.line {
        let line_start = buffer.line_to_char(line);

        if indent {
            buffer.insert(line_start, indent_str);
        } else {
            // Remove up to 4 spaces
            if let Some(slice) = buffer.line(line) {
                let s = slice.as_str().unwrap_or("");
                let spaces: usize = s.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    buffer.remove(line_start, line_start + spaces);
                }
            }
        }
    }

    OperatorResult {
        text: String::new(),
        linewise: true,
        cursor: start,
    }
}

fn normalize_range(start: Position, end: Position) -> (Position, Position) {
    if start.line < end.line || (start.line == end.line && start.col <= end.col) {
        (start, end)
    } else {
        (end, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_delete_chars() {
        let mut buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let result = apply_operator(
            &mut buffer,
            Operator::Delete,
            Position::new(0, 0),
            Position::new(0, 4),
            false,
        );
        assert_eq!(result.text, "hello");
        assert_eq!(buffer.to_string(), " world");
    }

    #[test]
    fn test_yank() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let result = apply_yank(&buffer, Position::new(0, 0), Position::new(0, 4), false);
        assert_eq!(result.text, "hello");
    }

    #[test]
    fn test_indent() {
        let mut buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2");
        apply_indent(&mut buffer, Position::new(0, 0), Position::new(1, 0), true);
        assert!(buffer.to_string().starts_with("    line1"));
    }
}
