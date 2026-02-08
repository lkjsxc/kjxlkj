//! Shell command execution and expression evaluation.

use crate::EditorState;

impl EditorState {
    /// Execute a shell command (`:!{cmd}`).
    /// Output is stored in the `"` register.
    pub(crate) fn do_shell_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return;
        }
        // Execute shell command and capture output.
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output();
        match output {
            Ok(out) => {
                let text = String::from_utf8_lossy(
                    &out.stdout,
                )
                .to_string();
                // Store output in unnamed register.
                use kjxlkj_core_types::RegisterName;
                self.register_file.store(
                    RegisterName::Unnamed,
                    text,
                    false,
                );
            }
            Err(_) => {}
        }
    }

    /// Filter lines through a shell command
    /// (`:1,5!sort` etc).
    pub(crate) fn do_filter_lines(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            return;
        }
        // Parse range and command from args.
        let (line, _) = self.cursor_pos();
        let line_count = self
            .active_buffer()
            .map(|b| b.content.line_count())
            .unwrap_or(1);
        let (start, end, cmd) =
            crate::editor_range_cmds::parse_range(
                args, line, line_count,
            );
        if cmd.is_empty() {
            return;
        }
        // Collect lines to filter.
        let input: String = (start..=end)
            .filter_map(|l| {
                self.active_buffer().map(|b| {
                    b.content.line_str(l)
                })
            })
            .collect::<Vec<_>>()
            .join("");
        // Run filter.
        let result =
            run_filter(&cmd, &input).unwrap_or(input);
        // Replace the range with the result.
        if let Some(buf) = self.active_buffer_mut() {
            for l in (start..=end).rev() {
                buf.content.delete_lines(l, l);
            }
            let result_lines: Vec<&str> =
                result.lines().collect();
            for (i, text) in
                result_lines.iter().enumerate()
            {
                let ins_line = start + i;
                let content = format!("{}\n", text);
                if ins_line < buf.content.line_count() {
                    let off = buf
                        .content
                        .line_start_offset(ins_line);
                    for ch in content.chars().rev() {
                        buf.content
                            .insert_char(off, ch);
                    }
                } else {
                    let off = buf.content.len_chars();
                    for ch in content.chars() {
                        buf.content.insert_char(off, ch);
                    }
                }
            }
            buf.modified = true;
        }
    }

    /// Execute command from string (`:execute {expr}`).
    pub(crate) fn do_execute_expr(&mut self, expr: &str) {
        let expr = expr.trim();
        // Strip surrounding quotes if present.
        let cmd = if (expr.starts_with('"')
            && expr.ends_with('"'))
            || (expr.starts_with('\'')
                && expr.ends_with('\''))
        {
            &expr[1..expr.len() - 1]
        } else {
            expr
        };
        if !cmd.is_empty() {
            if let Some(action) =
                crate::dispatch_command(cmd)
            {
                self.dispatch(action);
            }
        }
    }
}

/// Run a shell filter command on input text.
fn run_filter(
    cmd: &str,
    input: &str,
) -> Option<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(input.as_bytes());
    }
    let out = child.wait_with_output().ok()?;
    Some(
        String::from_utf8_lossy(&out.stdout).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_expr_simple() {
        let mut ed = EditorState::new(80, 24);
        ed.do_execute_expr("\"set\"");
        // Should not crash; `set` is a Nop command.
    }

    #[test]
    fn shell_command_runs() {
        let mut ed = EditorState::new(80, 24);
        ed.do_shell_command("echo hello");
        let reg = ed
            .register_file
            .get(kjxlkj_core_types::RegisterName::Unnamed);
        assert!(reg.is_some());
        assert!(reg.unwrap().content.contains("hello"));
    }
}
