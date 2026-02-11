use std::io::{self, Read, Write};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use kjxlkj_core_mode::Mode;
use kjxlkj_core_state::{EditorAction, EditorState};
use kjxlkj_input::{decode_byte, Key};

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
        let action = if state.mode() == Mode::TerminalInsert && awaiting_terminal_exit {
            awaiting_terminal_exit = false;
            if decoded.normalized_key == Key::Ctrl('n') {
                EditorAction::TerminalExitToNormal
            } else {
                action_from_key(state.mode(), decoded.normalized_key)
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
        } else {
            awaiting_wincmd = false;
            action_from_key(state.mode(), decoded.normalized_key)
        };
        let result = state.apply(action);
        writeln!(
            stdout,
            "TRACE event_seq={} mode_before={:?} focused_window_id={} focused_window_type={} normalized_key={} resolved_action={} cursor_before={} cursor_after={} geometry_ok={} line={}",
            seq,
            result.mode_before,
            state.focused_window_id(),
            state.focused_window_kind(),
            format_key(decoded.normalized_key),
            result.resolved_action,
            result.cursor_before,
            result.cursor_after,
            state.window_geometry_ok(),
            state.line()
        )?;
        stdout.flush()?;
        if result.should_quit {
            break;
        }
    }

    writeln!(
        stdout,
        "FINAL mode={:?} cursor={} focused_window_id={} focused_window_type={} geometry_ok={} line={} window_session={}",
        state.mode(),
        state.cursor(),
        state.focused_window_id(),
        state.focused_window_kind(),
        state.window_geometry_ok(),
        state.line(),
        state.window_session_dump()
    )?;
    stdout.flush()
}

fn action_from_key(mode: Mode, key: Key) -> EditorAction {
    match mode {
        Mode::Normal => match key {
            Key::Char(ch) => EditorAction::NormalModeKey(ch),
            Key::Ctrl('c') => EditorAction::Quit,
            _ => EditorAction::Ignore,
        },
        Mode::Insert => match key {
            Key::Char(ch) => EditorAction::InsertChar(ch),
            Key::Esc => EditorAction::Esc,
            Key::Ctrl('c') => EditorAction::Quit,
            _ => EditorAction::Ignore,
        },
        Mode::TerminalInsert => match key {
            Key::Ctrl('c') => EditorAction::Quit,
            _ => EditorAction::Ignore,
        },
    }
}

fn format_key(key: Key) -> String {
    match key {
        Key::Char(ch) => ch.to_string(),
        Key::Esc => "Esc".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Ctrl(ch) => format!("Ctrl+{ch}"),
        Key::Unknown(byte) => format!("Unknown({byte})"),
    }
}
