/// Substitute command execution.
use crate::editor::EditorState;
use crate::ex_parse::ExRange;
use crate::ex_parse_substitute::parse_substitute;
use crate::ex_substitute_confirm::{count_via_regex, substitute_line_regex};
use crate::regex_translate::compile_vim_pattern;

impl EditorState {
    pub(crate) fn execute_substitute(&mut self, input: &str, range: ExRange) {
        let sub_cmd = match parse_substitute(input) {
            Some(cmd) => cmd,
            None => {
                self.notify_error("E486: Invalid substitute command");
                return;
            }
        };
        if sub_cmd.count_only {
            self.substitute_count(&sub_cmd.pattern, range, sub_cmd.case_insensitive);
            return;
        }
        if sub_cmd.confirm {
            let lines: Vec<usize> = (range.start..=range.end).collect();
            if let Some(&first) = lines.first() {
                self.windows.focused_mut().cursor.line = first;
                self.windows.focused_mut().cursor.grapheme = 0;
                self.ensure_cursor_visible();
            }
            self.sub_confirm = Some(crate::editor::SubConfirmState {
                pattern: sub_cmd.pattern.clone(),
                replacement: sub_cmd.replacement.clone(),
                global: sub_cmd.global,
                lines,
                current_line_idx: 0,
            });
            self.notify_info("Replace? (y/n/a/q/l)");
            return;
        }
        let case_sensitive = !sub_cmd.case_insensitive;
        let re = compile_vim_pattern(&sub_cmd.pattern, case_sensitive);
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let mut total_subs = 0;
            let mut total_lines = 0;
            let end = range.end.min(buf.content.len_lines().saturating_sub(1));
            for line_idx in range.start..=end {
                let ls: String = buf.content.line(line_idx).chars().collect();
                let new = substitute_line_regex(
                    &ls, &sub_cmd.pattern, &sub_cmd.replacement,
                    sub_cmd.global, re.as_ref(),
                );
                if new != ls {
                    total_subs += count_via_regex(
                        &ls, &sub_cmd.pattern, sub_cmd.global, re.as_ref(),
                    );
                    total_lines += 1;
                    let sb = buf.content.line_to_byte(line_idx);
                    let eb = if line_idx + 1 < buf.content.len_lines() {
                        buf.content.line_to_byte(line_idx + 1)
                    } else { buf.content.len_bytes() };
                    let sc = buf.content.byte_to_char(sb);
                    let ec = buf.content.byte_to_char(eb);
                    buf.content.remove(sc..ec);
                    buf.content.insert(sc, &new);
                }
            }
            if total_subs > 0 {
                buf.increment_version();
                self.notify_info(&format!(
                    "{total_subs} substitution(s) on {total_lines} line(s)"
                ));
            } else if !sub_cmd.suppress_error {
                self.notify_error(&format!(
                    "E486: Pattern not found: {}", sub_cmd.pattern
                ));
            }
        }
    }

    fn substitute_count(&mut self, pattern: &str, range: ExRange, ci: bool) {
        let re = compile_vim_pattern(pattern, !ci);
        let buf_id = self.current_buffer_id();
        let mut total = 0;
        if let Some(buf) = self.buffers.get(buf_id) {
            let end = range.end.min(buf.content.len_lines().saturating_sub(1));
            for l in range.start..=end {
                let s: String = buf.content.line(l).chars().collect();
                total += count_via_regex(&s, pattern, true, re.as_ref());
            }
        }
        self.notify_info(&format!("{total} match(es) found"));
    }
}
