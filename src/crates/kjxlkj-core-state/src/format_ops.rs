//! Format (gq) operator: join and re-wrap lines.
//!
//! Basic implementation that joins a range of lines
//! into a single paragraph and wraps at `textwidth`.

use crate::editor::EditorState;

impl EditorState {
    /// Format (re-wrap) a range of lines at textwidth.
    /// If formatprg is set, uses external formatter (stub notification).
    /// Otherwise respects formatoptions: 'q' must be present for gq to work.
    pub(crate) fn format_lines(&mut self, start: usize, end: usize) {
        let fexpr = self.options.get_str("formatexpr").to_string();
        if !fexpr.is_empty() {
            self.handle_call_function(&format!("call {fexpr}"));
            return;
        }
        let fprg = self.options.get_str("formatprg").to_string();
        if !fprg.is_empty() {
            self.format_via_external(&fprg, start, end);
            return;
        }
        let fo = self.options.get_str("formatoptions").to_string();
        if !fo.contains('q') {
            return; // gq formatting disabled
        }
        let tw = self.get_textwidth();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let total = buf.content.len_lines();
            let end = end.min(total.saturating_sub(1));
            if start > end {
                return;
            }
            // Collect lines.
            let mut words: Vec<String> = Vec::new();
            for line_idx in start..=end {
                if line_idx >= buf.content.len_lines() {
                    break;
                }
                let ls = buf.content.line(line_idx);
                let s: std::borrow::Cow<str> = ls.into();
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    // Paragraph break: push empty sentinel.
                    words.push(String::new());
                } else {
                    for w in trimmed.split_whitespace() {
                        words.push(w.to_string());
                    }
                }
            }
            // Rebuild wrapped text.
            let wrapped = wrap_words(&words, tw);
            // Remove old lines and insert wrapped text.
            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end + 1 < buf.content.len_lines() {
                buf.content.line_to_byte(end + 1)
            } else {
                buf.content.len_bytes()
            };
            let sc = buf.content.byte_to_char(start_byte);
            let ec = buf.content.byte_to_char(end_byte);
            buf.content.remove(sc..ec);
            buf.content.insert(sc, &wrapped);
            buf.increment_version();
        }
        self.clamp_cursor();
    }

    fn get_textwidth(&self) -> usize {
        let tw = self.options.get_int("textwidth");
        if tw >= 10 {
            tw
        } else {
            79
        }
    }

    /// Reindent a range of lines based on the line above.
    /// If `equalprg` is set, pipes through external program instead.
    pub(crate) fn reindent_lines(&mut self, start: usize, end: usize) {
        let eprg = self.options.get_str("equalprg").to_string();
        if !eprg.is_empty() {
            self.format_via_external(&eprg, start, end);
            return;
        }
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let sw = self.options.get_int("shiftwidth").max(1);
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let total = buf.content.len_lines();
            let end = end.min(total.saturating_sub(1));
            // Get reference indent from line before start.
            let base_indent = if start > 0 {
                let ls = buf.content.line(start - 1);
                let s: std::borrow::Cow<str> = ls.into();
                s.chars().take_while(|c| *c == ' ').count()
            } else {
                0
            };
            for line_idx in start..=end {
                if line_idx >= buf.content.len_lines() { break; }
                let ls = buf.content.line(line_idx);
                let s: std::borrow::Cow<str> = ls.into();
                let old_indent = s.chars().take_while(|c| *c == ' ').count();
                let trimmed = s.trim_start();
                if trimmed.is_empty() || trimmed == "\n" { continue; }
                // Apply base_indent, keeping relative offset within range.
                let rel = if line_idx == start { 0 } else { sw.min(base_indent) };
                let new_indent = base_indent + if line_idx > start { rel } else { 0 };
                if new_indent != old_indent {
                    let lb = buf.content.line_to_byte(line_idx);
                    let sc = buf.content.byte_to_char(lb);
                    let ec = buf.content.byte_to_char(lb + old_indent);
                    buf.content.remove(sc..ec);
                    let spaces = " ".repeat(new_indent);
                    buf.content.insert(sc, &spaces);
                }
            }
            buf.increment_version();
        }
    }
}

fn wrap_words(words: &[String], width: usize) -> String {
    let mut result = String::new();
    let mut col = 0usize;
    for w in words {
        if w.is_empty() {
            // Paragraph break.
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push('\n');
            col = 0;
            continue;
        }
        if col == 0 {
            result.push_str(w);
            col = w.len();
        } else if col + 1 + w.len() > width {
            result.push('\n');
            result.push_str(w);
            col = w.len();
        } else {
            result.push(' ');
            result.push_str(w);
            col += 1 + w.len();
        }
    }
    if col > 0 {
        result.push('\n');
    }
    result
}

impl EditorState {
    /// Pipe lines through external formatprg command.
    #[rustfmt::skip]
    fn format_via_external(&mut self, prog: &str, start: usize, end: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let input = if let Some(buf) = self.buffers.get(buf_id) {
            let end = end.min(buf.content.len_lines().saturating_sub(1));
            let mut s = String::new();
            for li in start..=end { if li < buf.content.len_lines() { let line: std::borrow::Cow<str> = buf.content.line(li).into(); s.push_str(&line); } }
            s
        } else { return; };
        let parts: Vec<&str> = prog.split_whitespace().collect();
        let (cmd, args) = match parts.split_first() { Some((c, a)) => (*c, a), None => return };
        use std::process::{Command, Stdio};
        let mut child = match Command::new(cmd).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() {
            Ok(c) => c, Err(e) => { self.notify_error(&format!("E282: formatprg: {e}")); return; }
        };
        if let Some(mut stdin) = child.stdin.take() { use std::io::Write; let _ = stdin.write_all(input.as_bytes()); }
        match child.wait_with_output() {
            Ok(out) if out.status.success() => {
                let new_text = String::from_utf8_lossy(&out.stdout);
                if let Some(buf) = self.buffers.get_mut(buf_id) {
                    buf.save_undo_checkpoint(cursor);
                    let total = buf.content.len_lines();
                    let end = end.min(total.saturating_sub(1));
                    let sb = buf.content.line_to_byte(start);
                    let eb = if end + 1 < total { buf.content.line_to_byte(end + 1) } else { buf.content.len_bytes() };
                    let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
                    buf.content.remove(sc..ec); buf.content.insert(sc, &new_text); buf.increment_version();
                }
            }
            Ok(out) => self.notify_error(&format!("formatprg exit {}", out.status)),
            Err(e) => self.notify_error(&format!("formatprg: {e}")),
        }
    }
}
