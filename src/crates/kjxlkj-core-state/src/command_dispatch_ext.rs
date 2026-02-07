//! Extended command dispatch for less-common Ex commands.

use kjxlkj_core_types::{EditorError, Position};

use crate::commands::ExCommand;
use crate::editor_state::EditorState;
use crate::syntax_cmd;

/// Dispatch extended commands that are not handled by the main dispatcher.
pub fn dispatch_extended(state: &mut EditorState, cmd: ExCommand) -> Result<(), EditorError> {
    match cmd {
        ExCommand::Marks => {
            let lines: Vec<String> = state
                .marks
                .iter()
                .map(|(ch, m)| {
                    format!(
                        " {ch}  {:>5}  {:>3}  {}",
                        m.position.line + 1,
                        m.position.col,
                        m.buffer_id
                    )
                })
                .collect();
            state.set_message(if lines.is_empty() {
                "no marks".into()
            } else {
                lines.join("\n")
            });
            Ok(())
        }
        ExCommand::Registers => {
            let lines: Vec<String> = state
                .registers
                .iter()
                .map(|(ch, r)| {
                    let content = if r.content.len() > 40 {
                        format!("{}...", &r.content[..40])
                    } else {
                        r.content.clone()
                    };
                    format!("\"{ch}   {content}")
                })
                .collect();
            state.set_message(if lines.is_empty() {
                "no registers".into()
            } else {
                lines.join("\n")
            });
            Ok(())
        }
        ExCommand::Jumps => {
            let entries = state.jump_list.entries();
            let lines: Vec<String> = entries
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    format!(
                        "{:>3} {:>5} {:>3}  {}",
                        i,
                        e.position.line + 1,
                        e.position.col,
                        e.buffer_id
                    )
                })
                .collect();
            state.set_message(if lines.is_empty() {
                "no jumps".into()
            } else {
                lines.join("\n")
            });
            Ok(())
        }
        ExCommand::Changes => {
            let entries = state.change_list.entries();
            let lines: Vec<String> = entries
                .iter()
                .enumerate()
                .map(|(i, e)| format!("{:>3} {:>5} {:>3}", i, e.position.line + 1, e.position.col))
                .collect();
            state.set_message(if lines.is_empty() {
                "no changes".into()
            } else {
                lines.join("\n")
            });
            Ok(())
        }
        ExCommand::FileInfo => {
            let buf = state.active_buffer();
            let name = buf.path().unwrap_or(buf.name());
            let lines = buf.line_count();
            let modified = if buf.is_modified() { " [Modified]" } else { "" };
            state.set_message(format!("\"{name}\"{modified} {lines} lines"));
            Ok(())
        }
        ExCommand::Sort => {
            let buf = state.active_buffer_mut();
            let mut all_lines: Vec<String> =
                (0..buf.line_count()).filter_map(|i| buf.line(i)).collect();
            all_lines.sort();
            let new_text = all_lines.join("\n");
            let start = Position::ZERO;
            let end = Position::new(buf.line_count(), 0);
            buf.delete_range(start, end);
            buf.insert_text(Position::ZERO, &new_text);
            Ok(())
        }
        ExCommand::Pwd => {
            let cwd = std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".into());
            state.set_message(cwd);
            Ok(())
        }
        ExCommand::Cd(dir) => {
            if dir.is_empty() {
                return dispatch_extended(state, ExCommand::Pwd);
            }
            let expanded = crate::file_commands::expand_tilde(&dir);
            std::env::set_current_dir(&expanded)
                .map_err(|e| EditorError::InvalidCommand(format!("cd: {e}")))?;
            let msg = format!("cd: {expanded}");
            state.set_message(msg);
            Ok(())
        }
        ExCommand::SyntaxCmd(arg) => {
            if let Some(action) = syntax_cmd::parse_syntax_command(&arg) {
                match action {
                    syntax_cmd::SyntaxAction::On | syntax_cmd::SyntaxAction::Enable => {
                        state.options.syntax = true;
                    }
                    syntax_cmd::SyntaxAction::Off | syntax_cmd::SyntaxAction::Disable => {
                        state.options.syntax = false;
                    }
                    syntax_cmd::SyntaxAction::Manual => {
                        state.set_message("syntax manual mode");
                    }
                }
            } else {
                let info = syntax_cmd::format_syntax_info(state.options.syntax, "");
                state.set_message(info);
            }
            Ok(())
        }
        ExCommand::Messages => {
            state.set_message(
                state
                    .message
                    .clone()
                    .unwrap_or_else(|| "no messages".into()),
            );
            Ok(())
        }
        ExCommand::ShellCommand(cmd) => {
            state.set_message(format!("shell: {cmd} (not yet implemented)"));
            Ok(())
        }
        ExCommand::Substitute(pat, rep, flags) => dispatch_substitute(state, &pat, &rep, &flags),
        _ => dispatch_remaining(state, cmd),
    }
}

fn dispatch_substitute(
    state: &mut EditorState,
    pattern: &str,
    replacement: &str,
    flags: &str,
) -> Result<(), EditorError> {
    let global = flags.contains('g');
    let cursor_line = state.windows[state.active_window].cursor.line;
    let buf = state.active_buffer_mut();
    if let Some(line_text) = buf.line(cursor_line) {
        let re = crate::search_regex::compile_pattern(pattern, true)?;
        let new_line = if global {
            re.replace_all(&line_text, replacement).to_string()
        } else {
            re.replace(&line_text, replacement).to_string()
        };
        if new_line != line_text {
            let start = Position::new(cursor_line, 0);
            let end = Position::new(cursor_line, line_text.len());
            buf.delete_range(start, end);
            buf.insert_text(start, &new_line);
        }
    }
    Ok(())
}

fn dispatch_remaining(state: &mut EditorState, cmd: ExCommand) -> Result<(), EditorError> {
    let name = cmd.name();
    state.set_message(format!("{name}: not yet implemented"));
    Ok(())
}
