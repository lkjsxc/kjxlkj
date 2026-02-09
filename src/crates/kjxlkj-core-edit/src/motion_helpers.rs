//! Word and paragraph movement helpers for motions.

use crate::cursor::CursorPosition;
use kjxlkj_core_text::BufferContent;

pub(crate) fn move_word_forward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    big_word: bool,
) {
    let line = content.line_content(cursor.line);
    if let Some(next) =
        kjxlkj_core_text::find_word_boundary(&line, cursor.grapheme_offset, big_word)
    {
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

pub(crate) fn move_word_backward(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    big_word: bool,
) {
    let line = content.line_content(cursor.line);
    if let Some(prev) =
        kjxlkj_core_text::find_word_boundary(&line, cursor.grapheme_offset, big_word)
    {
        cursor.grapheme_offset = prev;
    } else if cursor.line > 0 {
        cursor.line -= 1;
        let gc = content.line_graphemes(cursor.line).count();
        cursor.grapheme_offset = if gc > 0 { gc - 1 } else { 0 };
    }
    cursor.clear_desired_col();
}

pub(crate) fn move_word_end(cursor: &mut CursorPosition, content: &BufferContent, big_word: bool) {
    let line = content.line_content(cursor.line);
    if let Some(end) = kjxlkj_core_text::find_word_boundary(&line, cursor.grapheme_offset, big_word)
    {
        cursor.grapheme_offset = end;
    }
    cursor.clear_desired_col();
}

pub(crate) fn move_paragraph_forward(cursor: &mut CursorPosition, content: &BufferContent) {
    let lc = content.line_count();
    let mut line = cursor.line + 1;
    while line < lc && !content.line_content(line).is_empty() {
        line += 1;
    }
    while line < lc && content.line_content(line).is_empty() {
        line += 1;
    }
    cursor.line = line.min(lc.saturating_sub(1));
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

pub(crate) fn move_paragraph_backward(cursor: &mut CursorPosition, content: &BufferContent) {
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

pub(crate) fn move_to_first_non_blank(cursor: &mut CursorPosition, content: &BufferContent) {
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
