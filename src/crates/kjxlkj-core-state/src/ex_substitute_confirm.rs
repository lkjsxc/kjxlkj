/// Substitute confirmation (:s///c) handler.
use crate::editor::EditorState;
use crate::regex_translate::compile_vim_pattern;

/// Substitute using regex if available, else plain text.
pub(crate) fn substitute_line_regex(
    line: &str,
    pattern: &str,
    replacement: &str,
    global: bool,
    re: Option<&regex::Regex>,
) -> String {
    if let Some(re) = re {
        if global {
            re.replace_all(line, replacement).into_owned()
        } else {
            re.replace(line, replacement).into_owned()
        }
    } else if global {
        line.replace(pattern, replacement)
    } else {
        line.replacen(pattern, replacement, 1)
    }
}

/// Count matches using regex if available.
pub(crate) fn count_via_regex(
    line: &str,
    pattern: &str,
    global: bool,
    re: Option<&regex::Regex>,
) -> usize {
    if let Some(re) = re {
        if global { re.find_iter(line).count() }
        else if re.is_match(line) { 1 } else { 0 }
    } else if global {
        line.matches(pattern).count()
    } else if line.contains(pattern) { 1 } else { 0 }
}

impl EditorState {
    /// Handle a key press during :s///c confirmation.
    pub(crate) fn handle_sub_confirm_key(&mut self, c: char) {
        let state = match self.sub_confirm.take() {
            Some(s) => s,
            None => return,
        };
        match c {
            'y' => {
                self.sub_confirm_do_one(&state);
                self.sub_confirm_advance(state);
            }
            'n' => self.sub_confirm_advance(state),
            'a' => self.sub_confirm_do_remaining(&state),
            'l' => self.sub_confirm_do_one(&state),
            _ => { /* 'q' or anything else: quit */ }
        }
    }

    fn sub_confirm_do_one(&mut self, state: &crate::editor::SubConfirmState) {
        let re = compile_vim_pattern(&state.pattern, true);
        if let Some(&line_idx) = state.lines.get(state.current_line_idx) {
            let buf_id = self.current_buffer_id();
            let cursor = self.windows.focused().cursor;
            if let Some(buf) = self.buffers.get_mut(buf_id) {
                buf.save_undo_checkpoint(cursor);
                if line_idx < buf.content.len_lines() {
                    let ls: String = buf.content.line(line_idx).chars().collect();
                    let new = substitute_line_regex(
                        &ls, &state.pattern, &state.replacement,
                        state.global, re.as_ref(),
                    );
                    if new != ls {
                        let sb = buf.content.line_to_byte(line_idx);
                        let eb = if line_idx + 1 < buf.content.len_lines() {
                            buf.content.line_to_byte(line_idx + 1)
                        } else { buf.content.len_bytes() };
                        let sc = buf.content.byte_to_char(sb);
                        let ec = buf.content.byte_to_char(eb);
                        buf.content.remove(sc..ec);
                        buf.content.insert(sc, &new);
                        buf.increment_version();
                    }
                }
            }
        }
    }

    fn sub_confirm_do_remaining(&mut self, state: &crate::editor::SubConfirmState) {
        let re = compile_vim_pattern(&state.pattern, true);
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            for &li in &state.lines[state.current_line_idx..] {
                if li >= buf.content.len_lines() { continue; }
                let ls: String = buf.content.line(li).chars().collect();
                let new = substitute_line_regex(
                    &ls, &state.pattern, &state.replacement,
                    state.global, re.as_ref(),
                );
                if new != ls {
                    let sb = buf.content.line_to_byte(li);
                    let eb = if li + 1 < buf.content.len_lines() {
                        buf.content.line_to_byte(li + 1)
                    } else { buf.content.len_bytes() };
                    let sc = buf.content.byte_to_char(sb);
                    let ec = buf.content.byte_to_char(eb);
                    buf.content.remove(sc..ec);
                    buf.content.insert(sc, &new);
                    buf.increment_version();
                }
            }
        }
    }

    fn sub_confirm_advance(&mut self, mut state: crate::editor::SubConfirmState) {
        state.current_line_idx += 1;
        if state.current_line_idx < state.lines.len() {
            let target_line = state.lines[state.current_line_idx];
            self.windows.focused_mut().cursor.line = target_line;
            self.windows.focused_mut().cursor.grapheme = 0;
            self.ensure_cursor_visible();
            self.sub_confirm = Some(state);
            self.notify_info("Replace? (y/n/a/q/l)");
        }
    }
}
