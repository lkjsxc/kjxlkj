//! Operator execution: applies operators over motions and text objects.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::{Operator, RegisterName};
use kjxlkj_core_undo::{UndoEntry, UndoTree};

use crate::cursor::CursorPosition;
use crate::registers::RegisterFile;

/// Execution context for an operator.
pub struct OperatorContext<'a> {
    pub content: &'a mut BufferContent,
    pub cursor: &'a mut CursorPosition,
    pub undo: &'a mut UndoTree,
    pub registers: &'a mut RegisterFile,
    pub target_register: RegisterName,
}

/// Execute an operator over the range from `start` to `end` (line, grapheme).
pub fn execute_operator(
    op: Operator,
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
    ctx: &mut OperatorContext<'_>,
) -> bool {
    match op {
        Operator::Delete => exec_delete(start, end, linewise, ctx),
        Operator::Change => {
            exec_delete(start, end, linewise, ctx);
            true // Caller enters insert mode
        }
        Operator::Yank => exec_yank(start, end, linewise, ctx),
        Operator::Indent => exec_indent(start, end, ctx, true),
        Operator::Dedent => exec_indent(start, end, ctx, false),
        Operator::ToggleCase => exec_case(start, end, ctx, CaseOp::Toggle),
        Operator::Lowercase => exec_case(start, end, ctx, CaseOp::Lower),
        Operator::Uppercase => exec_case(start, end, ctx, CaseOp::Upper),
        Operator::Reindent | Operator::Format => {
            // Placeholder: reindent/format not yet fully implemented
            false
        }
    }
}

fn exec_delete(
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
    ctx: &mut OperatorContext<'_>,
) -> bool {
    let (s, e) = normalize_range(start, end);
    let old_text = extract_text(ctx.content, &s, &e, linewise);

    if linewise {
        ctx.content.delete_lines(s.line, e.line + 1);
        ctx.cursor.line = s.line;
        ctx.cursor.grapheme_offset = 0;
    } else {
        ctx.content.delete(
            s.line,
            s.grapheme_offset,
            e.line,
            e.grapheme_offset + 1,
        );
        *ctx.cursor = s;
    }

    // Store in register
    if ctx.target_register != RegisterName::BlackHole {
        ctx.registers.store(
            ctx.target_register,
            old_text.clone(),
            linewise,
        );
    }

    // Record undo
    ctx.undo.record(UndoEntry {
        start: 0,
        old_text,
        new_text: String::new(),
        cursor_before: (start.line, start.grapheme_offset),
        cursor_after: (ctx.cursor.line, ctx.cursor.grapheme_offset),
    });

    false
}

fn exec_yank(
    start: CursorPosition,
    end: CursorPosition,
    linewise: bool,
    ctx: &mut OperatorContext<'_>,
) -> bool {
    let (s, e) = normalize_range(start, end);
    let text = extract_text(ctx.content, &s, &e, linewise);

    ctx.registers.store(ctx.target_register, text, linewise);
    // Yank doesn't move cursor
    false
}

fn exec_indent(
    start: CursorPosition,
    end: CursorPosition,
    ctx: &mut OperatorContext<'_>,
    indent: bool,
) -> bool {
    let (s, e) = normalize_range(start, end);
    for line in s.line..=e.line.min(ctx.content.line_count() - 1) {
        let content = ctx.content.line_content(line);
        let new_content = if indent {
            format!("    {content}")
        } else {
            strip_indent(&content)
        };
        replace_line_content(ctx.content, line, &new_content);
    }
    false
}

enum CaseOp {
    Toggle,
    Lower,
    Upper,
}

fn exec_case(
    start: CursorPosition,
    end: CursorPosition,
    ctx: &mut OperatorContext<'_>,
    op: CaseOp,
) -> bool {
    let (s, e) = normalize_range(start, end);
    let text = extract_text(ctx.content, &s, &e, false);
    let new_text = match op {
        CaseOp::Toggle => text
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            })
            .collect(),
        CaseOp::Lower => text.to_lowercase(),
        CaseOp::Upper => text.to_uppercase(),
    };
    ctx.content.replace(
        s.line,
        s.grapheme_offset,
        e.line,
        e.grapheme_offset + 1,
        &new_text,
    );
    false
}

fn normalize_range(
    a: CursorPosition,
    b: CursorPosition,
) -> (CursorPosition, CursorPosition) {
    if a.line < b.line
        || (a.line == b.line && a.grapheme_offset <= b.grapheme_offset)
    {
        (a, b)
    } else {
        (b, a)
    }
}

fn extract_text(
    content: &BufferContent,
    start: &CursorPosition,
    end: &CursorPosition,
    linewise: bool,
) -> String {
    if linewise {
        let mut result = String::new();
        for line in start.line..=end.line.min(content.line_count() - 1) {
            result.push_str(&content.line_str(line));
        }
        result
    } else {
        // Character-wise extraction
        if start.line == end.line {
            let line = content.line_content(start.line);
            let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
            let mut result = String::new();
            for i in start.grapheme_offset..=end.grapheme_offset {
                if let Some(g) = lg.get(i) {
                    result.push_str(g);
                }
            }
            result
        } else {
            let mut result = String::new();
            // First line from start to end
            let first = content.line_str(start.line);
            let lg = kjxlkj_core_text::LineGraphemes::from_str(&first);
            for i in start.grapheme_offset..lg.count() {
                if let Some(g) = lg.get(i) {
                    result.push_str(g);
                }
            }
            result.push('\n');
            // Middle lines
            for line in (start.line + 1)..end.line {
                result.push_str(&content.line_str(line));
            }
            // Last line
            if end.line > start.line {
                let last = content.line_content(end.line);
                let lg = kjxlkj_core_text::LineGraphemes::from_str(&last);
                for i in 0..=end.grapheme_offset {
                    if let Some(g) = lg.get(i) {
                        result.push_str(g);
                    }
                }
            }
            result
        }
    }
}

fn strip_indent(s: &str) -> String {
    if s.starts_with("    ") {
        s[4..].to_string()
    } else if s.starts_with('\t') {
        s[1..].to_string()
    } else {
        let trimmed = s.trim_start();
        trimmed.to_string()
    }
}

fn replace_line_content(
    content: &mut BufferContent,
    line: usize,
    new_content: &str,
) {
    let old_lg = content.line_graphemes(line);
    let gc = old_lg.count();
    if gc > 0 {
        content.delete(line, 0, line, gc);
    }
    content.insert(line, 0, new_content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_range_order() {
        let a = CursorPosition::new(0, 5);
        let b = CursorPosition::new(0, 2);
        let (s, e) = normalize_range(a, b);
        assert_eq!(s.grapheme_offset, 2);
        assert_eq!(e.grapheme_offset, 5);
    }
}
