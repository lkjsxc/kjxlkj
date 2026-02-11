mod command_routes;
mod input_routes;
mod profiling;
mod trace_output;

use command_routes::action_from_command;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use input_routes::{action_from_key, format_key};
use kjxlkj_core_mode::Mode;
use kjxlkj_core_state::{EditorAction, EditorState};
use kjxlkj_input::{ByteStreamDecoder, Key};
use kjxlkj_render::compute_render_diagnostics;
use profiling::PerfProfile;
use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::time::Instant;
use trace_output::{emit_final, emit_trace};

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
    let mut key_decoder = ByteStreamDecoder::new();
    let mut profile = PerfProfile::from_env();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let mut one = [0_u8; 1];
        if stdin.read_exact(&mut one).is_err() {
            break;
        }
        let Some(decoded) = key_decoder.decode_stream_byte(one[0]) else {
            continue;
        };
        seq += 1;
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
        let snapshot_start = Instant::now();
        let render = compute_render_diagnostics(state.line(), state.cursor(), cols, rows);
        let snapshot_duration = snapshot_start.elapsed();
        let normalized_key = format_key(decoded.normalized_key);
        recent_events.push_back(format!(
            "{seq}:0x{:02X}:{normalized_key}->{action}",
            one[0],
            action = result.resolved_action
        ));
        if recent_events.len() > 20 {
            recent_events.pop_front();
        }
        let render_start = Instant::now();
        emit_trace(&mut stdout, seq, &result, &state, render, &normalized_key)?;
        stdout.flush()?;
        let render_duration = render_start.elapsed();
        profile.record_cycle(
            &state,
            rows,
            cols,
            render,
            &result.resolved_action,
            snapshot_duration,
            render_duration,
        );
        if result.should_quit {
            break;
        }
    }

    let final_render = compute_render_diagnostics(state.line(), state.cursor(), cols, rows);
    let recent_joined = recent_events.into_iter().collect::<Vec<_>>().join("|");
    profile.emit_final(&mut stdout, rows)?;
    emit_final(&mut stdout, &state, final_render, &recent_joined)?;
    stdout.flush()
}
