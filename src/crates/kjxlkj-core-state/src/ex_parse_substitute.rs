//! Substitute command parser.

/// Parsed substitution command.
#[derive(Debug)]
pub struct SubstituteCmd {
    pub pattern: String,
    pub replacement: String,
    pub global: bool,
    pub case_insensitive: bool,
    pub count_only: bool,
    pub suppress_error: bool,
    pub confirm: bool,
}

/// Parse a substitution command: s/pattern/replacement/flags
pub fn parse_substitute(input: &str) -> Option<SubstituteCmd> {
    let input = input.trim_start();
    if input.is_empty() {
        return None;
    }

    let delim = input.chars().next()?;
    if delim.is_alphanumeric() {
        return None;
    }

    let rest = &input[delim.len_utf8()..];

    // Find pattern
    let pat_end = find_unescaped(rest, delim)?;
    let pattern = rest[..pat_end].to_string();
    let rest = &rest[pat_end + delim.len_utf8()..];

    // Find replacement
    let repl_end = find_unescaped(rest, delim).unwrap_or(rest.len());
    let replacement = rest[..repl_end].to_string();
    let rest = if repl_end < rest.len() {
        &rest[repl_end + delim.len_utf8()..]
    } else {
        ""
    };

    // Parse flags
    let mut global = false;
    let mut case_insensitive = false;
    let mut count_only = false;
    let mut suppress_error = false;
    let mut confirm = false;

    for c in rest.chars() {
        match c {
            'g' => global = true,
            'i' => case_insensitive = true,
            'I' => case_insensitive = false,
            'n' => count_only = true,
            'e' => suppress_error = true,
            'c' => confirm = true,
            _ => break,
        }
    }

    Some(SubstituteCmd {
        pattern,
        replacement,
        global,
        case_insensitive,
        count_only,
        suppress_error,
        confirm,
    })
}

/// Find the position of an unescaped delimiter character.
fn find_unescaped(s: &str, delim: char) -> Option<usize> {
    let mut escaped = false;
    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == delim {
            return Some(i);
        }
    }
    None
}
