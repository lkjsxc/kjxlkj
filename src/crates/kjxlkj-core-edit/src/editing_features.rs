//! Text manipulation features — join lines, change case, sort, trim.
//!
//! Provides higher-level editing operations that combine motions and
//! operators for common text transformation tasks.

/// Case conversion mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMode {
    Upper,
    Lower,
    Toggle,
    Title,
}

/// Result of a text manipulation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManipResult {
    pub text: String,
    pub lines_affected: usize,
}

/// Join multiple lines into one, replacing newlines with a separator.
pub fn join_lines(lines: &[&str], separator: &str) -> ManipResult {
    let count = lines.len();
    let joined = lines
        .iter()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join(separator);
    ManipResult { text: joined, lines_affected: count }
}

/// Convert text case according to the given mode.
pub fn convert_case(text: &str, mode: CaseMode) -> String {
    match mode {
        CaseMode::Upper => text.to_uppercase(),
        CaseMode::Lower => text.to_lowercase(),
        CaseMode::Toggle => text.chars().map(|c| {
            if c.is_uppercase() { c.to_lowercase().next().unwrap_or(c) }
            else { c.to_uppercase().next().unwrap_or(c) }
        }).collect(),
        CaseMode::Title => title_case(text),
    }
}

fn title_case(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut capitalize_next = true;
    for c in text.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            for uc in c.to_uppercase() { result.push(uc); }
            capitalize_next = false;
        } else {
            for lc in c.to_lowercase() { result.push(lc); }
        }
    }
    result
}

/// Sort lines alphabetically, optionally removing duplicates.
pub fn sort_lines(lines: &[&str], unique: bool, reverse: bool) -> Vec<String> {
    let mut sorted: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    sorted.sort();
    if unique {
        sorted.dedup();
    }
    if reverse {
        sorted.reverse();
    }
    sorted
}

/// Trim trailing whitespace from each line.
pub fn trim_trailing(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|l| l.trim_end().to_string()).collect()
}

/// Reverse character order within a string range.
pub fn reverse_chars(text: &str) -> String {
    text.chars().rev().collect()
}

/// Compute indentation level (number of leading spaces / tab_width).
pub fn indent_level(line: &str, tab_width: usize) -> usize {
    let tw = if tab_width == 0 { 4 } else { tab_width };
    let mut spaces = 0usize;
    for c in line.chars() {
        match c {
            ' ' => spaces += 1,
            '\t' => spaces += tw,
            _ => break,
        }
    }
    spaces / tw
}

/// Reindent a line to a target level.
pub fn reindent(line: &str, target_level: usize, tab_width: usize, use_tabs: bool) -> String {
    let content = line.trim_start();
    let indent = if use_tabs {
        "\t".repeat(target_level)
    } else {
        " ".repeat(target_level * tab_width)
    };
    format!("{}{}", indent, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_two_lines() {
        let r = join_lines(&["hello  ", "world"], " ");
        assert_eq!(r.text, "hello world");
        assert_eq!(r.lines_affected, 2);
    }

    #[test]
    fn case_upper() {
        assert_eq!(convert_case("hello", CaseMode::Upper), "HELLO");
    }

    #[test]
    fn case_toggle() {
        assert_eq!(convert_case("HeLLo", CaseMode::Toggle), "hEllO");
    }

    #[test]
    fn case_title() {
        assert_eq!(convert_case("hello world", CaseMode::Title), "Hello World");
    }

    #[test]
    fn sort_unique_reverse() {
        let sorted = sort_lines(&["b", "a", "b", "c"], true, true);
        assert_eq!(sorted, vec!["c", "b", "a"]);
    }

    #[test]
    fn trim_trailing_whitespace() {
        let trimmed = trim_trailing(&["hello  ", "world\t"]);
        assert_eq!(trimmed, vec!["hello", "world"]);
    }

    #[test]
    fn reverse_text() {
        assert_eq!(reverse_chars("abc"), "cba");
    }

    #[test]
    fn indent_detection() {
        assert_eq!(indent_level("    hello", 4), 1);
        assert_eq!(indent_level("\t\thello", 4), 2);
    }

    #[test]
    fn reindent_with_spaces() {
        let r = reindent("    hello", 2, 4, false);
        assert_eq!(r, "        hello");
    }
}
