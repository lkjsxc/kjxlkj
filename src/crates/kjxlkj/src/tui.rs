//! TUI mode: terminal event loop with rendering.

use anyhow::Result;
use kjxlkj_core_state::{dispatch_intent, handle_cmdline_key, EditorState};
use kjxlkj_core_types::{Mode, Size};
use kjxlkj_host::TerminalHost;
use kjxlkj_input::InputDecoder;
use kjxlkj_render::Renderer;

/// Run the editor in interactive TUI mode.
pub async fn run_tui(file: Option<String>) -> Result<()> {
    let mut host = TerminalHost::new();
    host.enter()?;

    let size = host.size()?;
    let mut state = EditorState::new(size);
    let decoder = InputDecoder::new();
    let mut renderer = Renderer::new(size);

    // Create initial buffer
    let text = if let Some(ref path) = file {
        match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let bid = if text.is_empty() {
        state.create_buffer()
    } else {
        state.create_buffer_from_text(&text)
    };

    if let Some(ref path) = file {
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.set_file_path(path.as_str());
        }
    }

    let wid = state.create_window(bid);
    if let Some(win) = state.windows.get_mut(&wid) {
        win.height = size.height.saturating_sub(2) as usize;
    }

    // Main event loop
    loop {
        // Render
        render_frame(&state, &mut renderer)?;

        // Wait for input
        let event = crossterm::event::read()?;
        if let Some(editor_event) = decoder.decode(event) {
            match editor_event {
                kjxlkj_input::EditorEvent::Key(key) => {
                    let core_key = convert_key(&key);
                    let intent = match state.current_mode() {
                        Mode::Normal | Mode::OperatorPending | Mode::InsertNormal => {
                            state.parser.parse_normal(&core_key)
                        }
                        Mode::Insert => state.parser.parse_insert(&core_key),
                        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                            state.parser.parse_visual(&core_key)
                        }
                        Mode::Command => {
                            let intent = handle_cmdline_key(
                                &mut state,
                                &core_key,
                            );
                            intent
                        }
                        Mode::Replace => {
                            state.parser.parse_replace(&core_key)
                        }
                    };
                    dispatch_intent(&mut state, intent);
                }
                kjxlkj_input::EditorEvent::Resize(new_size) => {
                    state.size = new_size;
                    renderer.resize(new_size);
                    if let Some(win) = state.active_window_mut() {
                        win.height = new_size.height.saturating_sub(2) as usize;
                    }
                }
                _ => {}
            }
        }

        if state.should_quit {
            break;
        }
    }

    host.leave()?;
    Ok(())
}

/// Convert input crate key to core-types key.
fn convert_key(key: &kjxlkj_input::KeyEvent) -> kjxlkj_core_types::KeyEvent {
    use kjxlkj_core_types::KeyCode as CK;
    use kjxlkj_input::KeyCode as IK;

    let code = match &key.code {
        IK::Char(c) => CK::Char(*c),
        IK::Esc => CK::Escape,
        IK::Enter => CK::Enter,
        IK::Backspace => CK::Backspace,
        IK::Tab | IK::BackTab => CK::Tab,
        IK::Delete => CK::Delete,
        IK::Up => CK::Up,
        IK::Down => CK::Down,
        IK::Left => CK::Left,
        IK::Right => CK::Right,
        IK::Home => CK::Home,
        IK::End => CK::End,
        IK::PageUp => CK::PageUp,
        IK::PageDown => CK::PageDown,
        IK::F(n) => CK::F(*n),
    };
    kjxlkj_core_types::KeyEvent {
        code,
        ctrl: key.modifiers.ctrl,
        alt: key.modifiers.alt,
        shift: key.modifiers.shift,
    }
}

/// Render current state to the terminal.
fn render_frame(
    state: &EditorState,
    _renderer: &mut Renderer,
) -> Result<()> {
    use crossterm::{cursor, execute, style, terminal};
    use std::io::{Write, stdout};

    let mut out = stdout();

    // Clear and draw buffer content
    execute!(out, cursor::Hide, cursor::MoveTo(0, 0))?;

    let win = match state.active_window_state() {
        Some(w) => w,
        None => return Ok(()),
    };
    let buf = match state.active_buffer() {
        Some(b) => b,
        None => return Ok(()),
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
    execute!(
        out,
        cursor::MoveTo(0, status_row),
        style::SetAttribute(style::Attribute::Reverse),
    )?;
    let mode_str = format!(" {} ", state.current_mode());
    let file_name = buf.text.name();
    let modified = if buf.modified { " [+]" } else { "" };
    let pos_str = format!(
        " {}:{} ",
        win.cursor_line + 1,
        win.cursor_col + 1
    );
    let middle_pad = width
        .saturating_sub(mode_str.len())
        .saturating_sub(file_name.len())
        .saturating_sub(modified.len())
        .saturating_sub(pos_str.len());
    write!(
        out,
        "{}{}{}{}{}",
        mode_str,
        file_name,
        modified,
        " ".repeat(middle_pad),
        pos_str
    )?;
    execute!(out, style::SetAttribute(style::Attribute::Reset))?;

    // Message / command line
    let msg_row = state.size.height.saturating_sub(1);
    execute!(out, cursor::MoveTo(0, msg_row))?;
    if state.current_mode() == Mode::Command {
        let cmdline_display = format!(
            "{}{}",
            state.cmdline.prefix,
            state.cmdline.text
        );
        let truncated =
            cmdline_display.chars().take(width).collect::<String>();
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
        let cmd_col = (state.cmdline.cursor + 1) as u16; // +1 for prefix char
        execute!(
            out,
            cursor::MoveTo(cmd_col, msg_row),
            cursor::Show
        )?;
    } else {
        let cursor_row =
            win.cursor_line.saturating_sub(win.top_line) as u16;
        let cursor_col = win.cursor_col as u16;
        execute!(
            out,
            cursor::MoveTo(cursor_col, cursor_row),
            cursor::Show
        )?;
    }

    out.flush()?;
    Ok(())
}
