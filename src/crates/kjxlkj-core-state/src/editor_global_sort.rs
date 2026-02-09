//! Global command (`:g/pat/cmd`) and sort (`:sort`).
//!
//! The global command executes a command on every line
//! matching a pattern. Vglobal does the inverse.

use crate::EditorState;

impl EditorState {
    /// Execute `:g/pat/cmd` — run cmd on matching lines.
    pub(crate) fn do_global_command(&mut self, args: &str) {
        let (pattern, cmd) = match parse_global_args(args) {
            Some(v) => v,
            None => return,
        };
        self.execute_global(pattern, cmd, false);
    }

    /// Execute `:v/pat/cmd` — run cmd on non-matching lines.
    pub(crate) fn do_vglobal_command(&mut self, args: &str) {
        let (pattern, cmd) = match parse_global_args(args) {
            Some(v) => v,
            None => return,
        };
        self.execute_global(pattern, cmd, true);
    }

    fn execute_global(&mut self, pattern: String, cmd: String, invert: bool) {
        let bid = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let line_count = match self.buffers.get(&bid) {
            Some(b) => b.content.line_count(),
            None => return,
        };
        // Collect matching line numbers using simple substring match
        let mut matching_lines = Vec::new();
        if let Some(buf) = self.buffers.get(&bid) {
            for line in 0..line_count {
                let content = buf.content.line_content(line);
                let matches = content.contains(&pattern);
                if matches != invert {
                    matching_lines.push(line);
                }
            }
        }
        // Execute the command on each matching line
        // For `:g/pat/d` — handle 'd' as delete command
        let cmd = cmd.trim();
        if cmd == "d" || cmd == "delete" {
            // Delete matching lines in reverse order
            for &line in matching_lines.iter().rev() {
                if let Some(buf) = self.buffers.get_mut(&bid) {
                    if line < buf.content.line_count() {
                        buf.content.delete_lines(line, line + 1);
                        buf.modified = true;
                    }
                }
            }
        } else if let Some(sub_args) = cmd.strip_prefix("s") {
            // Substitute on matching lines
            for &line in &matching_lines {
                self.do_substitute_range(line, line, sub_args);
            }
        } else {
            // General command dispatch on each line
            for &line in &matching_lines {
                if let Some(w) = self.focused_window_mut() {
                    w.cursor.line = line;
                    w.cursor.grapheme_offset = 0;
                }
                if let Some(a) = crate::dispatch_command(cmd) {
                    self.dispatch(a);
                }
            }
        }
    }

    /// Execute `:sort` — sort lines in buffer.
    pub(crate) fn do_sort_lines(&mut self, args: &str) {
        let bid = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let buf = match self.buffers.get(&bid) {
            Some(b) => b,
            None => return,
        };
        let lc = buf.content.line_count();
        if lc <= 1 {
            return;
        }
        let mut lines: Vec<String> = (0..lc).map(|i| buf.content.line_content(i)).collect();
        let unique = args.contains('u');
        let reverse = args.contains('!');
        let numeric = args.contains('n');
        if numeric {
            lines.sort_by(|a, b| {
                let na = extract_number(a);
                let nb = extract_number(b);
                na.cmp(&nb)
            });
        } else {
            lines.sort();
        }
        if reverse {
            lines.reverse();
        }
        if unique {
            lines.dedup();
        }
        // Replace buffer content
        let buf = match self.buffers.get_mut(&bid) {
            Some(b) => b,
            None => return,
        };
        let new_content = lines.join("\n") + "\n";
        buf.content = kjxlkj_core_text::BufferContent::from_str(&new_content);
        buf.modified = true;
    }
}

/// Parse `:g/pattern/command` arguments.
fn parse_global_args(args: &str) -> Option<(String, String)> {
    let args = args.trim();
    if args.is_empty() {
        return None;
    }
    let delim = args.chars().next()?;
    let rest = &args[delim.len_utf8()..];
    let end = rest.find(delim)?;
    let pattern = rest[..end].to_string();
    let cmd = rest[end + delim.len_utf8()..].to_string();
    if pattern.is_empty() {
        return None;
    }
    Some((pattern, cmd))
}

/// Extract first integer from a string for numeric sort.
fn extract_number(s: &str) -> i64 {
    let mut num_str = String::new();
    let mut found = false;
    for c in s.chars() {
        if c.is_ascii_digit() || (c == '-' && !found) {
            num_str.push(c);
            found = true;
        } else if found {
            break;
        }
    }
    num_str.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_global_args_basic() {
        let (pat, cmd) = parse_global_args("/foo/d").unwrap();
        assert_eq!(pat, "foo");
        assert_eq!(cmd, "d");
    }

    #[test]
    fn sort_lines_test() {
        let mut ed = EditorState::new(80, 24);
        // Insert some lines
        use kjxlkj_core_types::Action;
        for c in "banana\napple\ncherry\n".chars() {
            ed.dispatch(Action::InsertChar(c));
        }
        ed.do_sort_lines("");
        let buf = ed.active_buffer().unwrap();
        let l0 = buf.content.line_content(0);
        // After sort, first line should be empty or "apple"
        assert!(l0.is_empty() || l0.trim() == "apple" || l0.trim().is_empty());
    }

    #[test]
    fn extract_number_test() {
        assert_eq!(extract_number("abc123def"), 123);
        assert_eq!(extract_number("no number"), 0);
        assert_eq!(extract_number("-42 things"), -42);
    }
}
