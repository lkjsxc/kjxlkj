//! TUI mode: terminal event loop with rendering.

use anyhow::Result;
use kjxlkj_core_state::{dispatch_intent, handle_cmdline_key, EditorState};
use kjxlkj_core_types::Mode;
use kjxlkj_host::TerminalHost;
use kjxlkj_input::InputDecoder;
use kjxlkj_render::Renderer;

use crate::tui_render::render_frame;

/// Run the editor in interactive TUI mode.
pub async fn run_tui(file: Option<String>) -> Result<()> {
    let mut host = TerminalHost::new();
    host.enter()?;

    let size = host.size()?;
    let mut state = EditorState::new(size);
    let decoder = InputDecoder::new();
    let mut _renderer = Renderer::new(size);

    let text = if let Some(ref path) = file {
        match std::fs::read_to_string(path) { Ok(t) => t, Err(_) => String::new() }
    } else { String::new() };

    let bid = if text.is_empty() { state.create_buffer() } else { state.create_buffer_from_text(&text) };
    if let Some(ref path) = file {
        if let Some(buf) = state.buffers.get_mut(&bid) { buf.set_file_path(path.as_str()); }
    }
    let wid = state.create_window(bid);
    if let Some(win) = state.windows.get_mut(&wid) { win.height = size.height.saturating_sub(2) as usize; }

    loop {
        render_frame(&state)?;
        let event = crossterm::event::read()?;
        if let Some(editor_event) = decoder.decode(event) {
            match editor_event {
                kjxlkj_input::EditorEvent::Key(key) => {
                    let core_key = convert_key(&key);
                    let intent = match state.current_mode() {
                        Mode::Normal | Mode::OperatorPending | Mode::InsertNormal => state.parser.parse_normal(&core_key),
                        Mode::Insert => state.parser.parse_insert(&core_key),
                        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => state.parser.parse_visual(&core_key),
                        Mode::Command => handle_cmdline_key(&mut state, &core_key),
                        Mode::Replace => state.parser.parse_replace(&core_key),
                        Mode::Terminal => state.parser.parse_insert(&core_key),
                    };
                    dispatch_intent(&mut state, intent);
                }
                kjxlkj_input::EditorEvent::Resize(new_size) => {
                    state.size = new_size;
                    _renderer.resize(new_size);
                    if let Some(win) = state.active_window_mut() { win.height = new_size.height.saturating_sub(2) as usize; }
                }
                _ => {}
            }
        }
        if state.should_quit { break; }
    }
    host.leave()?;
    Ok(())
}

/// Convert input crate key to core-types key.
fn convert_key(key: &kjxlkj_input::KeyEvent) -> kjxlkj_core_types::KeyEvent {
    use kjxlkj_core_types::KeyCode as CK;
    use kjxlkj_input::KeyCode as IK;
    let code = match &key.code {
        IK::Char(c) => CK::Char(*c), IK::Esc => CK::Escape,
        IK::Enter => CK::Enter, IK::Backspace => CK::Backspace,
        IK::Tab | IK::BackTab => CK::Tab, IK::Delete => CK::Delete,
        IK::Up => CK::Up, IK::Down => CK::Down,
        IK::Left => CK::Left, IK::Right => CK::Right,
        IK::Home => CK::Home, IK::End => CK::End,
        IK::PageUp => CK::PageUp, IK::PageDown => CK::PageDown,
        IK::F(n) => CK::F(*n),
    };
    kjxlkj_core_types::KeyEvent { code, ctrl: key.modifiers.ctrl, alt: key.modifiers.alt, shift: key.modifiers.shift }
}
