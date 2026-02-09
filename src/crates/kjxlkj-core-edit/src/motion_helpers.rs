/// Word-motion and cursor helper functions for motions.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn line_graphemes_excl_newline(rope: &Rope, line: usize) -> usize {
    if line >= rope.len_lines() {
        return 0;
    }
    let slice = rope.line(line);
    let s: std::borrow::Cow<str> = slice.into();
    let trimmed = s.trim_end_matches(&['\n', '\r'][..]);
    trimmed.graphemes(true).count()
}

pub(crate) fn clamp_grapheme(rope: &Rope, line: usize, desired: usize) -> usize {
    let max = line_graphemes_excl_newline(rope, line);
    desired.min(max.saturating_sub(1))
}

pub(crate) fn first_non_blank(rope: &Rope, line: usize) -> usize {
    if line >= rope.len_lines() {
        return 0;
    }
    let slice = rope.line(line);
    let s: std::borrow::Cow<str> = slice.into();
    for (i, g) in s.graphemes(true).enumerate() {
        if !g.chars().all(|c| c == ' ' || c == '\t') {
            return i;
        }
    }
    0
}

pub(crate) fn word_forward(rope: &Rope, mut pos: CursorPosition, count: usize) -> CursorPosition {
    let line_count = rope.len_lines();
    for _ in 0..count {
        let max_g = line_graphemes_excl_newline(rope, pos.line);
        if pos.grapheme + 1 < max_g {
            pos.grapheme += 1;
            skip_word_chars(rope, &mut pos);
            skip_whitespace(rope, &mut pos);
        } else if pos.line + 1 < line_count {
            pos.line += 1;
            pos.grapheme = first_non_blank(rope, pos.line);
        }
    }
    pos
}

pub(crate) fn word_backward(rope: &Rope, mut pos: CursorPosition, count: usize) -> CursorPosition {
    for _ in 0..count {
        if pos.grapheme > 0 {
            pos.grapheme -= 1;
            skip_whitespace_back(rope, &mut pos);
            skip_word_chars_back(rope, &mut pos);
        } else if pos.line > 0 {
            pos.line -= 1;
            let max_g = line_graphemes_excl_newline(rope, pos.line);
            pos.grapheme = max_g.saturating_sub(1);
        }
    }
    pos
}

pub(crate) fn word_end_forward(
    rope: &Rope,
    mut pos: CursorPosition,
    count: usize,
) -> CursorPosition {
    let line_count = rope.len_lines();
    for _ in 0..count {
        let max_g = line_graphemes_excl_newline(rope, pos.line);
        if pos.grapheme + 1 < max_g {
            pos.grapheme += 1;
            skip_whitespace(rope, &mut pos);
            skip_word_chars_to_end(rope, &mut pos);
        } else if pos.line + 1 < line_count {
            pos.line += 1;
            pos.grapheme = 0;
            skip_whitespace(rope, &mut pos);
            skip_word_chars_to_end(rope, &mut pos);
        }
    }
    pos
}

fn skip_word_chars(rope: &Rope, pos: &mut CursorPosition) {
    let max_g = line_graphemes_excl_newline(rope, pos.line);
    while pos.grapheme < max_g {
        if let Some(g) = get_grapheme(rope, pos.line, pos.grapheme) {
            if g.chars().all(|c| c.is_alphanumeric() || c == '_') {
                pos.grapheme += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn skip_whitespace(rope: &Rope, pos: &mut CursorPosition) {
    let max_g = line_graphemes_excl_newline(rope, pos.line);
    while pos.grapheme < max_g {
        if let Some(g) = get_grapheme(rope, pos.line, pos.grapheme) {
            if g.chars().all(|c| c.is_whitespace()) {
                pos.grapheme += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn skip_word_chars_back(rope: &Rope, pos: &mut CursorPosition) {
    while pos.grapheme > 0 {
        if let Some(g) = get_grapheme(rope, pos.line, pos.grapheme) {
            if g.chars().all(|c| c.is_alphanumeric() || c == '_') {
                break;
            }
            pos.grapheme -= 1;
        } else {
            break;
        }
    }
}

fn skip_whitespace_back(rope: &Rope, pos: &mut CursorPosition) {
    while pos.grapheme > 0 {
        if let Some(g) = get_grapheme(rope, pos.line, pos.grapheme) {
            if g.chars().all(|c| c.is_whitespace()) {
                pos.grapheme -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn skip_word_chars_to_end(rope: &Rope, pos: &mut CursorPosition) {
    let max_g = line_graphemes_excl_newline(rope, pos.line);
    while pos.grapheme + 1 < max_g {
        let next = pos.grapheme + 1;
        if let Some(g) = get_grapheme(rope, pos.line, next) {
            if g.chars().all(|c| c.is_alphanumeric() || c == '_') {
                pos.grapheme = next;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn get_grapheme(rope: &Rope, line: usize, grapheme_idx: usize) -> Option<String> {
    if line >= rope.len_lines() {
        return None;
    }
    kjxlkj_core_text::nth_grapheme(rope.line(line), grapheme_idx)
}
