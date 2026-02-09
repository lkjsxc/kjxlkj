/// Ex buffer commands — write, navigation, window management.
use kjxlkj_core_types::{Action, ContentSource};

use crate::editor::EditorState;
use crate::ex_parse::ExRange;

#[rustfmt::skip]
impl EditorState {
    pub(crate) fn write_current_buffer(&mut self) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            if let Some(path) = &buf.path { let (content, path) = (buf.content.to_string(), path.clone());
                if let Ok(()) = std::fs::write(&path, &content) { buf.mark_saved(); self.notify_info(&format!("Written: {}", path.display())); }
                else { self.notify_error(&format!("E212: Failed to write: {}", path.display())); }
            } else { self.notify_error("E32: No file name"); }
        }
    }
    fn switch_buffer_common(&mut self) { let bid = self.current_buffer_id(); let c = self.windows.focused().cursor; self.marks.set_alternate(crate::marks::MarkPosition::new(bid.0 as usize, c.line, c.grapheme)); self.alternate_buffer = Some(bid); }
    pub(crate) fn next_buffer(&mut self) { self.switch_buffer_common(); self.buffers.next(); let buf_id = self.buffers.current_id(); self.windows.focused_mut().content = ContentSource::Buffer(buf_id); }
    pub(crate) fn prev_buffer(&mut self) { self.switch_buffer_common(); self.buffers.prev(); let buf_id = self.buffers.current_id(); self.windows.focused_mut().content = ContentSource::Buffer(buf_id); }
    pub(crate) fn split_horizontal(&mut self) { let buf_id = self.current_buffer_id(); self.windows.split_horizontal(buf_id); }
    pub(crate) fn split_vertical(&mut self) { let buf_id = self.current_buffer_id(); self.windows.split_vertical(buf_id); }
    pub(crate) fn close_window(&mut self) { if !self.windows.close_focused() { self.handle_action(Action::Quit); } }
    pub(crate) fn has_unsaved_buffers(&self) -> bool { self.buffers.iter().any(|b| b.modified) }

    pub(crate) fn delete_range(&mut self, range: ExRange) {
        let (buf_id, cursor) = (self.current_buffer_id(), self.windows.focused().cursor);
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let (start, end) = (range.start.min(buf.content.len_lines().saturating_sub(1)), (range.end + 1).min(buf.content.len_lines()));
            buf.save_undo_checkpoint(cursor);
            let (sb, eb) = (buf.content.line_to_byte(start), if end >= buf.content.len_lines() { buf.content.len_bytes() } else { buf.content.line_to_byte(end) });
            if sb < eb {
                let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
                self.registers.set_unnamed(buf.content.slice(sc..ec).to_string(), true);
                buf.content.remove(sc..ec); buf.increment_version();
            }
        }
        self.clamp_cursor(); self.ensure_cursor_visible();
    }
    pub(crate) fn yank_range(&mut self, range: ExRange) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let (start, end) = (range.start.min(buf.content.len_lines().saturating_sub(1)), (range.end + 1).min(buf.content.len_lines()));
            let (sb, eb) = (buf.content.line_to_byte(start), if end >= buf.content.len_lines() { buf.content.len_bytes() } else { buf.content.line_to_byte(end) });
            if sb < eb {
                let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
                self.registers.set_unnamed(buf.content.slice(sc..ec).to_string(), true);
                self.notify_info(&format!("{} line(s) yanked", range.line_count()));
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

    /// Handle `:center [width]`, `:left [indent]`, `:right [width]` — text alignment.
    #[rustfmt::skip]
    pub(crate) fn handle_alignment(&mut self, cmd: &str, range: crate::ex_parse::ExRange) {
        let (kind, args) = if let Some(a) = cmd.strip_prefix("center") { ("center", a.trim()) }
            else if let Some(a) = cmd.strip_prefix("left") { ("left", a.trim()) }
            else if let Some(a) = cmd.strip_prefix("right") { ("right", a.trim()) }
            else { return; };
        let width: usize = if args.is_empty() { self.options.get_int("textwidth").max(1) } else { args.parse().unwrap_or(79).max(1) };
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let text = buf.content.to_string();
            let lines: Vec<&str> = text.lines().collect();
            let mut result = String::with_capacity(text.len());
            let mut changed = 0usize;
            for (i, line) in lines.iter().enumerate() {
                if i >= range.start && i <= range.end {
                    let trimmed = line.trim();
                    let new_line = match kind {
                        "center" => { let pad = width.saturating_sub(trimmed.len()) / 2; format!("{}{}", " ".repeat(pad), trimmed) }
                        "right" => { let pad = width.saturating_sub(trimmed.len()); format!("{}{}", " ".repeat(pad), trimmed) }
                        _ /* left */ => { let indent: usize = if args.is_empty() { 0 } else { args.parse().unwrap_or(0) }; format!("{}{}", " ".repeat(indent), trimmed) }
                    };
                    if *line != new_line { changed += 1; }
                    result.push_str(&new_line);
                } else { result.push_str(line); }
                if i + 1 < lines.len() || text.ends_with('\n') { result.push('\n'); }
            }
            if changed > 0 {
                let len = buf.content.len_chars();
                buf.content.remove(0..len);
                buf.content.insert(0, &result);
                buf.increment_version();
            }
            self.notify_info(&format!(":{kind} — {changed} line(s) aligned"));
        }
    }

    /// Handle `:move {dest}` — move range of lines to after dest line.
    #[rustfmt::skip]
    pub(crate) fn handle_move_range(&mut self, range: crate::ex_parse::ExRange, dest: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let total = buf.content.len_lines();
            let (start, end) = (range.start.min(total.saturating_sub(1)), range.end.min(total.saturating_sub(1)));
            buf.save_undo_checkpoint(cursor);
            let mut lines = Vec::new();
            for l in start..=end { lines.push(buf.content.line(l).to_string()); }
            let text: String = lines.join("");
            // Delete source lines.
            let (sb, eb) = (buf.content.line_to_byte(start), if end + 1 >= total { buf.content.len_bytes() } else { buf.content.line_to_byte(end + 1) });
            let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
            buf.content.remove(sc..ec);
            // Insert at destination (adjusted for removal).
            let adj_dest = if dest > end { dest - (end - start + 1) } else { dest };
            let ins_line = (adj_dest + 1).min(buf.content.len_lines());
            let ins_byte = if ins_line >= buf.content.len_lines() { buf.content.len_bytes() } else { buf.content.line_to_byte(ins_line) };
            let ins_char = buf.content.byte_to_char(ins_byte);
            let ins_text = if ins_line >= buf.content.len_lines() && !text.ends_with('\n') { format!("\n{}", text) } else { text };
            buf.content.insert(ins_char, &ins_text);
            buf.increment_version();
            self.notify_info(&format!("{} line(s) moved", end - start + 1));
        }
        self.clamp_cursor(); self.ensure_cursor_visible();
    }

    /// Handle `:copy {dest}` / `:t {dest}` — copy range of lines to after dest line.
    #[rustfmt::skip]
    pub(crate) fn handle_copy_range(&mut self, range: crate::ex_parse::ExRange, dest: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let total = buf.content.len_lines();
            let (start, end) = (range.start.min(total.saturating_sub(1)), range.end.min(total.saturating_sub(1)));
            buf.save_undo_checkpoint(cursor);
            let mut lines = Vec::new();
            for l in start..=end { lines.push(buf.content.line(l).to_string()); }
            let text: String = lines.join("");
            let ins_line = (dest + 1).min(total);
            let ins_byte = if ins_line >= total { buf.content.len_bytes() } else { buf.content.line_to_byte(ins_line) };
            let ins_char = buf.content.byte_to_char(ins_byte);
            let ins_text = if ins_line >= total && !text.ends_with('\n') { format!("\n{}", text) } else { text };
            buf.content.insert(ins_char, &ins_text);
            buf.increment_version();
            self.notify_info(&format!("{} line(s) copied", end - start + 1));
        }
    }
}
