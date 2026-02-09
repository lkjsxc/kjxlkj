/// Substitute command execution and helpers.
use crate::editor::EditorState;
use crate::ex_parse::ExRange;
use crate::ex_parse_substitute::parse_substitute;

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

        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;

        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);

            let mut total_replacements = 0;
            let mut total_lines_changed = 0;

            for line_idx in range.start..=range.end.min(buf.content.len_lines().saturating_sub(1)) {
                let line_slice = buf.content.line(line_idx);
                let line_str: String = line_slice.chars().collect();

                let new_line = if sub_cmd.case_insensitive {
                    substitute_line_ci(
                        &line_str,
                        &sub_cmd.pattern,
                        &sub_cmd.replacement,
                        sub_cmd.global,
                    )
                } else {
                    substitute_line(
                        &line_str,
                        &sub_cmd.pattern,
                        &sub_cmd.replacement,
                        sub_cmd.global,
                    )
                };

                if new_line != line_str {
                    let count = count_replacements(
                        &line_str,
                        &sub_cmd.pattern,
                        sub_cmd.global,
                        sub_cmd.case_insensitive,
                    );
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

    fn substitute_count(&mut self, pattern: &str, range: ExRange, case_insensitive: bool) {
        let buf_id = self.current_buffer_id();
        let mut total = 0;

        if let Some(buf) = self.buffers.get(buf_id) {
            for line_idx in range.start..=range.end.min(buf.content.len_lines().saturating_sub(1)) {
                let line_slice = buf.content.line(line_idx);
                let line_str: String = line_slice.chars().collect();
                total += count_replacements(&line_str, pattern, true, case_insensitive);
            }
        }

        self.notify_info(&format!("{total} match(es) found"));
    }
}

/// Plain-text substitution on a single line.
pub(crate) fn substitute_line(
    line: &str,
    pattern: &str,
    replacement: &str,
    global: bool,
) -> String {
    if global {
        line.replace(pattern, replacement)
    } else {
        line.replacen(pattern, replacement, 1)
    }
}

/// Case-insensitive substitution.
pub(crate) fn substitute_line_ci(
    line: &str,
    pattern: &str,
    replacement: &str,
    global: bool,
) -> String {
    let pat_lower = pattern.to_lowercase();
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    loop {
        let lower = remaining.to_lowercase();
        if let Some(pos) = lower.find(&pat_lower) {
            result.push_str(&remaining[..pos]);
            result.push_str(replacement);
            let advance = pos + pattern.len();
            remaining = &remaining[advance..];
            if !global {
                result.push_str(remaining);
                return result;
            }
        } else {
            result.push_str(remaining);
            return result;
        }
    }
}

/// Count how many replacements would occur.
pub(crate) fn count_replacements(
    line: &str,
    pattern: &str,
    global: bool,
    case_insensitive: bool,
) -> usize {
    if case_insensitive {
        let line_lower = line.to_lowercase();
        let pat_lower = pattern.to_lowercase();
        if global {
            line_lower.matches(&pat_lower).count()
        } else if line_lower.contains(&pat_lower) {
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
