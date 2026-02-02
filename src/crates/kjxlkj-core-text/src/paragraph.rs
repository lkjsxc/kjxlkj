//! Paragraph motion utilities.
//!
//! Helper functions for paragraph-based cursor movement ({ and }).

/// Finds the next paragraph start (} motion).
/// A paragraph boundary is a blank line.
pub fn next_paragraph(text: &str, line: usize) -> Option<usize> {
    let lines: Vec<&str> = text.lines().collect();
    if line >= lines.len() {
        return None;
    }

    let mut i = line;

    // Skip non-blank lines.
    while i < lines.len() && !lines[i].trim().is_empty() {
        i += 1;
    }

    // Skip blank lines.
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }

    if i < lines.len() { Some(i) } else { None }
}

/// Finds the previous paragraph start ({ motion).
pub fn prev_paragraph(text: &str, line: usize) -> Option<usize> {
    let lines: Vec<&str> = text.lines().collect();
    if line == 0 || lines.is_empty() {
        return Some(0);
    }

    let mut i = line.min(lines.len()).saturating_sub(1);

    // Skip blank lines above cursor.
    while i > 0 && lines[i].trim().is_empty() {
        i -= 1;
    }

    // Skip non-blank lines.
    while i > 0 && !lines[i - 1].trim().is_empty() {
        i -= 1;
    }

    // We're at start of paragraph or at first blank line.
    Some(i)
}

/// Finds paragraph range containing the given line.
pub fn paragraph_range(text: &str, line: usize) -> (usize, usize) {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return (0, 0);
    }

    let line = line.min(lines.len().saturating_sub(1));

    // If on blank line, return just that line.
    if lines[line].trim().is_empty() {
        return (line, line + 1);
    }

    // Find start of paragraph.
    let mut start = line;
    while start > 0 && !lines[start - 1].trim().is_empty() {
        start -= 1;
    }

    // Find end of paragraph.
    let mut end = line;
    while end < lines.len() && !lines[end].trim().is_empty() {
        end += 1;
    }

    (start, end)
}

/// Checks if a line is blank (empty or whitespace only).
pub fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}

/// Counts paragraphs in text.
pub fn paragraph_count(text: &str) -> usize {
    let mut count = 0;
    let mut in_paragraph = false;

    for line in text.lines() {
        if line.trim().is_empty() {
            if in_paragraph {
                in_paragraph = false;
            }
        } else if !in_paragraph {
            in_paragraph = true;
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_paragraph_simple() {
        let text = "line1\nline2\n\nline3\nline4";
        assert_eq!(next_paragraph(text, 0), Some(3));
    }

    #[test]
    fn test_next_paragraph_from_blank() {
        let text = "line1\n\nline2";
        assert_eq!(next_paragraph(text, 1), Some(2));
    }

    #[test]
    fn test_prev_paragraph() {
        let text = "line1\n\nline2\nline3";
        assert_eq!(prev_paragraph(text, 3), Some(2));
    }

    #[test]
    fn test_paragraph_range() {
        let text = "para1\npara1\n\npara2\npara2";
        assert_eq!(paragraph_range(text, 0), (0, 2));
        assert_eq!(paragraph_range(text, 3), (3, 5));
    }

    #[test]
    fn test_is_blank_line() {
        assert!(is_blank_line(""));
        assert!(is_blank_line("   "));
        assert!(is_blank_line("\t"));
        assert!(!is_blank_line("text"));
    }

    #[test]
    fn test_paragraph_count() {
        let text = "para1\n\npara2\n\npara3";
        assert_eq!(paragraph_count(text), 3);
    }

    #[test]
    fn test_paragraph_count_no_blanks() {
        let text = "line1\nline2\nline3";
        assert_eq!(paragraph_count(text), 1);
    }
}
