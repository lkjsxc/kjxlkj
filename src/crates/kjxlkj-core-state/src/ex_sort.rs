/// Sort command implementation.
use crate::editor::EditorState;
use crate::ex_parse::ExRange;

impl EditorState {
    /// Handle `:sort` command with optional flags.
    /// Flags: `!` reverse, `i` case-insensitive, `n` numeric, `u` unique.
    #[rustfmt::skip]
    pub(crate) fn handle_sort(&mut self, flags: &str, range: ExRange) {
        let reverse = flags.contains('!') || flags.contains('r');
        let case_insensitive = flags.contains('i');
        let numeric = flags.contains('n');
        let unique = flags.contains('u');
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let total = buf.content.len_lines();
            let end = range.end.min(total.saturating_sub(1));
            if range.start > end { return; }
            let mut lines: Vec<String> = (range.start..=end)
                .map(|i| { let l: std::borrow::Cow<str> = buf.content.line(i).into(); l.to_string() })
                .collect();
            if numeric {
                lines.sort_by(|a, b| {
                    let na = extract_num(a);
                    let nb = extract_num(b);
                    na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
                });
            } else if case_insensitive {
                lines.sort_by_key(|a| a.to_lowercase());
            } else {
                lines.sort();
            }
            if reverse { lines.reverse(); }
            if unique { lines.dedup(); }
            let sb = buf.content.line_to_byte(range.start);
            let eb = if end + 1 < total { buf.content.line_to_byte(end + 1) } else { buf.content.len_bytes() };
            let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
            buf.content.remove(sc..ec);
            let joined = lines.join("");
            buf.content.insert(sc, &joined);
            buf.increment_version();
            self.notify_info(&format!("{} lines sorted", lines.len()));
        }
    }

    /// Handle `:{range}!{cmd}` — filter lines through external shell command.
    #[rustfmt::skip]
    pub(crate) fn handle_filter_shell(&mut self, cmd_str: &str, range: ExRange) {
        if cmd_str.is_empty() { return self.notify_error("E471: Argument required"); }
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let total = buf.content.len_lines();
            let end = range.end.min(total.saturating_sub(1));
            if range.start > end { return; }
            let input: String = (range.start..=end)
                .map(|i| { let l: std::borrow::Cow<str> = buf.content.line(i).into(); l.to_string() })
                .collect();
            use std::io::Write;
            use std::process::{Command, Stdio};
            let result = (|| -> Result<String, String> {
                let mut child = Command::new("sh").arg("-c").arg(cmd_str).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
                    .spawn().map_err(|e| format!("E282: {e}"))?;
                if let Some(ref mut stdin) = child.stdin { let _ = stdin.write_all(input.as_bytes()); }
                drop(child.stdin.take());
                let out = child.wait_with_output().map_err(|e| format!("E282: {e}"))?;
                Ok(String::from_utf8_lossy(&out.stdout).to_string())
            })();
            match result {
                Ok(output) => {
                    let sb = buf.content.line_to_byte(range.start);
                    let eb = if end + 1 < total { buf.content.line_to_byte(end + 1) } else { buf.content.len_bytes() };
                    let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
                    buf.content.remove(sc..ec);
                    buf.content.insert(sc, &output);
                    buf.increment_version();
                    self.notify_info(&format!("{} lines filtered", end - range.start + 1));
                }
                Err(e) => self.notify_error(&e),
            }
        }
    }
}

/// Extract leading numeric value from a string for numeric sort.
fn extract_num(s: &str) -> f64 {
    let s = s.trim();
    let end = s.find(|c: char| !c.is_ascii_digit() && c != '-' && c != '.').unwrap_or(s.len());
    s[..end].parse().unwrap_or(0.0)
}
