mod command_routes;
mod input_routes;

use std::collections::VecDeque;
use std::io::{self, Read, Write};

use command_routes::action_from_command;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use input_routes::{action_from_key, format_key};
use kjxlkj_core_mode::Mode;
use kjxlkj_core_state::{EditorAction, EditorState};
use kjxlkj_input::{decode_byte, Key};
use kjxlkj_render::compute_render_diagnostics;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let result = run();
    disable_raw_mode()?;
    result
}

fn run() -> io::Result<()> {
    let initial_line = std::env::var("KJXLKJ_INITIAL_LINE").unwrap_or_else(|_| "abc".to_string());
    let start_cursor = std::env::var("KJXLKJ_START_CURSOR")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_else(|| initial_line.chars().count().saturating_sub(1));
    let rows = std::env::var("KJXLKJ_ROWS")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(20);
    let cols = std::env::var("KJXLKJ_COLS")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(80);
    let mut state = EditorState::new(initial_line, start_cursor);
    if let Ok(session_dump) = std::env::var("KJXLKJ_WINDOW_SESSION") {
        state
            .restore_window_session(&session_dump)
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid KJXLKJ_WINDOW_SESSION: {error}"),
                )
            })?;
    }
    state.set_window_area(rows, cols);
    let mut seq: u64 = 0;
    let mut awaiting_wincmd = false;
    let mut awaiting_terminal_exit = false;
    let mut awaiting_leader = false;
    let mut awaiting_terminal_leader_suffix = false;
    let mut command_mode = false;
    let mut command_buffer = String::new();
    let mut recent_events: VecDeque<String> = VecDeque::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let mut one = [0_u8; 1];
        if stdin.read_exact(&mut one).is_err() {
            break;
        }
        seq += 1;
        let decoded = decode_byte(one[0]);
        let supports_wincmd = matches!(state.mode(), Mode::Normal | Mode::TerminalInsert);
        let action = if command_mode {
            match decoded.normalized_key {
                Key::Enter => {
                    command_mode = false;
                    action_from_command(&command_buffer)
                }
                Key::Esc => {
                    command_mode = false;
                    command_buffer.clear();
                    EditorAction::Ignore
                }
                Key::Char(ch) => {
                    command_buffer.push(ch);
                    EditorAction::Ignore
                }
                _ => EditorAction::Ignore,
            }
        } else if state.mode() == Mode::Normal && decoded.normalized_key == Key::Char(':') {
            command_mode = true;
            command_buffer.clear();
            EditorAction::Ignore
        } else if state.mode() == Mode::TerminalInsert && awaiting_terminal_exit {
            awaiting_terminal_exit = false;
            if decoded.normalized_key == Key::Ctrl('n') {
                EditorAction::TerminalExitToNormal
            } else {
                action_from_key(
                    state.mode(),
                    decoded.normalized_key,
                    state.focused_window_kind(),
                )
            }
        } else if state.mode() == Mode::TerminalInsert && decoded.normalized_key == Key::Ctrl('\\')
        {
            awaiting_terminal_exit = true;
            EditorAction::Ignore
        } else if supports_wincmd && awaiting_wincmd {
            awaiting_wincmd = false;
            match decoded.normalized_key {
                Key::Char(ch) => EditorAction::WindowCommand(ch),
                _ => EditorAction::Ignore,
            }
        } else if supports_wincmd && decoded.normalized_key == Key::Ctrl('w') {
            awaiting_wincmd = true;
            EditorAction::Ignore
        } else if state.mode() == Mode::Normal && awaiting_terminal_leader_suffix {
            awaiting_terminal_leader_suffix = false;
            match decoded.normalized_key {
                Key::Char('h') => EditorAction::WindowCommand('H'),
                Key::Char('v') | Key::Enter => EditorAction::WindowCommand('T'),
                _ => EditorAction::WindowCommand('T'),
            }
        } else if state.mode() == Mode::Normal && awaiting_leader {
            awaiting_leader = false;
            match decoded.normalized_key {
                Key::Char('e') | Key::Char('E') => EditorAction::WindowCommand('E'),
                Key::Char('t') => {
                    awaiting_terminal_leader_suffix = true;
                    EditorAction::Ignore
                }
                _ => EditorAction::Ignore,
            }
        } else if state.mode() == Mode::Normal && decoded.normalized_key == Key::Char(' ') {
            awaiting_leader = true;
            EditorAction::Ignore
        } else {
            awaiting_wincmd = false;
            awaiting_leader = false;
            awaiting_terminal_leader_suffix = false;
            action_from_key(
                state.mode(),
                decoded.normalized_key,
                state.focused_window_kind(),
            )
        };
        if !command_mode {
            command_buffer.clear();
        }
        let result = state.apply(action);
        let render = compute_render_diagnostics(state.line(), state.cursor(), cols, rows);
        let normalized_key = format_key(decoded.normalized_key);
        recent_events.push_back(format!(
            "{seq}:{normalized_key}->{action}",
            action = result.resolved_action
        ));
        if recent_events.len() > 20 {
            recent_events.pop_front();
        }
        writeln!(
            stdout,
            "TRACE event_seq={} mode_before={:?} focused_window_id={} focused_window_type={} normalized_key={} resolved_action={} cursor_before={} cursor_after={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} line={}",
            seq,
            result.mode_before,
            state.focused_window_id(),
            state.focused_window_kind(),
            normalized_key,
            result.resolved_action,
            result.cursor_before,
            result.cursor_after,
            state.window_geometry_ok(),
            render.bounds_ok,
            render.cursor_visible,
            render.cursor_on_continuation,
            render.cursor_span,
            render.wrap_signature,
            state.line()
        )?;
        stdout.flush()?;
        if result.should_quit {
            break;
        }
    }

    let final_render = compute_render_diagnostics(state.line(), state.cursor(), cols, rows);
    let recent_joined = recent_events.into_iter().collect::<Vec<_>>().join("|");
    writeln!(
        stdout,
        "FINAL mode={:?} cursor={} focused_window_id={} focused_window_type={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} line={} window_session={} recent_events={}",
        state.mode(),
        state.cursor(),
        state.focused_window_id(),
        state.focused_window_kind(),
        state.window_geometry_ok(),
        final_render.bounds_ok,
        final_render.cursor_visible,
        final_render.cursor_on_continuation,
        final_render.cursor_span,
        final_render.wrap_signature,
        state.line(),
        state.window_session_dump(),
        recent_joined
    )?;
    stdout.flush()
}
