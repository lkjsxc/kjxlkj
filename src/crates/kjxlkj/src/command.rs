//! Ex command execution.

use kjxlkj_core::EditorState;
use std::path::Path;

/// Execute an Ex command.
pub async fn execute_command(editor: &mut EditorState, cmd: &str) {
    let cmd = cmd.trim();

    // Handle search commands.
    if let Some(pattern) = cmd.strip_prefix('/') {
        editor.search_pattern = Some(pattern.to_string());
        editor.search_forward = true;
        search_next(editor);
        return;
    }
    if let Some(pattern) = cmd.strip_prefix('?') {
        editor.search_pattern = Some(pattern.to_string());
        editor.search_forward = false;
        search_prev(editor);
        return;
    }

    // Parse command.
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let command = parts[0];
    let args = parts.get(1).map(|s| s.trim());

    match command {
        "q" | "q!" | "qa" | "qa!" => {
            if command.contains('!') || !editor.buffer.modified {
                editor.should_quit = true;
            } else {
                editor.set_error("No write since last change (add ! to override)");
            }
        }
        "w" => {
            if let Some(path) = args {
                editor.buffer.path = Some(path.to_string());
            }
            write_buffer(editor).await;
        }
        "wa" => {
            write_buffer(editor).await;
        }
        "wq" | "x" => {
            if let Some(path) = args {
                editor.buffer.path = Some(path.to_string());
            }
            write_buffer(editor).await;
            if !editor.status_error {
                editor.should_quit = true;
            }
        }
        "e" | "e!" => {
            if let Some(path) = args {
                load_file(editor, path, command.contains('!')).await;
            } else {
                editor.set_error("No file name");
            }
        }
        "!" => {
            if let Some(shell_cmd) = args {
                execute_shell(editor, shell_cmd).await;
            }
        }
        "s" => {
            if let Some(sub_args) = args {
                execute_substitute(editor, sub_args);
            }
        }
        "g" => {
            if let Some(global_args) = args {
                execute_global(editor, global_args, false);
            }
        }
        "v" => {
            if let Some(global_args) = args {
                execute_global(editor, global_args, true);
            }
        }
        _ => {
            // Try to parse as line number.
            if let Ok(line_num) = command.parse::<usize>() {
                let max = editor.buffer.text.len_lines();
                editor.buffer.cursor.line = line_num.saturating_sub(1).min(max.saturating_sub(1));
                editor.buffer.clamp_cursor();
            } else {
                editor.set_error(format!("Unknown command: {}", command));
            }
        }
    }
}

async fn write_buffer(editor: &mut EditorState) {
    if let Some(ref path) = editor.buffer.path {
        let content = editor.buffer.text.to_string();
        match kjxlkj_service_fs::FsService::write_file(Path::new(path), &content).await {
            Ok(()) => {
                editor.buffer.modified = false;
                let lines = editor.buffer.text.len_lines();
                let bytes = content.len();
                editor.set_message(format!("\"{}\" {}L, {}B written", path, lines, bytes));
            }
            Err(e) => {
                editor.set_error(format!("Error writing: {}", e));
            }
        }
    } else {
        editor.set_error("No file name");
    }
}

async fn load_file(editor: &mut EditorState, path: &str, force: bool) {
    if editor.buffer.modified && !force {
        editor.set_error("No write since last change (add ! to override)");
        return;
    }

    let path_obj = Path::new(path);
    if path_obj.exists() {
        match kjxlkj_service_fs::FsService::read_file(path_obj).await {
            Ok(content) => {
                editor.buffer = kjxlkj_core::state::BufferState::from_file(
                    kjxlkj_core::types::BufferId::new(0),
                    path.to_string(),
                    &content,
                );
                let lines = editor.buffer.text.len_lines();
                editor.set_message(format!("\"{}\" {}L", path, lines));
            }
            Err(e) => {
                editor.set_error(format!("Error reading: {}", e));
            }
        }
    } else {
        editor.buffer = kjxlkj_core::state::BufferState::new(kjxlkj_core::types::BufferId::new(0));
        editor.buffer.path = Some(path.to_string());
        editor.set_message(format!("\"{}\" [New File]", path));
    }
}

async fn execute_shell(editor: &mut EditorState, cmd: &str) {
    match kjxlkj_service_terminal::TerminalService::execute(cmd).await {
        Ok(output) => {
            let first_line = output.lines().next().unwrap_or("").to_string();
            editor.set_message(first_line);
        }
        Err(e) => {
            editor.set_error(format!("Shell error: {}", e));
        }
    }
}

fn execute_substitute(editor: &mut EditorState, args: &str) {
    // Parse :s/pattern/replacement/flags
    let parts: Vec<&str> = args.split('/').collect();
    if parts.len() < 3 {
        editor.set_error("Invalid substitute syntax");
        return;
    }

    let pattern = parts[0];
    let replacement = parts[1];
    let flags = parts.get(2).unwrap_or(&"");
    let global = flags.contains('g');

    let line_idx = editor.buffer.cursor.line;
    if let Some(line_content) = editor.buffer.text.line_content(line_idx) {
        let new_content = if global {
            line_content.replace(pattern, replacement)
        } else {
            line_content.replacen(pattern, replacement, 1)
        };

        if new_content != line_content {
            // Replace the line.
            if let Some(start_byte) = editor
                .buffer
                .text
                .cursor_to_byte(kjxlkj_core::Cursor::new(line_idx, 0))
            {
                let old_len = line_content.len();
                editor
                    .buffer
                    .text
                    .delete_range(start_byte, start_byte + old_len);
                editor.buffer.text.insert(start_byte, &new_content);
                editor.buffer.modified = true;
            }
        }
    }
}

fn execute_global(editor: &mut EditorState, args: &str, inverted: bool) {
    // Parse :g/pattern/command or :v/pattern/command
    let parts: Vec<&str> = args.splitn(3, '/').collect();
    if parts.len() < 2 {
        editor.set_error("Invalid global syntax");
        return;
    }

    let pattern = parts[0];
    let command = parts.get(1).unwrap_or(&"d");

    let total_lines = editor.buffer.text.len_lines();
    let mut lines_to_process: Vec<usize> = Vec::new();

    for i in 0..total_lines {
        if let Some(content) = editor.buffer.text.line_content(i) {
            let matches = content.contains(pattern);
            if matches != inverted {
                lines_to_process.push(i);
            }
        }
    }

    // Process in reverse order for deletions.
    if *command == "d" {
        lines_to_process.reverse();
        for line_idx in lines_to_process {
            if let (Some(start_byte), Some(end_byte)) = (
                editor
                    .buffer
                    .text
                    .cursor_to_byte(kjxlkj_core::Cursor::new(line_idx, 0)),
                if line_idx + 1 < editor.buffer.text.len_lines() {
                    editor
                        .buffer
                        .text
                        .cursor_to_byte(kjxlkj_core::Cursor::new(line_idx + 1, 0))
                } else {
                    Some(editor.buffer.text.len_bytes())
                },
            ) {
                editor.buffer.text.delete_range(start_byte, end_byte);
            }
        }
        editor.buffer.modified = true;
        editor.buffer.clamp_cursor();
    }
}

fn search_next(editor: &mut EditorState) {
    if let Some(ref pattern) = editor.search_pattern.clone() {
        let total_lines = editor.buffer.text.len_lines();
        let start_line = editor.buffer.cursor.line;
        let start_col = editor.buffer.cursor.col + 1;

        // Search from cursor position.
        for offset in 0..total_lines {
            let line_idx = (start_line + offset) % total_lines;
            if let Some(content) = editor.buffer.text.line_content(line_idx) {
                let search_start = if offset == 0 { start_col } else { 0 };
                if let Some(pos) = content[search_start..].find(pattern) {
                    editor.buffer.cursor.line = line_idx;
                    editor.buffer.cursor.col = search_start + pos;
                    if offset >= total_lines - start_line {
                        editor.set_message("search hit BOTTOM, continuing at TOP");
                    }
                    return;
                }
            }
        }
        editor.set_error(format!("Pattern not found: {}", pattern));
    }
}

fn search_prev(editor: &mut EditorState) {
    if let Some(ref pattern) = editor.search_pattern.clone() {
        let total_lines = editor.buffer.text.len_lines();
        let start_line = editor.buffer.cursor.line;
        let start_col = editor.buffer.cursor.col;

        // Search backward from cursor position.
        for offset in 0..total_lines {
            let line_idx = (start_line + total_lines - offset) % total_lines;
            if let Some(content) = editor.buffer.text.line_content(line_idx) {
                let search_end = if offset == 0 {
                    start_col
                } else {
                    content.len()
                };
                if let Some(pos) = content[..search_end].rfind(pattern) {
                    editor.buffer.cursor.line = line_idx;
                    editor.buffer.cursor.col = pos;
                    if offset > start_line {
                        editor.set_message("search hit TOP, continuing at BOTTOM");
                    }
                    return;
                }
            }
        }
        editor.set_error(format!("Pattern not found: {}", pattern));
    }
}
