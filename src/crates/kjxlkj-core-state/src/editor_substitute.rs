//! Substitute command implementation: `:s/pat/repl/flags`.

use crate::EditorState;

/// Parsed substitute parts.
struct SubstituteArgs<'a> {
    pattern: &'a str,
    replacement: &'a str,
    global: bool,
    case_insensitive: bool,
}

impl EditorState {
    /// Execute `:s/pat/repl/flags` on the current line.
    pub(crate) fn do_substitute(
        &mut self,
        args: &str,
    ) {
        let parsed = match parse_sub_args(args) {
            Some(p) => p,
            None => return,
        };
        if parsed.pattern.is_empty() {
            return;
        }
        let (line, _) = self.cursor_pos();
        let line_text = match self
            .active_buffer()
            .map(|b| b.content.line_content(line))
        {
            Some(t) => t,
            None => return,
        };
        let new_text = substitute_in_line(
            &line_text,
            &parsed,
        );
        if new_text == line_text {
            return;
        }
        if let Some(buf) = self.active_buffer_mut()
        {
            let start = buf
                .content
                .line_start_offset(line);
            let end_off = start
                + buf
                    .content
                    .line_content(line)
                    .len();
            buf.content
                .delete_range(start, end_off);
            for (i, ch) in
                new_text.chars().enumerate()
            {
                buf.content
                    .insert_char(start + i, ch);
            }
            buf.modified = true;
        }
        self.push_change();
    }

    /// Execute `:s` on a range of lines.
    pub(crate) fn do_substitute_range(
        &mut self, start_line: usize,
        end_line: usize, args: &str,
    ) {
        let parsed = match parse_sub_args(args) {
            Some(p) => p, None => return,
        };
        if parsed.pattern.is_empty() { return; }
        for line in start_line..=end_line {
            let line_text = match self.active_buffer()
                .map(|b| b.content.line_content(line))
            { Some(t) => t, None => continue };
            let new_text = substitute_in_line(&line_text, &parsed);
            if new_text == line_text { continue; }
            if let Some(buf) = self.active_buffer_mut() {
                let s = buf.content.line_start_offset(line);
                let e = s + buf.content.line_content(line).len();
                buf.content.delete_range(s, e);
                for (i, ch) in new_text.chars().enumerate() {
                    buf.content.insert_char(s + i, ch);
                }
                buf.modified = true;
            }
        }
        self.push_change();
    }
}

fn parse_sub_args(args: &str) -> Option<SubstituteArgs<'_>> {
    let args = args.trim();
    if args.is_empty() {
        return None;
    }
    let delim = args.as_bytes()[0] as char;
    let rest = &args[1..];
    let parts: Vec<&str> =
        rest.splitn(3, delim).collect();
    if parts.len() < 2 {
        return None;
    }
    let pattern = parts[0];
    let replacement = parts[1];
    let flags = if parts.len() > 2 {
        parts[2]
    } else {
        ""
    };
    Some(SubstituteArgs {
        pattern,
        replacement,
        global: flags.contains('g'),
        case_insensitive: flags.contains('i'),
    })
}

fn substitute_in_line(
    line: &str,
    args: &SubstituteArgs<'_>,
) -> String {
    if args.case_insensitive {
        let pat_lower = args.pattern.to_lowercase();
        let mut result = String::new();
        let src_lower = line.to_lowercase();
        let mut start = 0;
        loop {
            if let Some(pos) =
                src_lower[start..].find(&pat_lower)
            {
                let abs = start + pos;
                result.push_str(&line[start..abs]);
                result.push_str(args.replacement);
                start = abs + args.pattern.len();
                if !args.global {
                    result.push_str(&line[start..]);
                    break;
                }
            } else {
                result.push_str(&line[start..]);
                break;
            }
        }
        result
    } else if args.global {
        line.replace(args.pattern, args.replacement)
    } else {
        line.replacen(args.pattern, args.replacement, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sub_args_basic() {
        let args = parse_sub_args("/foo/bar/g");
        assert!(args.is_some());
        let a = args.unwrap();
        assert_eq!(a.pattern, "foo");
        assert_eq!(a.replacement, "bar");
        assert!(a.global);
    }

    #[test]
    fn substitute_in_line_basic() {
        let args = SubstituteArgs {
            pattern: "hello",
            replacement: "world",
            global: false,
            case_insensitive: false,
        };
        let result = substitute_in_line(
            "hello hello",
            &args,
        );
        assert_eq!(result, "world hello");
    }

    #[test]
    fn substitute_global() {
        let args = SubstituteArgs {
            pattern: "a",
            replacement: "b",
            global: true,
            case_insensitive: false,
        };
        let result = substitute_in_line(
            "aaa",
            &args,
        );
        assert_eq!(result, "bbb");
    }
}
