//! TUI rendering: draw buffer content, status line, and command line.

use anyhow::Result;
use crossterm::{cursor, execute, style};
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Mode;
use std::io::{Write, stdout};

/// Render current state to the terminal.
pub(crate) fn render_frame(state: &EditorState) -> Result<()> {
    let mut out = stdout();
    execute!(out, cursor::Hide, cursor::MoveTo(0, 0))?;

    let win = match state.active_window_state() {
        Some(w) => w, None => return Ok(()),
    };
    let buf = match state.active_buffer() {
        Some(b) => b, None => return Ok(()),
    };

    let height = state.size.height.saturating_sub(2) as usize;
    let width = state.size.width as usize;

    for row in 0..height {
        let line_idx = win.top_line + row;
        execute!(out, cursor::MoveTo(0, row as u16))?;
        if line_idx < buf.text.line_count() {
            let text = buf.text.line_to_string(line_idx);
            let truncated: String = text.chars().take(width).collect();
            let pad = width.saturating_sub(truncated.len());
            write!(out, "{}{}", truncated, " ".repeat(pad))?;
        } else {
            write!(out, "~{}", " ".repeat(width.saturating_sub(1)))?;
        }
    }

    // Status line
    let status_row = state.size.height.saturating_sub(2);
    execute!(out, cursor::MoveTo(0, status_row), style::SetAttribute(style::Attribute::Reverse))?;
    let mode_str = format!(" {} ", state.current_mode());
    let file_name = buf.file_path.as_deref().unwrap_or("[No Name]");
    let modified = if buf.modified { " [+]" } else { "" };
    let lang_str = format!(" {} ", buf.language);
    let total = buf.text.line_count();
    let pct = if total <= 1 { "All".to_string() }
        else if win.cursor_line == 0 { "Top".to_string() }
        else if win.cursor_line >= total.saturating_sub(1) { "Bot".to_string() }
        else { format!("{}%", win.cursor_line * 100 / total) };
    let pos_str = format!(" {}:{} {} ", win.cursor_line + 1, win.cursor_col + 1, pct);
    let left_len = mode_str.len() + file_name.len() + modified.len();
    let right_len = lang_str.len() + pos_str.len();
    let middle_pad = width.saturating_sub(left_len).saturating_sub(right_len);
    write!(out, "{}{}{}{}{}{}", mode_str, file_name, modified, " ".repeat(middle_pad), lang_str, pos_str)?;
    execute!(out, style::SetAttribute(style::Attribute::Reset))?;

    // Message / command line
    let msg_row = state.size.height.saturating_sub(1);
    execute!(out, cursor::MoveTo(0, msg_row))?;
    if state.current_mode() == Mode::Command {
        let cmdline_display = format!("{}{}", state.cmdline.prefix, state.cmdline.text);
        let truncated = cmdline_display.chars().take(width).collect::<String>();
        let pad = width.saturating_sub(truncated.len());
        write!(out, "{}{}", truncated, " ".repeat(pad))?;
    } else if let Some(ref msg) = state.message {
        let truncated = msg.chars().take(width).collect::<String>();
        let pad = width.saturating_sub(truncated.len());
        write!(out, "{}{}", truncated, " ".repeat(pad))?;
    } else {
        write!(out, "{}", " ".repeat(width))?;
    }

    // Position cursor
    if state.current_mode() == Mode::Command {
        let cmd_col = (state.cmdline.cursor + 1) as u16;
        execute!(out, cursor::MoveTo(cmd_col, msg_row), cursor::Show)?;
    } else {
        let cursor_row = win.cursor_line.saturating_sub(win.top_line) as u16;
        let cursor_col = win.cursor_col as u16;
        execute!(out, cursor::MoveTo(cursor_col, cursor_row), cursor::Show)?;
    }
    out.flush()?;
    Ok(())
}
