//! Word classification and boundary detection.
//!
//! Implements Vim-style word (`w`) and WORD (`W`) boundaries.

/// Word character classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordKind {
    /// Whitespace characters.
    Whitespace,
    /// Word characters (alphanumeric + underscore).
    Word,
    /// Punctuation / symbol characters.
    Punctuation,
}

/// Classify a character into a word kind.
///
/// - Whitespace: space, tab, newline, etc.
/// - Word: alphanumeric or underscore.
/// - Punctuation: everything else.
pub fn classify_word_char(c: char) -> WordKind {
    if c.is_whitespace() {
        WordKind::Whitespace
    } else if c.is_alphanumeric() || c == '_' {
        WordKind::Word
    } else {
        WordKind::Punctuation
    }
}

/// Find the start of the next word in a line from a given grapheme index.
///
/// Returns `None` if at end of line.
pub fn find_word_boundary(
    line: &str,
    grapheme_idx: usize,
    big_word: bool,
) -> Option<usize> {
    use unicode_segmentation::UnicodeSegmentation;
    let graphemes: Vec<&str> = line.graphemes(true).collect();
    if grapheme_idx >= graphemes.len() {
        return None;
    }

    let current_kind = first_char_kind(graphemes[grapheme_idx], big_word);
    let mut idx = grapheme_idx + 1;

    // Skip same-kind chars
    while idx < graphemes.len() {
        let kind = first_char_kind(graphemes[idx], big_word);
        if kind != current_kind {
            break;
        }
        idx += 1;
    }

    // Skip whitespace
    while idx < graphemes.len() {
        let kind = first_char_kind(graphemes[idx], big_word);
        if kind != effective_whitespace() {
            break;
        }
        idx += 1;
    }

    if idx <= graphemes.len() {
        Some(idx)
    } else {
        None
    }
}

/// Find the end of the current/next word.
pub fn find_word_end(
    line: &str,
    grapheme_idx: usize,
    big_word: bool,
) -> Option<usize> {
    use unicode_segmentation::UnicodeSegmentation;
    let graphemes: Vec<&str> = line.graphemes(true).collect();
    if grapheme_idx >= graphemes.len() {
        return None;
    }

    let mut idx = grapheme_idx + 1;

    // Skip whitespace
    while idx < graphemes.len() {
        let kind = first_char_kind(graphemes[idx], big_word);
        if kind != effective_whitespace() {
            break;
        }
        idx += 1;
    }

    if idx >= graphemes.len() {
        return Some(graphemes.len().saturating_sub(1));
    }

    let current_kind = first_char_kind(graphemes[idx], big_word);

    // Skip same-kind chars until end
    while idx + 1 < graphemes.len() {
        let kind = first_char_kind(graphemes[idx + 1], big_word);
        if kind != current_kind {
            break;
        }
        idx += 1;
    }

    Some(idx)
}

/// Find the start of the current/previous word.
pub fn find_word_start_backward(
    line: &str,
    grapheme_idx: usize,
    big_word: bool,
) -> Option<usize> {
    use unicode_segmentation::UnicodeSegmentation;
    let graphemes: Vec<&str> = line.graphemes(true).collect();
    if grapheme_idx == 0 || graphemes.is_empty() {
        return None;
    }

    let mut idx = grapheme_idx - 1;

    // Skip whitespace
    while idx > 0 {
        let kind = first_char_kind(graphemes[idx], big_word);
        if kind != effective_whitespace() {
            break;
        }
        idx -= 1;
    }

    let current_kind = first_char_kind(graphemes[idx], big_word);

    // Go backward through same-kind chars
    while idx > 0 {
        let kind = first_char_kind(graphemes[idx - 1], big_word);
        if kind != current_kind {
            break;
        }
        idx -= 1;
    }

    Some(idx)
}

fn first_char_kind(grapheme: &str, big_word: bool) -> WordKind {
    let c = grapheme.chars().next().unwrap_or(' ');
    if big_word {
        if c.is_whitespace() {
            WordKind::Whitespace
        } else {
            WordKind::Word
        }
    } else {
        classify_word_char(c)
    }
}

fn effective_whitespace() -> WordKind {
    WordKind::Whitespace
}
