//! Text manipulation utilities.

pub use crate::text_stats::{byte_count, char_count, line_count, word_count};

/// Joins lines with an optional separator.
pub fn join_lines(lines: &[&str], separator: &str) -> String {
    lines.join(separator)
}

/// Splits text into lines.
pub fn split_lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// Duplicates a line.
pub fn duplicate_line(line: &str) -> String {
    format!("{}\n{}", line, line)
}

/// Reverses a line.
pub fn reverse_line(line: &str) -> String {
    line.chars().rev().collect()
}

/// Reverses multiple lines.
pub fn reverse_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().rev().map(|s| s.to_string()).collect()
}

/// Sorts lines.
pub fn sort_lines(lines: &[&str]) -> Vec<String> {
    let mut sorted: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    sorted.sort();
    sorted
}

/// Sorts lines in reverse.
pub fn sort_lines_reverse(lines: &[&str]) -> Vec<String> {
    let mut sorted = sort_lines(lines);
    sorted.reverse();
    sorted
}

/// Removes duplicate adjacent lines.
pub fn uniq_lines(lines: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        if result.last().map(|l| l.as_str()) != Some(*line) {
            result.push(line.to_string());
        }
    }
    result
}

/// Removes trailing whitespace from lines.
pub fn strip_trailing_whitespace(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Normalizes line endings to LF.
pub fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Ensures text ends with a newline.
pub fn ensure_final_newline(text: &str) -> String {
    if text.is_empty() || text.ends_with('\n') {
        text.to_string()
    } else {
        format!("{}\n", text)
    }
}

/// Removes blank lines.
pub fn remove_blank_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Collapses multiple blank lines into single blank lines.
pub fn collapse_blank_lines(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut result: Vec<&str> = Vec::new();
    let mut prev_blank = false;

    for line in lines {
        let is_blank = line.trim().is_empty();
        if is_blank && prev_blank {
            continue;
        }
        result.push(line);
        prev_blank = is_blank;
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_lines() {
        assert_eq!(join_lines(&["a", "b", "c"], " "), "a b c");
    }

    #[test]
    fn test_split_lines() {
        assert_eq!(split_lines("a\nb\nc"), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_duplicate_line() {
        assert_eq!(duplicate_line("hello"), "hello\nhello");
    }

    #[test]
    fn test_reverse_line() {
        assert_eq!(reverse_line("hello"), "olleh");
    }

    #[test]
    fn test_reverse_lines() {
        assert_eq!(reverse_lines(&["a", "b", "c"]), vec!["c", "b", "a"]);
    }

    #[test]
    fn test_sort_lines() {
        assert_eq!(sort_lines(&["c", "a", "b"]), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_uniq_lines() {
        assert_eq!(
            uniq_lines(&["a", "a", "b", "b", "b", "c"]),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_strip_trailing_whitespace() {
        assert_eq!(strip_trailing_whitespace("foo  \nbar  "), "foo\nbar");
    }

    #[test]
    fn test_normalize_line_endings() {
        assert_eq!(normalize_line_endings("a\r\nb\rc"), "a\nb\nc");
    }

    #[test]
    fn test_ensure_final_newline() {
        assert_eq!(ensure_final_newline("hello"), "hello\n");
        assert_eq!(ensure_final_newline("hello\n"), "hello\n");
    }

    #[test]
    fn test_remove_blank_lines() {
        assert_eq!(remove_blank_lines("a\n\nb\n  \nc"), "a\nb\nc");
    }

    #[test]
    fn test_collapse_blank_lines() {
        assert_eq!(collapse_blank_lines("a\n\n\nb"), "a\n\nb");
    }
}
