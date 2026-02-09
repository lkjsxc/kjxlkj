//! Motion execution: resolves a Motion to a new cursor
//! position.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::Motion;

use crate::cursor::CursorPosition;
use crate::motion_extended::*;
use crate::motion_helpers::*;
use crate::motion_line::*;
use crate::motion_search::*;

/// Execute a motion and return the new cursor position.
pub fn execute_motion(
    cursor: &mut CursorPosition,
    motion: &Motion,
    count: u32,
    content: &BufferContent,
) -> CursorPosition {
    for _ in 0..count {
        execute_single(cursor, motion, content);
    }
    *cursor
}

fn execute_single(cursor: &mut CursorPosition, motion: &Motion, content: &BufferContent) {
    let line_count = content.line_count();
    match motion {
        Motion::Left => {
            if cursor.grapheme_offset > 0 {
                cursor.grapheme_offset -= 1;
            }
            cursor.clear_desired_col();
        }
        Motion::Right => {
            let gc = content.line_graphemes(cursor.line).count();
            let max = if gc > 0 { gc - 1 } else { 0 };
            if cursor.grapheme_offset < max {
                cursor.grapheme_offset += 1;
            }
            cursor.clear_desired_col();
        }
        Motion::Down => {
            if cursor.line + 1 < line_count {
                cursor.line += 1;
                apply_desired_col(cursor, content);
            }
        }
        Motion::Up => {
            if cursor.line > 0 {
                cursor.line -= 1;
                apply_desired_col(cursor, content);
            }
        }
        Motion::LineStart => exec_line_start(cursor),
        Motion::FirstNonBlank => exec_first_non_blank(cursor, content),
        Motion::LineEnd => exec_line_end(cursor, content),
        Motion::LastNonBlank => exec_last_non_blank(cursor, content),
        Motion::GotoFirstLine => exec_goto_first_line(cursor),
        Motion::GotoLastLine => exec_goto_last_line(cursor, line_count),
        Motion::GotoLine(n) => exec_goto_line(cursor, *n, line_count),
        Motion::GotoColumn(col) => exec_goto_column(cursor, *col, content),
        Motion::WordForward => {
            move_word_forward(cursor, content, false);
        }
        Motion::WordForwardBig => {
            move_word_forward(cursor, content, true);
        }
        Motion::WordBackward => {
            move_word_backward(cursor, content, false);
        }
        Motion::WordBackwardBig => {
            move_word_backward(cursor, content, true);
        }
        Motion::WordEndForward => {
            move_word_end(cursor, content, false);
        }
        Motion::WordEndForwardBig => {
            move_word_end(cursor, content, true);
        }
        Motion::ParagraphForward => {
            move_paragraph_forward(cursor, content);
        }
        Motion::ParagraphBackward => {
            move_paragraph_backward(cursor, content);
        }
        Motion::NextLineFirstNonBlank => {
            if cursor.line + 1 < line_count {
                cursor.line += 1;
            }
            move_to_first_non_blank(cursor, content);
        }
        Motion::PrevLineFirstNonBlank => {
            if cursor.line > 0 {
                cursor.line -= 1;
            }
            move_to_first_non_blank(cursor, content);
        }
        Motion::FindCharForward(ch) => {
            exec_find_char(cursor, content, *ch, true, false);
        }
        Motion::FindCharBackward(ch) => {
            exec_find_char(cursor, content, *ch, false, false);
        }
        Motion::TillCharForward(ch) => {
            exec_find_char(cursor, content, *ch, true, true);
        }
        Motion::TillCharBackward(ch) => {
            exec_find_char(cursor, content, *ch, false, true);
        }
        Motion::MatchingBracket => {
            exec_matching_bracket(cursor, content);
        }
        Motion::ScreenTop => {
            cursor.line = 0;
            move_to_first_non_blank(cursor, content);
        }
        Motion::ScreenMiddle => {
            let mid = line_count / 2;
            cursor.line = mid.min(line_count.saturating_sub(1));
            move_to_first_non_blank(cursor, content);
        }
        Motion::ScreenBottom => {
            cursor.line = line_count.saturating_sub(1);
            move_to_first_non_blank(cursor, content);
        }
        Motion::StarForward => {
            exec_star_search(cursor, content, true);
        }
        Motion::StarBackward => {
            exec_star_search(cursor, content, false);
        }
        Motion::SentenceForward => {
            move_sentence_forward(cursor, content);
        }
        Motion::SentenceBackward => {
            move_sentence_backward(cursor, content);
        }
        _ => {}
    }
}
