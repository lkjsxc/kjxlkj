//! Motion execution against a buffer.

use crate::cursor::Cursor;
use crate::motion_big_word;
use crate::motion_find;
use crate::motion_word;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::Motion;
use unicode_segmentation::UnicodeSegmentation;

/// Apply a motion to a cursor, returning the new cursor position.
pub fn apply_motion(
    cursor: &Cursor,
    motion: &Motion,
    buffer: &Buffer,
) -> Cursor {
    let mut c = *cursor;
    match motion {
        Motion::Left => {
            if c.col > 0 { c.col -= 1; }
            c.desired_col = c.col;
        }
        Motion::Right => {
            let max = line_max_col(buffer, c.line);
            if c.col < max { c.col += 1; }
            c.desired_col = c.col;
        }
        Motion::Up => {
            if c.line > 0 {
                c.line -= 1;
                c.col = c.desired_col.min(line_max_col(buffer, c.line));
            }
        }
        Motion::Down => {
            if c.line + 1 < buffer.line_count() {
                c.line += 1;
                c.col = c.desired_col.min(line_max_col(buffer, c.line));
            }
        }
        Motion::LineStart => { c.col = 0; c.desired_col = 0; }
        Motion::LineEnd => {
            c.col = line_max_col(buffer, c.line);
            c.desired_col = usize::MAX;
        }
        Motion::FirstNonBlank => {
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::LastNonBlank => {
            c.col = last_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::GotoFirstLine => {
            c.line = 0;
            c.col = first_nonblank_col(buffer, 0);
            c.desired_col = c.col;
        }
        Motion::GotoLastLine => {
            c.line = buffer.line_count().saturating_sub(1);
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::GotoLine(n) => {
            let target = (*n).min(buffer.line_count()).saturating_sub(1);
            c.line = target;
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::WordForward => motion_word::word_forward(&mut c, buffer),
        Motion::WordBackward => motion_word::word_backward(&mut c, buffer),
        Motion::WordEndForward => motion_word::word_end_forward(&mut c, buffer),
        Motion::WordEndBackward => motion_word::word_end_backward(&mut c, buffer),
        Motion::BigWordForward => motion_big_word::big_word_forward(&mut c, buffer),
        Motion::BigWordBackward => motion_big_word::big_word_backward(&mut c, buffer),
        Motion::BigWordEndForward => motion_big_word::big_word_end_forward(&mut c, buffer),
        Motion::BigWordEndBackward => motion_big_word::big_word_end_backward(&mut c, buffer),
        Motion::ParagraphForward => motion_find::paragraph_forward(&mut c, buffer),
        Motion::ParagraphBackward => motion_find::paragraph_backward(&mut c, buffer),
        Motion::SentenceForward | Motion::SentenceBackward => {}
        Motion::FindForward(ch) => motion_find::find_char_forward(&mut c, buffer, *ch),
        Motion::FindBackward(ch) => motion_find::find_char_backward(&mut c, buffer, *ch),
        Motion::TillForward(ch) => {
            let before = c;
            motion_find::find_char_forward(&mut c, buffer, *ch);
            if c.col != before.col && c.col > 0 { c.col -= 1; }
        }
        Motion::TillBackward(ch) => {
            let before = c;
            motion_find::find_char_backward(&mut c, buffer, *ch);
            if c.col != before.col { c.col += 1; }
        }
        Motion::MatchParen => motion_find::match_paren(&mut c, buffer),
        _ => { /* Window/scroll/search: editor level */ }
    }
    c
}

/// Max cursor column on a line (0-based, last grapheme).
pub(crate) fn line_max_col(buffer: &Buffer, line: usize) -> usize {
    let text = buffer.line(line).unwrap_or_default();
    let trimmed = text.trim_end_matches('\n');
    let visible: usize = trimmed.graphemes(true).count();
    if visible > 0 { visible - 1 } else { 0 }
}

/// First non-blank grapheme column on a line.
pub(crate) fn first_nonblank_col(buffer: &Buffer, line: usize) -> usize {
    let text = buffer.line(line).unwrap_or_default();
    for (i, g) in text.graphemes(true).enumerate() {
        if !g.chars().all(|c| c == ' ' || c == '\t') {
            return i;
        }
    }
    0
}

/// Last non-blank grapheme column on a line (g_).
pub(crate) fn last_nonblank_col(buffer: &Buffer, line: usize) -> usize {
    let text = buffer.line(line).unwrap_or_default();
    let trimmed = text.trim_end_matches('\n');
    let gs: Vec<&str> = trimmed.graphemes(true).collect();
    for i in (0..gs.len()).rev() {
        if !gs[i].chars().all(|c| c == ' ' || c == '\t') {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn motion_left_at_zero() {
        let b = Buffer::from_text(BufferId(0), "t", "hello");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::Left, &b);
        assert_eq!(nc.col, 0);
    }

    #[test]
    fn motion_right_clamps() {
        let b = Buffer::from_text(BufferId(0), "t", "hi");
        let c = Cursor::new(0, 1);
        let nc = apply_motion(&c, &Motion::Right, &b);
        assert_eq!(nc.col, 1);
    }

    #[test]
    fn motion_down() {
        let b = Buffer::from_text(BufferId(0), "t", "a\nb");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::Down, &b);
        assert_eq!(nc.line, 1);
    }

    #[test]
    fn goto_last_line() {
        let b = Buffer::from_text(BufferId(0), "t", "a\nb\nc");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::GotoLastLine, &b);
        assert_eq!(nc.line, 2);
    }

    #[test]
    fn find_forward_char() {
        let b = Buffer::from_text(BufferId(0), "t", "abcdef");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::FindForward('d'), &b);
        assert_eq!(nc.col, 3);
    }

    #[test]
    fn match_paren_forward() {
        let b = Buffer::from_text(BufferId(0), "t", "(ab)cd");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::MatchParen, &b);
        assert_eq!(nc.col, 3);
    }

    #[test]
    fn match_paren_scans_forward_for_bracket() {
        // When cursor is not on a bracket, % scans forward.
        let b = Buffer::from_text(BufferId(0), "t", "ab(cd)ef");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::MatchParen, &b);
        // Scans forward from col 0, finds '(' at col 2, jumps to ')' at col 5.
        assert_eq!(nc.col, 5);
    }

    #[test]
    fn paragraph_forward_to_blank() {
        let b = Buffer::from_text(BufferId(0), "t", "abc\ndef\n\nghi");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::ParagraphForward, &b);
        assert_eq!(nc.line, 2);
    }
}
