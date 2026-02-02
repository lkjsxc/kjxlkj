//! Motion execution for cursor movement.

use kjxlkj_core_types::{
    motion::{Motion, MotionResult},
    Position,
};

use super::motion_misc;
use super::motion_word;

/// Trait for text access during motion execution.
pub trait MotionContext {
    /// Returns the total number of lines.
    fn line_count(&self) -> usize;
    /// Returns the length of a line (excluding newline).
    fn line_len(&self, line: usize) -> usize;
    /// Returns a line's content.
    fn line_content(&self, line: usize) -> &str;
    /// Returns the character at a position.
    fn char_at(&self, line: usize, col: usize) -> Option<char>;
}

/// Executor for cursor motions.
pub struct MotionExecutor;

impl MotionExecutor {
    /// Executes a motion from a position.
    pub fn execute<C: MotionContext>(
        ctx: &C,
        pos: Position,
        motion: Motion,
        count: usize,
    ) -> MotionResult {
        let count = count.max(1);
        let line = pos.line.as_usize();
        let col = pos.col.as_usize();
        let mut result = Self::execute_once(ctx, line, col, &motion);

        for _ in 1..count {
            result = Self::execute_once(ctx, result.line, result.column, &motion);
            if result.hit_boundary {
                break;
            }
        }

        result
    }

    fn execute_once<C: MotionContext>(
        ctx: &C,
        line: usize,
        col: usize,
        motion: &Motion,
    ) -> MotionResult {
        match motion {
            Motion::Left => Self::move_left(col, line),
            Motion::Right => Self::move_right(ctx, line, col),
            Motion::Up => Self::move_up(ctx, line, col),
            Motion::Down => Self::move_down(ctx, line, col),
            Motion::FirstColumn => Self::first_column(line),
            Motion::FirstNonBlank => Self::first_non_blank(ctx, line),
            Motion::LineEnd => Self::line_end(ctx, line),
            Motion::WordForward => motion_word::word_forward(ctx, line, col, false),
            Motion::BigWordForward => motion_word::word_forward(ctx, line, col, true),
            Motion::WordBackward => motion_word::word_backward(ctx, line, col, false),
            Motion::BigWordBackward => motion_word::word_backward(ctx, line, col, true),
            Motion::WordEnd => motion_word::word_end(ctx, line, col, false),
            Motion::BigWordEnd => motion_word::word_end(ctx, line, col, true),
            Motion::DocumentStart => motion_misc::document_start(),
            Motion::DocumentEnd => motion_misc::document_end(ctx),
            Motion::GoToLine(target) => motion_misc::go_to_line(ctx, *target),
            Motion::GoToColumn(target) => motion_misc::go_to_column(ctx, line, *target),
            Motion::ParagraphForward => motion_misc::paragraph_forward(ctx, line),
            Motion::ParagraphBackward => motion_misc::paragraph_backward(ctx, line),
            Motion::MatchingBracket => motion_misc::matching_bracket(ctx, line, col),
            _ => MotionResult {
                line,
                column: col,
                wrapped: false,
                hit_boundary: true,
            },
        }
    }

    fn move_left(col: usize, line: usize) -> MotionResult {
        if col > 0 {
            MotionResult {
                line,
                column: col - 1,
                wrapped: false,
                hit_boundary: false,
            }
        } else {
            MotionResult {
                line,
                column: 0,
                wrapped: false,
                hit_boundary: true,
            }
        }
    }

    fn move_right<C: MotionContext>(ctx: &C, line: usize, col: usize) -> MotionResult {
        let line_len = ctx.line_len(line);
        let max_col = line_len.saturating_sub(1);
        if col < max_col {
            MotionResult {
                line,
                column: col + 1,
                wrapped: false,
                hit_boundary: false,
            }
        } else {
            MotionResult {
                line,
                column: max_col,
                wrapped: false,
                hit_boundary: true,
            }
        }
    }

    fn move_up<C: MotionContext>(ctx: &C, line: usize, col: usize) -> MotionResult {
        if line > 0 {
            let new_line = line - 1;
            let line_len = ctx.line_len(new_line);
            let new_col = col.min(line_len.saturating_sub(1));
            MotionResult {
                line: new_line,
                column: new_col,
                wrapped: false,
                hit_boundary: false,
            }
        } else {
            MotionResult {
                line: 0,
                column: col,
                wrapped: false,
                hit_boundary: true,
            }
        }
    }

    fn move_down<C: MotionContext>(ctx: &C, line: usize, col: usize) -> MotionResult {
        let max_line = ctx.line_count().saturating_sub(1);
        if line < max_line {
            let new_line = line + 1;
            let line_len = ctx.line_len(new_line);
            let new_col = col.min(line_len.saturating_sub(1));
            MotionResult {
                line: new_line,
                column: new_col,
                wrapped: false,
                hit_boundary: false,
            }
        } else {
            MotionResult {
                line: max_line,
                column: col,
                wrapped: false,
                hit_boundary: true,
            }
        }
    }

    fn first_column(line: usize) -> MotionResult {
        MotionResult {
            line,
            column: 0,
            wrapped: false,
            hit_boundary: false,
        }
    }

    fn first_non_blank<C: MotionContext>(ctx: &C, line: usize) -> MotionResult {
        let content = ctx.line_content(line);
        let col = content
            .chars()
            .position(|c| !c.is_whitespace())
            .unwrap_or(0);
        MotionResult {
            line,
            column: col,
            wrapped: false,
            hit_boundary: false,
        }
    }

    fn line_end<C: MotionContext>(ctx: &C, line: usize) -> MotionResult {
        let line_len = ctx.line_len(line);
        let col = line_len.saturating_sub(1);
        MotionResult {
            line,
            column: col,
            wrapped: false,
            hit_boundary: false,
        }
    }
}
