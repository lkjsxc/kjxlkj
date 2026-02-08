//! Range command execution: :delete, :yank, :copy, :move,
//! :normal, and range parsing utilities.

use crate::EditorState;

impl EditorState {
    /// Execute `:delete` on a line range.
    pub(crate) fn do_range_delete(
        &mut self,
        start: usize,
        end: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let lc = buf.line_count();
            let s = start.min(lc.saturating_sub(1));
            let e = end.min(lc);
            if s >= e {
                return;
            }
            let so = buf.content.line_start_offset(s);
            let eo = if e >= lc {
                buf.content.len_chars()
            } else {
                buf.content.line_start_offset(e)
            };
            buf.content.delete_range(so, eo);
            buf.modified = true;
        }
        let max = self
            .active_buffer()
            .map(|b| b.line_count().saturating_sub(1))
            .unwrap_or(0);
        if let Some(w) = self.focused_window_mut() {
            if w.cursor.line > max {
                w.cursor.line = max;
            }
            w.cursor.grapheme_offset = 0;
        }
    }

    /// Execute `:yank` on a line range (stores in unnamed).
    pub(crate) fn do_range_yank(
        &mut self,
        start: usize,
        end: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        if let Some(buf) = self.buffers.get(&bid) {
            let lc = buf.line_count();
            let s = start.min(lc.saturating_sub(1));
            let e = end.min(lc);
            let mut text = String::new();
            for l in s..e {
                text.push_str(&buf.content.line_str(l));
            }
            self.register_file.store(
                kjxlkj_core_types::RegisterName::Unnamed,
                text,
                true,
            );
        }
    }

    /// Execute `:copy` (`:t`): copy lines to a destination.
    pub(crate) fn do_range_copy(
        &mut self,
        start: usize,
        end: usize,
        dest: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        if let Some(buf) = self.buffers.get(&bid) {
            let lc = buf.line_count();
            let s = start.min(lc.saturating_sub(1));
            let e = end.min(lc);
            let mut text = String::new();
            for l in s..e {
                text.push_str(&buf.content.line_str(l));
            }
            drop(buf);
            if let Some(buf) =
                self.buffers.get_mut(&bid)
            {
                let d = dest.min(buf.line_count());
                let off = if d >= buf.line_count() {
                    buf.content.len_chars()
                } else {
                    buf.content.line_start_offset(d)
                };
                for (i, ch) in
                    text.chars().enumerate()
                {
                    buf.content.insert_char(off + i, ch);
                }
                buf.modified = true;
            }
        }
    }

    /// Execute `:move` (`:m`): move lines to a destination.
    pub(crate) fn do_range_move(
        &mut self,
        start: usize,
        end: usize,
        dest: usize,
    ) {
        // Copy first, then delete source lines.
        self.do_range_copy(start, end, dest);
        // After copy, the source shifted if dest < start.
        let offset = end - start;
        let (ds, de) = if dest <= start {
            (start + offset, end + offset)
        } else {
            (start, end)
        };
        self.do_range_delete(ds, de);
    }

    /// Execute `:normal` on a range of lines.
    pub(crate) fn do_range_normal(
        &mut self,
        start: usize,
        end: usize,
        keys_str: &str,
    ) {
        let keys = parse_key_string(keys_str);
        let lc = self
            .active_buffer()
            .map(|b| b.line_count())
            .unwrap_or(0);
        let s = start.min(lc.saturating_sub(1));
        let e = end.min(lc);
        for line in s..e {
            if let Some(w) = self.focused_window_mut() {
                w.cursor.line = line;
                w.cursor.grapheme_offset = 0;
            }
            for key in &keys {
                self.dispatch_key(key.clone());
            }
        }
    }
}

/// Parse a simple key string to Key events.
fn parse_key_string(s: &str) -> Vec<kjxlkj_core_types::Key> {
    let mut keys = Vec::new();
    for ch in s.chars() {
        keys.push(kjxlkj_core_types::Key::char(ch));
    }
    keys
}

/// Parse a range specification from a command string.
///
/// Returns (start_line, end_line, remaining_args).
/// Lines are 0-indexed. Range is exclusive of end.
pub(crate) fn parse_range(
    args: &str,
    current_line: usize,
    last_line: usize,
) -> (usize, usize, String) {
    let args = args.trim();
    if args.is_empty() {
        return (
            current_line,
            current_line + 1,
            String::new(),
        );
    }
    // Check for % (whole file)
    if args.starts_with('%') {
        let rest = args[1..].trim().to_string();
        return (0, last_line + 1, rest);
    }
    // Try to parse n,m range
    if let Some(comma) = args.find(',') {
        let a = &args[..comma];
        let rest = &args[comma + 1..];
        let s = parse_line_spec(a, current_line, last_line);
        // Find end of second line spec
        let end_str: String = rest
            .chars()
            .take_while(|c| {
                c.is_ascii_digit()
                    || *c == '$'
                    || *c == '.'
            })
            .collect();
        let e = parse_line_spec(
            &end_str,
            current_line,
            last_line,
        );
        let remaining = rest[end_str.len()..].trim().to_string();
        (s, e + 1, remaining)
    } else {
        // Single line or address
        let line_str: String = args
            .chars()
            .take_while(|c| {
                c.is_ascii_digit()
                    || *c == '$'
                    || *c == '.'
            })
            .collect();
        if line_str.is_empty() {
            return (
                current_line,
                current_line + 1,
                args.to_string(),
            );
        }
        let l = parse_line_spec(
            &line_str,
            current_line,
            last_line,
        );
        let remaining =
            args[line_str.len()..].trim().to_string();
        (l, l + 1, remaining)
    }
}

fn parse_line_spec(
    s: &str,
    current: usize,
    last: usize,
) -> usize {
    let s = s.trim();
    match s {
        "." => current,
        "$" => last,
        _ => {
            s.parse::<usize>()
                .map(|n| n.saturating_sub(1))
                .unwrap_or(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_range_percent() {
        let (s, e, rest) = parse_range("% d", 5, 20);
        assert_eq!(s, 0);
        assert_eq!(e, 21);
        assert_eq!(rest, "d");
    }

    #[test]
    fn parse_range_explicit() {
        let (s, e, rest) = parse_range("3,7", 0, 20);
        assert_eq!(s, 2);
        assert_eq!(e, 7);
        assert!(rest.is_empty());
    }

    #[test]
    fn range_delete() {
        let mut ed = EditorState::new(80, 24);
        ed.mode = kjxlkj_core_types::Mode::Insert;
        ed.insert_char('a');
        ed.insert_char('\n');
        ed.insert_char('b');
        ed.insert_char('\n');
        ed.insert_char('c');
        ed.mode = kjxlkj_core_types::Mode::Normal;
        let initial = ed
            .active_buffer()
            .unwrap()
            .line_count();
        ed.do_range_delete(0, 1);
        let after = ed
            .active_buffer()
            .unwrap()
            .line_count();
        assert!(after < initial);
    }
}
