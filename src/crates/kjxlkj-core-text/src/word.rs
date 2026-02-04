//! Word boundary utilities.

/// Check if a character is a word character.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Find the start of a word.
pub fn find_word_start(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if col == 0 || chars.is_empty() {
        return 0;
    }
    let col = col.min(chars.len());
    let mut pos = col.saturating_sub(1);
    // Skip whitespace
    while pos > 0 && chars[pos].is_whitespace() {
        pos -= 1;
    }
    // Find word boundary
    let in_word = is_word_char(chars[pos]);
    while pos > 0 {
        let prev_in_word = is_word_char(chars[pos - 1]);
        if prev_in_word != in_word {
            break;
        }
        pos -= 1;
    }
    pos
}

/// Find the end of a word.
pub fn find_word_end(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return 0;
    }
    let col = col.min(chars.len().saturating_sub(1));
    let mut pos = col;
    // Skip whitespace first
    while pos < chars.len() && chars[pos].is_whitespace() {
        pos += 1;
    }
    if pos >= chars.len() {
        return chars.len().saturating_sub(1);
    }
    // Find word boundary
    let in_word = is_word_char(chars[pos]);
    while pos + 1 < chars.len() {
        let next_in_word = is_word_char(chars[pos + 1]);
        if next_in_word != in_word || chars[pos + 1].is_whitespace() {
            break;
        }
        pos += 1;
    }
    pos
}

/// Find the next word boundary (start of next word).
pub fn next_word_boundary(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() || col >= chars.len() {
        return chars.len();
    }
    let mut pos = col;
    // Skip current word
    if pos < chars.len() && !chars[pos].is_whitespace() {
        let in_word = is_word_char(chars[pos]);
        while pos < chars.len() && !chars[pos].is_whitespace() {
            if is_word_char(chars[pos]) != in_word {
                break;
            }
            pos += 1;
        }
    }
    // Skip whitespace
    while pos < chars.len() && chars[pos].is_whitespace() {
        pos += 1;
    }
    pos
}

/// Find the previous word boundary (start of current/previous word).
pub fn prev_word_boundary(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() || col == 0 {
        return 0;
    }
    let mut pos = col.min(chars.len()).saturating_sub(1);
    // Skip whitespace
    while pos > 0 && chars[pos].is_whitespace() {
        pos -= 1;
    }
    // Skip current word
    if pos > 0 && !chars[pos].is_whitespace() {
        let in_word = is_word_char(chars[pos]);
        while pos > 0 && !chars[pos - 1].is_whitespace() {
            if is_word_char(chars[pos - 1]) != in_word {
                break;
            }
            pos -= 1;
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_word() {
        assert_eq!(next_word_boundary("hello world", 0), 6);
        assert_eq!(next_word_boundary("hello world", 5), 6);
        assert_eq!(next_word_boundary("hello world", 6), 11);
    }

    #[test]
    fn test_prev_word() {
        assert_eq!(prev_word_boundary("hello world", 11), 6);
        assert_eq!(prev_word_boundary("hello world", 6), 0);
        assert_eq!(prev_word_boundary("hello world", 3), 0);
    }

    #[test]
    fn test_find_word_end() {
        assert_eq!(find_word_end("hello world", 0), 4);
        assert_eq!(find_word_end("hello world", 6), 10);
    }
}
