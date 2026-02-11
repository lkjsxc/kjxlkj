//! Vim regex (magic mode) to Rust regex translator.
//! See /docs/spec/editing/regex/magic-modes.md.

/// Compile a Vim regex (magic mode) into a Rust regex string.
pub fn vim_to_rust_regex(pat: &str) -> Result<String, String> {
    Ok(vim_to_rust_regex_ex(pat)?.0)
}

/// Extended: returns (pattern, case_flag). case_flag: Some(true)=\c, Some(false)=\C.
pub fn vim_to_rust_regex_ex(pat: &str) -> Result<(String, Option<bool>), String> {
    if pat.is_empty() { return Err("empty pattern".into()); }
    let chars: Vec<char> = pat.chars().collect();
    let mut out = String::with_capacity(pat.len() * 2);
    let (mut i, mut vm) = (0, false); // vm = very_magic
    let mut case_flag: Option<bool> = None;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            i += 1;
            emit_escaped(&chars, &mut i, &mut out, &mut vm, &mut case_flag);
        } else if vm {
            match c {
                '(' | ')' | '|' | '+' | '?' | '{' | '}' | '.' | '*'
                | '^' | '$' | '[' | ']' => out.push(c),
                _ => { if is_meta(c) { out.push('\\'); } out.push(c); }
            }
        } else {
            match c {
                '.' | '*' | '^' | '$' | '[' => out.push(c),
                '+' | '?' | '(' | ')' | '|' | '{' | '}' => { out.push('\\'); out.push(c); }
                _ => { if is_meta(c) { out.push('\\'); } out.push(c); }
            }
        }
        i += 1;
    }
    if case_flag == Some(true) { out = format!("(?i){out}"); }
    Ok((out, case_flag))
}

fn emit_escaped(
    chars: &[char], i: &mut usize, out: &mut String,
    vm: &mut bool, cf: &mut Option<bool>,
) {
    let nc = chars[*i];
    match nc {
        'v' => *vm = true,
        'm' | 'M' | 'V' => *vm = false,
        'c' => *cf = Some(true),
        'C' => *cf = Some(false),
        'd' => out.push_str("[0-9]"),       'D' => out.push_str("[^0-9]"),
        'w' => out.push_str("[0-9A-Za-z_]"), 'W' => out.push_str("[^0-9A-Za-z_]"),
        's' => out.push_str("[ \\t]"),       'S' => out.push_str("[^ \\t]"),
        'a' => out.push_str("[A-Za-z]"),     'A' => out.push_str("[^A-Za-z]"),
        'l' => out.push_str("[a-z]"),        'L' => out.push_str("[^a-z]"),
        'u' => out.push_str("[A-Z]"),        'U' => out.push_str("[^A-Z]"),
        'x' => out.push_str("[0-9A-Fa-f]"), 'X' => out.push_str("[^0-9A-Fa-f]"),
        'o' => out.push_str("[0-7]"),        'O' => out.push_str("[^0-7]"),
        'h' => out.push_str("[A-Za-z_]"),    'H' => out.push_str("[^A-Za-z_]"),
        'n' => out.push('\n'), 'r' => out.push('\r'),
        't' => out.push('\t'), 'e' => out.push('\x1b'),
        '<' | '>' => out.push_str("\\b"),
        '+' if !*vm => out.push('+'),
        '?' | '=' if !*vm => out.push('?'),
        '|' if !*vm => out.push('|'),
        '(' if !*vm => out.push('('),
        ')' if !*vm => out.push(')'),
        '{' if !*vm => emit_brace(chars, i, out),
        c @ '1'..='9' => { out.push('\\'); out.push(c); }
        c => { if is_meta(c) { out.push('\\'); } out.push(c); }
    }
}

fn emit_brace(chars: &[char], i: &mut usize, out: &mut String) {
    if let Some(end) = chars[*i + 1..].iter().position(|&c| c == '}').map(|p| *i + 1 + p) {
        let inner: String = chars[*i + 1..=end].iter().collect();
        out.push('{');
        if inner.starts_with('-') {
            let rest = &inner[1..inner.len() - 1];
            if rest.is_empty() { out.push_str("0,}?"); } else {
                out.push_str(rest); out.push_str("}?");
            }
        } else { out.push_str(&inner); }
        *i = end;
    } else { out.push_str("\\{"); }
}

fn is_meta(c: char) -> bool {
    matches!(c, '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn simple_literal() { assert_eq!(vim_to_rust_regex("hello").unwrap(), "hello"); }
    #[test] fn dot_star() { assert_eq!(vim_to_rust_regex(".*").unwrap(), ".*"); }
    #[test] fn vim_plus() { assert_eq!(vim_to_rust_regex(r"\d\+").unwrap(), "[0-9]+"); }
    #[test] fn vim_group_alt() {
        assert_eq!(vim_to_rust_regex(r"\(foo\|bar\)").unwrap(), "(foo|bar)");
    }
    #[test] fn very_magic_group() {
        assert_eq!(vim_to_rust_regex(r"\v(foo|bar)+").unwrap(), "(foo|bar)+");
    }
    #[test] fn word_boundary() { assert_eq!(vim_to_rust_regex(r"\<word\>").unwrap(), r"\bword\b"); }
    #[test] fn shortcut_atoms() {
        assert_eq!(vim_to_rust_regex(r"\w\s\d").unwrap(), "[0-9A-Za-z_][ \\t][0-9]");
    }
    #[test] fn empty_err() { assert!(vim_to_rust_regex("").is_err()); }
    #[test] fn literal_parens() { assert_eq!(vim_to_rust_regex("f(x)").unwrap(), r"f\(x\)"); }
    #[test] fn anchors() { assert_eq!(vim_to_rust_regex("^start$").unwrap(), "^start$"); }
    #[test] fn octal_atoms() { assert_eq!(vim_to_rust_regex(r"\o\O").unwrap(), "[0-7][^0-7]"); }
    #[test] fn non_head() { assert_eq!(vim_to_rust_regex(r"\H").unwrap(), "[^A-Za-z_]"); }
    #[test] fn case_insensitive() {
        let (p, ci) = vim_to_rust_regex_ex(r"\cfoo").unwrap();
        assert!(p.starts_with("(?i)")); assert_eq!(ci, Some(true));
    }
    #[test] fn case_sensitive() {
        let (p, ci) = vim_to_rust_regex_ex(r"\Cfoo").unwrap();
        assert!(!p.starts_with("(?i)")); assert_eq!(ci, Some(false));
    }
}
