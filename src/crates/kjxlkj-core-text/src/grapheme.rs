//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Count the number of grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Get the display width of a string.
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Convert a grapheme index to a byte offset.
pub fn grapheme_to_byte_offset(s: &str, grapheme_idx: usize) -> usize {
    s.grapheme_indices(true)
        .nth(grapheme_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Convert a grapheme index to a character offset.
/// If grapheme_idx exceeds the number of graphemes, returns the total char count.
pub fn grapheme_to_char_offset(s: &str, grapheme_idx: usize) -> usize {
    let mut char_count = 0;
    let mut grapheme_count = 0;
    for g in s.graphemes(true) {
        if grapheme_count >= grapheme_idx {
            break;
        }
        char_count += g.chars().count();
        grapheme_count += 1;
    }
    // If grapheme_idx >= total graphemes, return total char count
    if grapheme_count < grapheme_idx {
        s.chars().count()
    } else {
        char_count
    }
}

/// Convert a byte offset to a grapheme index.
pub fn byte_to_grapheme_offset(s: &str, byte_offset: usize) -> usize {
    s.grapheme_indices(true)
        .take_while(|(i, _)| *i < byte_offset)
        .count()
}

/// Get the grapheme at a specific index.
pub fn grapheme_at(s: &str, idx: usize) -> Option<&str> {
    s.graphemes(true).nth(idx)
}

/// Get a slice of graphemes from a string.
pub fn grapheme_slice(s: &str, start: usize, end: usize) -> String {
    s.graphemes(true).skip(start).take(end - start).collect()
}

/// Check if a character is a word character.
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Find the start of the current word.
pub fn word_start(s: &str, grapheme_idx: usize) -> usize {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    if grapheme_idx >= graphemes.len() {
        return graphemes.len();
    }

    let mut idx = grapheme_idx;
    while idx > 0 {
        let prev = graphemes[idx - 1];
        if let Some(c) = prev.chars().next() {
            if !is_word_char(c) {
                break;
            }
        }
        idx -= 1;
    }
    idx
}

/// Find the end of the current word.
pub fn word_end(s: &str, grapheme_idx: usize) -> usize {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    if grapheme_idx >= graphemes.len() {
        return graphemes.len();
    }

    let mut idx = grapheme_idx;
    while idx < graphemes.len() {
        let g = graphemes[idx];
        if let Some(c) = g.chars().next() {
            if !is_word_char(c) {
                break;
            }
        }
        idx += 1;
    }
    idx
}

/// Find the next word boundary.
pub fn next_word_start(s: &str, grapheme_idx: usize) -> usize {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    if grapheme_idx >= graphemes.len() {
        return graphemes.len();
    }

    let mut idx = grapheme_idx;

    // Skip current word
    while idx < graphemes.len() {
        if let Some(c) = graphemes[idx].chars().next() {
            if !is_word_char(c) {
                break;
            }
        }
        idx += 1;
    }

    // Skip non-word characters
    while idx < graphemes.len() {
        if let Some(c) = graphemes[idx].chars().next() {
            if is_word_char(c) {
                break;
            }
        }
        idx += 1;
    }

    idx
}

/// Find the previous word start.
pub fn prev_word_start(s: &str, grapheme_idx: usize) -> usize {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    if grapheme_idx == 0 {
        return 0;
    }

    let mut idx = grapheme_idx.min(graphemes.len());

    // Skip non-word characters before cursor
    while idx > 0 {
        if let Some(c) = graphemes[idx - 1].chars().next() {
            if is_word_char(c) {
                break;
            }
        }
        idx -= 1;
    }

    // Skip word characters
    while idx > 0 {
        if let Some(c) = graphemes[idx - 1].chars().next() {
            if !is_word_char(c) {
                break;
            }
        }
        idx -= 1;
    }

    idx
}
