/// Substitute command execution and helpers.
use crate::editor::EditorState;
use crate::ex_parse::ExRange;
use crate::ex_parse_substitute::parse_substitute;
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

        let case_sensitive = !sub_cmd.case_insensitive;
        let re = compile_vim_pattern(&sub_cmd.pattern, case_sensitive);
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;

        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let mut total_replacements = 0;
            let mut total_lines_changed = 0;

            for line_idx in range.start..=range.end.min(buf.content.len_lines().saturating_sub(1)) {
                let line_slice = buf.content.line(line_idx);
                let line_str: String = line_slice.chars().collect();

                let new_line = substitute_line_regex(
                    &line_str,
                    &sub_cmd.pattern,
                    &sub_cmd.replacement,
                    sub_cmd.global,
                    re.as_ref(),
                );

                if new_line != line_str {
                    let count =
                        count_via_regex(&line_str, &sub_cmd.pattern, sub_cmd.global, re.as_ref());
                    total_replacements += count;
                    total_lines_changed += 1;
                    let start_byte = buf.content.line_to_byte(line_idx);
                    let end_byte = if line_idx + 1 < buf.content.len_lines() {
                        buf.content.line_to_byte(line_idx + 1)
                    } else {
                        buf.content.len_bytes()
                    };
                    let start_char = buf.content.byte_to_char(start_byte);
                    let end_char = buf.content.byte_to_char(end_byte);
                    buf.content.remove(start_char..end_char);
                    buf.content.insert(start_char, &new_line);
                }
            }

            if total_replacements > 0 {
                buf.increment_version();
                self.notify_info(&format!(
                    "{total_replacements} substitution(s) on \
                     {total_lines_changed} line(s)"
                ));
            } else if !sub_cmd.suppress_error {
                self.notify_error(&format!("E486: Pattern not found: {}", sub_cmd.pattern));
            }
        }
    }

    fn substitute_count(&mut self, pattern: &str, range: ExRange, ci: bool) {
        let re = compile_vim_pattern(pattern, !ci);
        let buf_id = self.current_buffer_id();
        let mut total = 0;
        if let Some(buf) = self.buffers.get(buf_id) {
            for l in range.start..=range.end.min(buf.content.len_lines().saturating_sub(1)) {
                let s: String = buf.content.line(l).chars().collect();
                total += count_via_regex(&s, pattern, true, re.as_ref());
            }
        }
        self.notify_info(&format!("{total} match(es) found"));
    }
}

/// Substitute using regex if available, else plain text.
fn substitute_line_regex(
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
fn count_via_regex(line: &str, pattern: &str, global: bool, re: Option<&regex::Regex>) -> usize {
    if let Some(re) = re {
        if global {
            re.find_iter(line).count()
        } else if re.is_match(line) {
            1
        } else {
            0
        }
    } else if global {
        line.matches(pattern).count()
    } else if line.contains(pattern) {
        1
    } else {
        0
    }
}
