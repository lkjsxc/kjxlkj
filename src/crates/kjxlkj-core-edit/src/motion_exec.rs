//! Motion execution: resolves a Motion to a new cursor
//! position.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::Motion;

use crate::cursor::CursorPosition;
use crate::motion_helpers::*;
use crate::motion_line::*;

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

fn execute_single(
    cursor: &mut CursorPosition,
    motion: &Motion,
    content: &BufferContent,
) {
    let line_count = content.line_count();
    match motion {
        Motion::Left => {
            if cursor.grapheme_offset > 0 {
                cursor.grapheme_offset -= 1;
            }
            cursor.clear_desired_col();
        }
        Motion::Right => {
            let gc = content
                .line_graphemes(cursor.line)
                .count();
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
        Motion::FirstNonBlank => {
            exec_first_non_blank(cursor, content)
        }
        Motion::LineEnd => {
            exec_line_end(cursor, content)
        }
        Motion::LastNonBlank => {
            exec_last_non_blank(cursor, content)
        }
        Motion::GotoFirstLine => {
            exec_goto_first_line(cursor)
        }
        Motion::GotoLastLine => {
            exec_goto_last_line(cursor, line_count)
        }
        Motion::GotoLine(n) => {
            exec_goto_line(cursor, *n, line_count)
        }
        Motion::GotoColumn(col) => {
            exec_goto_column(cursor, *col, content)
        }
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
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_left_right() {
        let content =
            BufferContent::from_str("hello\n");
        let mut cursor = CursorPosition::new(0, 2);
        execute_motion(
            &mut cursor,
            &Motion::Left,
            1,
            &content,
        );
        assert_eq!(cursor.grapheme_offset, 1);
        execute_motion(
            &mut cursor,
            &Motion::Right,
            1,
            &content,
        );
        assert_eq!(cursor.grapheme_offset, 2);
    }

    #[test]
    fn move_down_up() {
        let content =
            BufferContent::from_str("abc\ndef\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(
            &mut cursor,
            &Motion::Down,
            1,
            &content,
        );
        assert_eq!(cursor.line, 1);
        execute_motion(
            &mut cursor, &Motion::Up, 1, &content,
        );
        assert_eq!(cursor.line, 0);
    }

    #[test]
    fn goto_lines() {
        let content =
            BufferContent::from_str("a\nb\nc\nd\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(
            &mut cursor,
            &Motion::GotoLastLine,
            1,
            &content,
        );
        assert!(cursor.line >= 3);
    }
}
