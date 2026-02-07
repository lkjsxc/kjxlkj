//! Text manipulation functions: join, case, sort, indent.

use serde::{Deserialize, Serialize};

/// Kind of case conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseKind {
    Upper,
    Lower,
    Toggle,
    Title,
}

/// Join multiple lines with a separator.
pub fn join_lines(lines: &[&str], separator: &str) -> String {
    lines.join(separator)
}

/// Convert the case of a string.
pub fn convert_case(s: &str, case: CaseKind) -> String {
    match case {
        CaseKind::Upper => s.to_uppercase(),
        CaseKind::Lower => s.to_lowercase(),
        CaseKind::Toggle => s
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            })
            .collect(),
        CaseKind::Title => {
            let mut result = String::with_capacity(s.len());
            let mut capitalize_next = true;
            for c in s.chars() {
                if c.is_whitespace() {
                    capitalize_next = true;
                    result.push(c);
                } else if capitalize_next {
                    result.extend(c.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.extend(c.to_lowercase());
                }
            }
            result
        }
    }
}

/// Sort lines in place, optionally removing duplicates and/or reversing.
pub fn sort_lines(lines: &mut Vec<String>, unique: bool, reverse: bool) {
    lines.sort();
    if unique {
        lines.dedup();
    }
    if reverse {
        lines.reverse();
    }
}

/// Remove trailing whitespace from a line.
pub fn trim_trailing(line: &str) -> String {
    line.trim_end().to_string()
}

/// Reverse the characters in a string.
pub fn reverse_chars(s: &str) -> String {
    s.chars().rev().collect()
}

/// Calculate indentation level (in units of tabstop).
pub fn indent_level(line: &str, tabstop: usize) -> usize {
    if tabstop == 0 {
        return 0;
    }
    let mut cols = 0usize;
    for c in line.chars() {
        match c {
            '\t' => cols += tabstop,
            ' ' => cols += 1,
            _ => break,
        }
    }
    cols / tabstop
}

/// Re-indent a line to a target indentation level.
pub fn reindent(line: &str, target_level: usize, use_tabs: bool, tabstop: usize) -> String {
    let content = line.trim_start();
    let prefix = if use_tabs {
        "\t".repeat(target_level)
    } else {
        " ".repeat(target_level * tabstop)
    };
    format!("{prefix}{content}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join() {
        assert_eq!(join_lines(&["a", "b", "c"], " "), "a b c");
    }

    #[test]
    fn case_toggle() {
        assert_eq!(convert_case("Hello", CaseKind::Toggle), "hELLO");
    }

    #[test]
    fn case_title() {
        assert_eq!(convert_case("hello world", CaseKind::Title), "Hello World");
    }

    #[test]
    fn sort_unique_reverse() {
        let mut v = vec!["b".into(), "a".into(), "b".into(), "c".into()];
        sort_lines(&mut v, true, true);
        assert_eq!(v, vec!["c", "b", "a"]);
    }

    #[test]
    fn trim() {
        assert_eq!(trim_trailing("hello   "), "hello");
    }

    #[test]
    fn reverse() {
        assert_eq!(reverse_chars("abc"), "cba");
    }

    #[test]
    fn indent() {
        assert_eq!(indent_level("\t\thello", 4), 2);
        assert_eq!(indent_level("        hello", 4), 2);
    }

    #[test]
    fn reindent_tabs() {
        assert_eq!(reindent("  hello", 2, true, 4), "\t\thello");
    }

    #[test]
    fn reindent_spaces() {
        assert_eq!(reindent("\thello", 1, false, 4), "    hello");
    }
}
