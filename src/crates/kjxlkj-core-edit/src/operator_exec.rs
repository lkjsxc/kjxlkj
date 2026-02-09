//! Operator execution: applies operators over motions
//! and text objects.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::{Operator, RegisterName};
use kjxlkj_core_undo::{UndoEntry, UndoTree};

use crate::cursor::CursorPosition;
use crate::operator_helpers::*;
use crate::registers::RegisterFile;

/// Execution context for an operator.
pub struct OperatorContext<'a> {
    pub content: &'a mut BufferContent,
    pub cursor: &'a mut CursorPosition,
    pub undo: &'a mut UndoTree,
    pub registers: &'a mut RegisterFile,
    pub target_register: RegisterName,
}

/// Execute an operator over the range [start, end].
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
            true
        }
        Operator::Yank => exec_yank(start, end, linewise, ctx),
        Operator::Indent => exec_indent(start, end, ctx, true),
        Operator::Dedent => exec_indent(start, end, ctx, false),
        Operator::ToggleCase => exec_case(start, end, ctx, CaseOp::Toggle),
        Operator::Lowercase => exec_case(start, end, ctx, CaseOp::Lower),
        Operator::Uppercase => exec_case(start, end, ctx, CaseOp::Upper),
        Operator::Reindent | Operator::Format => false,
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
        ctx.content
            .delete(s.line, s.grapheme_offset, e.line, e.grapheme_offset + 1);
        *ctx.cursor = s;
    }

    if ctx.target_register != RegisterName::BlackHole {
        ctx.registers
            .store(ctx.target_register, old_text.clone(), linewise);
    }

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
