//! Vim regex (magic mode) to Rust regex translator.
//! See /docs/spec/editing/regex/magic-modes.md.
//!
//! Translates Vim's default "magic" mode regex syntax
//! into Rust `regex` crate syntax. Handles mode switches
//! (\v, \m, \M, \V) mid-pattern and Vim-specific atoms.

/// Compile a Vim regex pattern (magic mode default)
/// into a Rust regex pattern string.
pub fn vim_to_rust_regex(pat: &str) -> Result<String, String> {
    if pat.is_empty() {
        return Err("empty pattern".into());
    }
    let chars: Vec<char> = pat.chars().collect();
    let mut out = String::with_capacity(pat.len() * 2);
    let mut i = 0;
    let mut very_magic = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            i += 1;
            let nc = chars[i];
            match nc {
                'v' => { very_magic = true; }
                'm' => { very_magic = false; }
                'M' | 'V' => { very_magic = false; }
                // Vim shortcut atoms.
                'd' => out.push_str("[0-9]"),
                'D' => out.push_str("[^0-9]"),
                'w' => out.push_str("[0-9A-Za-z_]"),
                'W' => out.push_str("[^0-9A-Za-z_]"),
                's' => out.push_str("[ \\t]"),
                'S' => out.push_str("[^ \\t]"),
                'a' => out.push_str("[A-Za-z]"),
                'A' => out.push_str("[^A-Za-z]"),
                'l' => out.push_str("[a-z]"),
                'L' => out.push_str("[^a-z]"),
                'u' => out.push_str("[A-Z]"),
                'U' => out.push_str("[^A-Z]"),
                'x' => out.push_str("[0-9A-Fa-f]"),
                'X' => out.push_str("[^0-9A-Fa-f]"),
                'h' => out.push_str("[A-Za-z_]"),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'e' => out.push('\x1b'),
                '<' => out.push_str("\\b"),
                '>' => out.push_str("\\b"),
                // Magic mode: these need backslash to be special.
                '+' if !very_magic => out.push('+'),
                '?' if !very_magic => out.push('?'),
                '=' if !very_magic => out.push('?'),
                '|' if !very_magic => out.push('|'),
                '(' if !very_magic => out.push('('),
                ')' if !very_magic => out.push(')'),
                '{' if !very_magic => {
                    // \{n,m} → {n,m}
                    if let Some(end) = find_brace_end(&chars, i + 1) {
                        let inner: String = chars[i + 1..=end].iter().collect();
                        out.push('{');
                        // Handle Vim lazy prefix \{-
                        if inner.starts_with('-') {
                            let rest = &inner[1..inner.len() - 1];
                            if rest.is_empty() {
                                out.push_str("0,}?");
                            } else {
                                out.push_str(rest);
                                out.push_str("}?");
                            }
                        } else {
                            out.push_str(&inner);
                        }
                        if !inner.starts_with('-') {
                            // closing brace already in inner
                        }
                        i = end;
                    } else {
                        out.push_str("\\{");
                    }
                }
                // Back-references.
                c @ '1'..='9' => {
                    out.push('\\');
                    out.push(c);
                }
                // Literal escape.
                c => {
                    if is_regex_meta(c) {
                        out.push('\\');
                    }
                    out.push(c);
                }
            }
        } else if very_magic {
            // Very magic: most punct is special already.
            match c {
                '(' | ')' | '|' | '+' | '?' | '{' | '}' => {
                    out.push(c);
                }
                '.' | '*' | '^' | '$' | '[' | ']' => {
                    out.push(c);
                }
                _ => {
                    if is_regex_meta(c) {
                        out.push('\\');
                    }
                    out.push(c);
                }
            }
        } else {
            // Default magic mode.
            match c {
                '.' | '*' | '^' | '$' | '[' => {
                    out.push(c);
                }
                // These are literal in magic mode.
                '+' | '?' | '(' | ')' | '|' | '{' | '}' => {
                    out.push('\\');
                    out.push(c);
                }
                _ => {
                    if is_regex_meta(c) {
                        out.push('\\');
                    }
                    out.push(c);
                }
            }
        }
        i += 1;
    }
    Ok(out)
}

fn is_regex_meta(c: char) -> bool {
    matches!(c, '\\' | '.' | '*' | '+' | '?' | '(' | ')'
        | '[' | ']' | '{' | '}' | '^' | '$' | '|')
}

fn find_brace_end(chars: &[char], start: usize) -> Option<usize> {
    for i in start..chars.len() {
        if chars[i] == '}' {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_literal() { assert_eq!(vim_to_rust_regex("hello").unwrap(), "hello"); }

    #[test]
    fn dot_star() { assert_eq!(vim_to_rust_regex(".*").unwrap(), ".*"); }

    #[test]
    fn vim_plus() { assert_eq!(vim_to_rust_regex(r"\d\+").unwrap(), "[0-9]+"); }

    #[test]
    fn vim_group_alternation() {
        assert_eq!(vim_to_rust_regex(r"\(foo\|bar\)").unwrap(), "(foo|bar)");
    }

    #[test]
    fn very_magic_group() {
        assert_eq!(vim_to_rust_regex(r"\v(foo|bar)+").unwrap(), "(foo|bar)+");
    }

    #[test]
    fn word_boundary() { assert_eq!(vim_to_rust_regex(r"\<word\>").unwrap(), r"\bword\b"); }

    #[test]
    fn shortcut_atoms() {
        assert_eq!(vim_to_rust_regex(r"\w\s\d").unwrap(), "[0-9A-Za-z_][ \\t][0-9]");
    }

    #[test]
    fn empty_pattern_err() { assert!(vim_to_rust_regex("").is_err()); }

    #[test]
    fn literal_parens_magic() { assert_eq!(vim_to_rust_regex("f(x)").unwrap(), r"f\(x\)"); }

    #[test]
    fn anchors() { assert_eq!(vim_to_rust_regex("^start$").unwrap(), "^start$"); }
}
