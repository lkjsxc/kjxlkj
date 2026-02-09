//! Text manipulation utilities: case toggling, line joining,
//! indentation, and sorting.

use kjxlkj_core_text::BufferContent;

/// Toggle the case of a single character.
pub fn case_toggle(c: char) -> char {
    if c.is_uppercase() {
        c.to_lowercase().next().unwrap_or(c)
    } else if c.is_lowercase() {
        c.to_uppercase().next().unwrap_or(c)
    } else {
        c
    }
}

/// Join lines `start..=end` with a space between each.
///
/// The cursor should remain on the original line.
pub fn join_lines(content: &mut BufferContent, start: usize, end: usize, with_space: bool) {
    let end = end.min(content.line_count().saturating_sub(1));
    if start >= end {
        return;
    }
    // Work from the end to avoid index shifting
    for line in (start + 1..=end).rev() {
        let current = content.line_content(line);
        let trimmed = current.trim_start().to_string();

        // Delete the line ending of the previous line and the current line
        let prev_gc = content.line_graphemes(line - 1).count();
        if prev_gc > 0 {
            // Remove the newline at end of previous line
            let prev_len = content.line_graphemes(line - 1).count();
            // Delete from end of prev line to start of current line
            content.delete(line - 1, prev_len, line, 0);
        }

        // Insert separator and trimmed content
        let new_gc = content.line_graphemes(line - 1).count();
        if with_space && !trimmed.is_empty() {
            content.insert(line - 1, new_gc, &format!(" {trimmed}"));
        } else {
            content.insert(line - 1, new_gc, &trimmed);
        }
    }
}

/// Indent a line by prepending spaces.
pub fn indent_line(content: &mut BufferContent, line: usize, indent: &str) {
    content.insert(line, 0, indent);
}

/// Sort lines in a range alphabetically.
pub fn sort_lines(content: &mut BufferContent, start: usize, end: usize) {
    let end = end.min(content.line_count());
    let mut lines: Vec<String> = (start..end).map(|i| content.line_content(i)).collect();
    lines.sort();

    // Replace the lines
    for (i, sorted) in lines.iter().enumerate() {
        let line = start + i;
        let gc = content.line_graphemes(line).count();
        if gc > 0 {
            content.delete(line, 0, line, gc);
        }
        content.insert(line, 0, sorted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_case() {
        assert_eq!(case_toggle('a'), 'A');
        assert_eq!(case_toggle('A'), 'a');
        assert_eq!(case_toggle('1'), '1');
    }

    #[test]
    fn indent() {
        let mut content = BufferContent::from_str("hello\n");
        indent_line(&mut content, 0, "    ");
        assert_eq!(content.line_content(0), "    hello");
    }
}
