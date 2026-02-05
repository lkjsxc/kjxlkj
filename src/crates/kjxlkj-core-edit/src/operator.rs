//! Operator implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{CursorPosition, Operator, RegisterContent, RegisterType};

/// Result of an operator execution.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    /// Text that was affected (for yank/delete).
    pub text: Option<RegisterContent>,
    /// New cursor position.
    pub cursor: CursorPosition,
    /// Whether to enter insert mode after.
    pub enter_insert: bool,
}

/// Execute an operator on a range.
pub fn execute_operator(
    buffer: &mut TextBuffer,
    operator: Operator,
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
) -> OperatorResult {
    let (start, end) = normalize_range(start, end);

    match operator {
        Operator::Delete => delete_range(buffer, start, end, linewise),
        Operator::Yank => yank_range(buffer, start, end, linewise),
        Operator::Change => change_range(buffer, start, end, linewise),
        Operator::Indent => indent_range(buffer, start, end),
        Operator::Outdent => outdent_range(buffer, start, end),
        Operator::ToggleCase => toggle_case_range(buffer, start, end),
        Operator::Uppercase => uppercase_range(buffer, start, end),
        Operator::Lowercase => lowercase_range(buffer, start, end),
        Operator::Format => format_range(buffer, start, end),
    }
}

fn normalize_range(start: CursorPosition, end: CursorPosition) -> (CursorPosition, CursorPosition) {
    if start.line < end.line || (start.line == end.line && start.column <= end.column) {
        (start, end)
    } else {
        (end, start)
    }
}

fn delete_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
) -> OperatorResult {
    let (start, end) = if linewise {
        let start = CursorPosition::new(start.line, 0);
        let end_line = end.line.min(buffer.line_count().saturating_sub(1));
        let end = if end_line + 1 < buffer.line_count() {
            CursorPosition::new(end_line + 1, 0)
        } else {
            CursorPosition::new(end_line, buffer.line_len(end_line))
        };
        (start, end)
    } else {
        (start, CursorPosition::new(end.line, end.column + 1))
    };

    let text = get_range_text(buffer, start, end);
    let reg_type = if linewise {
        RegisterType::Line
    } else {
        RegisterType::Char
    };

    buffer.delete_range(start, end);

    OperatorResult {
        text: Some(RegisterContent::new(text, reg_type)),
        cursor: buffer.clamp_cursor(start, false),
        enter_insert: false,
    }
}

fn yank_range(
    buffer: &TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
) -> OperatorResult {
    let (start, end) = if linewise {
        let start = CursorPosition::new(start.line, 0);
        let end = CursorPosition::new(end.line + 1, 0);
        (start, end)
    } else {
        (start, CursorPosition::new(end.line, end.column + 1))
    };

    let text = get_range_text(buffer, start, end);
    let reg_type = if linewise {
        RegisterType::Line
    } else {
        RegisterType::Char
    };

    OperatorResult {
        text: Some(RegisterContent::new(text, reg_type)),
        cursor: start,
        enter_insert: false,
    }
}

fn change_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
) -> OperatorResult {
    let mut result = delete_range(buffer, start, end, linewise);
    result.enter_insert = true;
    result
}

fn indent_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
) -> OperatorResult {
    for line in start.line..=end.line {
        let pos = CursorPosition::new(line, 0);
        buffer.insert(pos, "    ");
    }
    OperatorResult {
        text: None,
        cursor: CursorPosition::new(start.line, 4),
        enter_insert: false,
    }
}

fn outdent_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
) -> OperatorResult {
    for line in start.line..=end.line {
        if let Some(text) = buffer.line(line) {
            let spaces = text.chars().take_while(|c| *c == ' ').count().min(4);
            if spaces > 0 {
                buffer.delete_range(
                    CursorPosition::new(line, 0),
                    CursorPosition::new(line, spaces),
                );
            }
        }
    }
    OperatorResult {
        text: None,
        cursor: CursorPosition::new(start.line, 0),
        enter_insert: false,
    }
}

fn toggle_case_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
) -> OperatorResult {
    transform_range(buffer, start, end, |c| {
        if c.is_uppercase() {
            c.to_lowercase().collect()
        } else {
            c.to_uppercase().collect()
        }
    })
}

fn uppercase_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
) -> OperatorResult {
    transform_range(buffer, start, end, |c| c.to_uppercase().collect())
}

fn lowercase_range(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
) -> OperatorResult {
    transform_range(buffer, start, end, |c| c.to_lowercase().collect())
}

fn format_range(
    _buffer: &mut TextBuffer,
    start: CursorPosition,
    _end: CursorPosition,
) -> OperatorResult {
    OperatorResult {
        text: None,
        cursor: start,
        enter_insert: false,
    }
}

fn transform_range<F>(
    buffer: &mut TextBuffer,
    start: CursorPosition,
    end: CursorPosition,
    transform: F,
) -> OperatorResult
where
    F: Fn(char) -> String,
{
    let end = CursorPosition::new(end.line, end.column + 1);
    let text = get_range_text(buffer, start, end);
    let transformed: String = text.chars().map(&transform).collect();
    buffer.delete_range(start, end);
    buffer.insert(start, &transformed);
    OperatorResult {
        text: None,
        cursor: start,
        enter_insert: false,
    }
}

fn get_range_text(buffer: &TextBuffer, start: CursorPosition, end: CursorPosition) -> String {
    let start_idx = buffer.pos_to_char(start);
    let end_idx = buffer.pos_to_char(end);
    let text = buffer.text();
    text.chars()
        .skip(start_idx)
        .take(end_idx - start_idx)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, BufferName};

    #[test]
    fn test_delete_range() {
        let mut buf =
            TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello world");
        let result = execute_operator(
            &mut buf,
            Operator::Delete,
            CursorPosition::new(0, 0),
            CursorPosition::new(0, 4),
            false,
        );
        assert_eq!(buf.text(), " world");
        assert!(result.text.is_some());
    }

    #[test]
    fn test_yank_range() {
        let buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello world");
        let result = execute_operator(
            &mut buf.clone(),
            Operator::Yank,
            CursorPosition::new(0, 0),
            CursorPosition::new(0, 4),
            false,
        );
        assert!(result.text.is_some());
        assert_eq!(result.text.unwrap().text, "hello");
    }
}
