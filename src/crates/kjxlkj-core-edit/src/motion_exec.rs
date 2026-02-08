//! Motion execution: resolves a Motion type to a new cursor position.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::Motion;

use crate::cursor::CursorPosition;

/// Execute a motion and return the new cursor position.
///
/// The motion is applied `count` times. The current cursor position
/// is modified in-place and also returned.
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
        Motion::LineStart => {
            cursor.grapheme_offset = 0;
            cursor.clear_desired_col();
        }
        Motion::FirstNonBlank => {
            let line = content.line_content(cursor.line);
            let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
            let mut idx = 0;
            for i in 0..lg.count() {
                if let Some(g) = lg.get(i) {
                    let c = g.chars().next().unwrap_or(' ');
                    if !c.is_whitespace() {
                        idx = i;
                        break;
                    }
                }
            }
            cursor.grapheme_offset = idx;
            cursor.clear_desired_col();
        }
        Motion::LineEnd => {
            let gc = content.line_graphemes(cursor.line).count();
            cursor.grapheme_offset = if gc > 0 { gc - 1 } else { 0 };
            cursor.clear_desired_col();
        }
        Motion::LastNonBlank => {
            let line = content.line_content(cursor.line);
            let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
            let mut idx = if lg.count() > 0 { lg.count() - 1 } else { 0 };
            for i in (0..lg.count()).rev() {
                if let Some(g) = lg.get(i) {
                    let c = g.chars().next().unwrap_or(' ');
                    if !c.is_whitespace() {
                        idx = i;
                        break;
                    }
                }
            }
            cursor.grapheme_offset = idx;
            cursor.clear_desired_col();
        }
        Motion::GotoFirstLine => {
            cursor.line = 0;
            cursor.grapheme_offset = 0;
            cursor.clear_desired_col();
        }
        Motion::GotoLastLine => {
            cursor.line = if line_count > 0 {
                line_count - 1
            } else {
                0
            };
            cursor.grapheme_offset = 0;
            cursor.clear_desired_col();
        }
        Motion::GotoLine(n) => {
            let target = (*n).min(line_count.saturating_sub(1));
            cursor.line = target;
            cursor.grapheme_offset = 0;
            cursor.clear_desired_col();
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
        Motion::GotoColumn(col) => {
            let gc = content.line_graphemes(cursor.line).count();
            cursor.grapheme_offset = (*col).min(
                if gc > 0 { gc - 1 } else { 0 }
            );
            cursor.clear_desired_col();
        }
        // For motions that require additional state (search, find char etc.),
        // they are handled at the caller level.
        _ => {}
    }
}

fn apply_desired_col(
    cursor: &mut CursorPosition,
    content: &BufferContent,
) {
    let lg = content.line_graphemes(cursor.line);
    let gc = lg.count();
    if let Some(desired) = cursor.desired_col {
        cursor.grapheme_offset = lg.grapheme_at_col(desired)
            .min(if gc > 0 { gc - 1 } else { 0 });
    } else {
        // First vertical move: set desired col from current position
        let current_lg = content.line_graphemes(cursor.line);
        let col = current_lg
            .display_col_at(cursor.grapheme_offset)
            .unwrap_or(0);
        cursor.desired_col = Some(col);
        cursor.grapheme_offset = cursor.grapheme_offset
            .min(if gc > 0 { gc - 1 } else { 0 });
    }
}

fn move_word_forward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    big_word: bool,
) {
    let line = content.line_content(cursor.line);
    if let Some(next) = kjxlkj_core_text::find_word_boundary(
        &line,
        cursor.grapheme_offset,
        big_word,
    ) {
        let gc = content.line_graphemes(cursor.line).count();
        if next < gc {
            cursor.grapheme_offset = next;
        } else if cursor.line + 1 < content.line_count() {
            cursor.line += 1;
            cursor.grapheme_offset = 0;
        }
    } else if cursor.line + 1 < content.line_count() {
        cursor.line += 1;
        cursor.grapheme_offset = 0;
    }
    cursor.clear_desired_col();
}

fn move_word_backward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    big_word: bool,
) {
    let line = content.line_content(cursor.line);
    if let Some(prev) = kjxlkj_core_text::find_word_boundary(
        &line,
        cursor.grapheme_offset,
        big_word,
    ) {
        cursor.grapheme_offset = prev;
    } else if cursor.line > 0 {
        cursor.line -= 1;
        let gc = content.line_graphemes(cursor.line).count();
        cursor.grapheme_offset = if gc > 0 { gc - 1 } else { 0 };
    }
    cursor.clear_desired_col();
}

fn move_word_end(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    big_word: bool,
) {
    let line = content.line_content(cursor.line);
    if let Some(end) = kjxlkj_core_text::find_word_boundary(
        &line,
        cursor.grapheme_offset,
        big_word,
    ) {
        cursor.grapheme_offset = end;
    }
    cursor.clear_desired_col();
}

fn move_paragraph_forward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
) {
    let lc = content.line_count();
    let mut line = cursor.line + 1;
    // Skip non-empty lines
    while line < lc && !content.line_content(line).is_empty() {
        line += 1;
    }
    // Skip empty lines
    while line < lc && content.line_content(line).is_empty() {
        line += 1;
    }
    cursor.line = line.min(lc.saturating_sub(1));
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

fn move_paragraph_backward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
) {
    if cursor.line == 0 {
        cursor.grapheme_offset = 0;
        return;
    }
    let mut line = cursor.line.saturating_sub(1);
    while line > 0 && !content.line_content(line).is_empty() {
        line -= 1;
    }
    while line > 0 && content.line_content(line).is_empty() {
        line -= 1;
    }
    cursor.line = line;
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

fn move_to_first_non_blank(
    cursor: &mut CursorPosition,
    content: &BufferContent,
) {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
    for i in 0..lg.count() {
        if let Some(g) = lg.get(i) {
            let c = g.chars().next().unwrap_or(' ');
            if !c.is_whitespace() {
                cursor.grapheme_offset = i;
                return;
            }
        }
    }
    cursor.grapheme_offset = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_left_right() {
        let content = BufferContent::from_str("hello\n");
        let mut cursor = CursorPosition::new(0, 2);
        execute_motion(&mut cursor, &Motion::Left, 1, &content);
        assert_eq!(cursor.grapheme_offset, 1);
        execute_motion(&mut cursor, &Motion::Right, 1, &content);
        assert_eq!(cursor.grapheme_offset, 2);
    }

    #[test]
    fn move_down_up() {
        let content = BufferContent::from_str("abc\ndef\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::Down, 1, &content);
        assert_eq!(cursor.line, 1);
        execute_motion(&mut cursor, &Motion::Up, 1, &content);
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
