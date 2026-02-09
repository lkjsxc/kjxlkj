/// Ex buffer commands — write, navigation, window management.
use kjxlkj_core_types::{Action, ContentSource};

use crate::editor::EditorState;
use crate::ex_parse::ExRange;

impl EditorState {
    pub(crate) fn write_current_buffer(&mut self) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            if let Some(path) = &buf.path {
                let content = buf.content.to_string();
                let path = path.clone();
                if let Ok(()) = std::fs::write(&path, &content) {
                    buf.mark_saved();
                    self.notify_info(&format!("Written: {}", path.display()));
                } else {
                    self.notify_error(&format!("E212: Failed to write: {}", path.display()));
                }
            } else {
                self.notify_error("E32: No file name");
            }
        }
    }

    pub(crate) fn next_buffer(&mut self) {
        let bid = self.current_buffer_id();
        let c = self.windows.focused().cursor;
        self.marks.set_alternate(crate::marks::MarkPosition::new(bid.0 as usize, c.line, c.grapheme));
        self.alternate_buffer = Some(bid);
        self.buffers.next();
        let buf_id = self.buffers.current_id();
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }

    pub(crate) fn prev_buffer(&mut self) {
        let bid = self.current_buffer_id();
        let c = self.windows.focused().cursor;
        self.marks.set_alternate(crate::marks::MarkPosition::new(bid.0 as usize, c.line, c.grapheme));
        self.alternate_buffer = Some(bid);
        self.buffers.prev();
        let buf_id = self.buffers.current_id();
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }

    pub(crate) fn split_horizontal(&mut self) {
        let buf_id = self.current_buffer_id();
        self.windows.split_horizontal(buf_id);
    }

    pub(crate) fn split_vertical(&mut self) {
        let buf_id = self.current_buffer_id();
        self.windows.split_vertical(buf_id);
    }

    pub(crate) fn close_window(&mut self) {
        if !self.windows.close_focused() {
            self.handle_action(Action::Quit);
        }
    }

    pub(crate) fn has_unsaved_buffers(&self) -> bool {
        self.buffers.iter().any(|b| b.modified)
    }

    pub(crate) fn delete_range(&mut self, range: ExRange) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;

        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let max_line = buf.content.len_lines().saturating_sub(1);
            let start = range.start.min(max_line);
            let end = (range.end + 1).min(buf.content.len_lines());

            buf.save_undo_checkpoint(cursor);

            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end)
            };

            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
                buf.content.remove(start_char..end_char);
                buf.increment_version();
            }
        }

        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub(crate) fn yank_range(&mut self, range: ExRange) {
        let buf_id = self.current_buffer_id();

        if let Some(buf) = self.buffers.get(buf_id) {
            let max_line = buf.content.len_lines().saturating_sub(1);
            let start = range.start.min(max_line);
            let end = (range.end + 1).min(buf.content.len_lines());

            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end)
            };

            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
                let count = range.line_count();
                self.notify_info(&format!("{count} line(s) yanked"));
            }
        }
    }

    /// Handle `:retab[!] [new_tabstop]` — convert between tabs and spaces.
    /// Without `!`, only replaces sequences of tabs. With `!`, replaces all leading whitespace.
    #[rustfmt::skip]
    pub(crate) fn handle_retab(&mut self, args: &str) {
        let bang = args.starts_with('!');
        let rest = if bang { args[1..].trim() } else { args.trim() };
        let new_ts: usize = if rest.is_empty() { self.options.get_int("tabstop").max(1) } else { rest.parse().unwrap_or(8).max(1) };
        let expand = self.options.get_bool("expandtab");
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let text = buf.content.to_string();
            let mut result = String::with_capacity(text.len());
            let mut changed = 0usize;
            for line in text.split('\n') {
                let leading_len = line.len() - line.trim_start().len();
                let leading = &line[..leading_len];
                let rest_line = &line[leading_len..];
                // Calculate visual column width of leading whitespace.
                let old_ts = self.options.get_int("tabstop").max(1);
                let mut vcol = 0usize;
                for ch in leading.chars() {
                    if ch == '\t' { vcol = (vcol / old_ts + 1) * old_ts; } else { vcol += 1; }
                }
                let new_leading = if expand {
                    " ".repeat(vcol)
                } else {
                    let tabs = vcol / new_ts;
                    let spaces = vcol % new_ts;
                    "\t".repeat(tabs) + &" ".repeat(spaces)
                };
                if new_leading != leading { changed += 1; }
                result.push_str(&new_leading);
                result.push_str(rest_line);
                result.push('\n');
            }
            // Remove trailing newline added by split loop.
            if !text.ends_with('\n') && result.ends_with('\n') { result.pop(); }
            if changed > 0 {
                let len = buf.content.len_chars();
                buf.content.remove(0..len);
                buf.content.insert(0, &result);
                buf.increment_version();
            }
            self.notify_info(&format!(":retab — {changed} line(s) changed (tabstop={new_ts})"));
        }
    }
}
