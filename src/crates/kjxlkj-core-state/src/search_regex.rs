//! Search regex utilities: pattern compilation and matching.

use kjxlkj_core_types::EditorError;
use regex::Regex;

/// Compile a search pattern into a Regex, optionally case-insensitive.
pub fn compile_pattern(pattern: &str, case_sensitive: bool) -> Result<Regex, EditorError> {
    let translated = translate_vim_pattern(pattern);
    let full = if case_sensitive {
        translated
    } else {
        format!("(?i){translated}")
    };
    Regex::new(&full).map_err(|e| EditorError::InvalidRegex(e.to_string()))
}

/// Translate common Vim regex atoms to Rust regex syntax.
pub fn translate_vim_pattern(pattern: &str) -> String {
    let mut out = String::with_capacity(pattern.len());
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                '<' | '>' => {
                    out.push_str("\\b");
                    i += 2;
                }
                '(' => {
                    out.push('(');
                    i += 2;
                }
                ')' => {
                    out.push(')');
                    i += 2;
                }
                '|' => {
                    out.push('|');
                    i += 2;
                }
                '+' => {
                    out.push('+');
                    i += 2;
                }
                '?' => {
                    out.push('?');
                    i += 2;
                }
                '{' => {
                    out.push('{');
                    i += 2;
                }
                '}' => {
                    out.push('}');
                    i += 2;
                }
                _ => {
                    out.push('\\');
                    out.push(chars[i + 1]);
                    i += 2;
                }
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Find all non-overlapping matches in text, returning (byte_start, byte_end).
pub fn find_all_matches(text: &str, regex: &Regex) -> Vec<(usize, usize)> {
    regex.find_iter(text).map(|m| (m.start(), m.end())).collect()
}

/// Find next match at or after byte offset.
pub fn find_next(text: &str, regex: &Regex, offset: usize) -> Option<(usize, usize)> {
    let offset = offset.min(text.len());
    regex.find_at(text, offset).map(|m| (m.start(), m.end()))
}

/// Find previous match before byte offset.
pub fn find_prev(text: &str, regex: &Regex, offset: usize) -> Option<(usize, usize)> {
    let offset = offset.min(text.len());
    let mut last = None;
    for m in regex.find_iter(text) {
        if m.start() >= offset {
            break;
        }
        last = Some((m.start(), m.end()));
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_case_insensitive() {
        let re = compile_pattern("hello", false).unwrap();
        assert!(re.is_match("HELLO"));
    }

    #[test]
    fn compile_case_sensitive() {
        let re = compile_pattern("hello", true).unwrap();
        assert!(!re.is_match("HELLO"));
        assert!(re.is_match("hello"));
    }

    #[test]
    fn translate_word_boundary() {
        let r = translate_vim_pattern("\\<word\\>");
        assert_eq!(r, "\\bword\\b");
    }

    #[test]
    fn translate_groups() {
        let r = translate_vim_pattern("\\(a\\|b\\)");
        assert_eq!(r, "(a|b)");
    }

    #[test]
    fn find_all() {
        let re = Regex::new("ab").unwrap();
        let matches = find_all_matches("ababab", &re);
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn find_next_from_offset() {
        let re = Regex::new("x").unwrap();
        let m = find_next("..x..x", &re, 3).unwrap();
        assert_eq!(m.0, 5);
    }

    #[test]
    fn find_prev_before_offset() {
        let re = Regex::new("x").unwrap();
        let m = find_prev("x..x..", &re, 4).unwrap();
        assert_eq!(m.0, 3);
    }
}
