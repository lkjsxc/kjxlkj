//! Vim magic regex translation to Rust regex syntax.
//!
//! Vim "magic" mode treats some characters as special by default,
//! while others require a backslash. This module translates Vim
//! patterns to the Rust `regex` crate syntax.

/// Translate a Vim magic-mode pattern to a Rust regex pattern.
///
/// Key mappings:
/// - `\+` → `+`  (one or more)
/// - `\?` or `\=` → `?`  (zero or one)
/// - `\|` → `|`  (alternation)
/// - `\(` ... `\)` → `(` ... `)` (group)
/// - `\<` and `\>` → `\b` (word boundary)
/// - `\{n,m}` → `{n,m}` (quantifier)
/// - `\c` → case-insensitive flag
/// - `\C` → case-sensitive flag
/// - Unescaped `(` `|` `+` `?` `{` `}` are literals
pub fn translate_vim_to_rust(pattern: &str) -> TranslateResult {
    let mut out = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();
    let mut case_override = None;
    let mut group_starts: Vec<usize> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('+') => out.push('+'),
                Some('?') | Some('=') => out.push('?'),
                Some('|') => out.push('|'),
                Some('(') => {
                    group_starts.push(out.len());
                    out.push('(');
                }
                Some(')') => {
                    out.push(')');
                    if let Some(start) = group_starts.pop() {
                        apply_lookaround(&mut chars, &mut out, start);
                    }
                }
                Some('<') => out.push_str("\\b"),
                Some('>') => out.push_str("\\b"),
                Some('{') => {
                    // Check for \{-} non-greedy quantifier.
                    if chars.peek() == Some(&'-') {
                        let mut probe = chars.clone();
                        probe.next(); // consume '-'
                        if probe.peek() == Some(&'}') {
                            probe.next(); // consume '}'
                            chars = probe;
                            out.push_str("*?"); // non-greedy
                        } else {
                            out.push('{');
                            consume_until(&mut chars, &mut out, '}');
                            out.push('}');
                        }
                    } else {
                        out.push('{');
                        consume_until(&mut chars, &mut out, '}');
                        out.push('}');
                    }
                }
                Some('d') => out.push_str("\\d"),
                Some('D') => out.push_str("\\D"),
                Some('w') => out.push_str("\\w"),
                Some('W') => out.push_str("\\W"),
                Some('s') => out.push_str("\\s"),
                Some('S') => out.push_str("\\S"),
                Some('n') => out.push_str("\\n"),
                Some('t') => out.push_str("\\t"),
                Some('r') => out.push_str("\\r"),
                Some('c') => case_override = Some(false),
                Some('C') => case_override = Some(true),
                // Multi-line class atoms: \_s, \_d, \_w, \_.
                Some('_') => match chars.next() {
                    Some('s') => out.push_str("[\\s\\n]"),
                    Some('S') => out.push_str("[^\\s]"),
                    Some('d') => out.push_str("[\\d\\n]"),
                    Some('w') => out.push_str("[\\w\\n]"),
                    Some('.') => out.push_str("(?s:.)"),
                    Some(other) => { out.push_str("\\_"); out.push(other); }
                    None => out.push_str("\\_"),
                },
                // \%[abc] collection → [abc], \%(…\) non-capturing group → (?:…)
                Some('%') => match chars.peek() {
                    Some('[') => { chars.next(); out.push('['); consume_until(&mut chars, &mut out, ']'); out.push(']'); }
                    Some('(') => { chars.next(); group_starts.push(out.len()); out.push_str("(?:"); }
                    _ => { out.push_str("\\%"); }
                },
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            // In magic mode, these are literal (need escaping for regex)
            match c {
                '(' | ')' | '|' | '+' | '?' | '{' | '}' => {
                    out.push('\\');
                    out.push(c);
                }
                _ => out.push(c),
            }
        }
    }

    TranslateResult {
        pattern: out,
        case_override,
    }
}

/// Result of translating a Vim pattern.
pub struct TranslateResult {
    /// Rust regex pattern.
    pub pattern: String,
    /// Case override from \c (false) or \C (true); None=use option.
    pub case_override: Option<bool>,
}

fn consume_until(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    out: &mut String,
    end: char,
) {
    for c in chars.by_ref() {
        if c == end {
            return;
        }
        out.push(c);
    }
}

/// Check for `\@=`, `\@!`, `\@<=`, `\@<!` after a group close
/// and convert the group to a Rust regex lookaround.
fn apply_lookaround(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    out: &mut String,
    group_start: usize,
) {
    // Check if next is \@
    if chars.peek() != Some(&'\\') {
        return;
    }
    let mut probe = chars.clone();
    probe.next(); // consume '\'
    if probe.peek() != Some(&'@') {
        return;
    }
    probe.next(); // consume '@'
    let prefix = match probe.peek() {
        Some('=') => {
            probe.next();
            "(?="
        }
        Some('!') => {
            probe.next();
            "(?!"
        }
        Some('>') => {
            probe.next();
            // Atomic group: Rust regex doesn't support (?>...) natively,
            // translate to non-capturing group (?:...) as best approximation.
            "(?:"
        }
        Some('<') => {
            probe.next();
            match probe.peek() {
                Some('=') => {
                    probe.next();
                    "(?<="
                }
                Some('!') => {
                    probe.next();
                    "(?<!"
                }
                _ => return,
            }
        }
        _ => return,
    };
    // Commit: advance real iterator to match probe position.
    *chars = probe;
    // Replace the '(' at group_start with the lookaround prefix.
    out.replace_range(group_start..group_start + 1, prefix);
}

/// Compile a Vim magic pattern to a regex::Regex.
/// Returns None if the pattern is invalid.
pub fn compile_vim_pattern(pattern: &str, case_sensitive: bool) -> Option<regex::Regex> {
    let result = translate_vim_to_rust(pattern);
    let effective_case = result.case_override.unwrap_or(case_sensitive);
    let final_pattern = if !effective_case {
        format!("(?i){}", result.pattern)
    } else {
        result.pattern
    };
    regex::Regex::new(&final_pattern).ok()
}
