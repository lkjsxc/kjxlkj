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
    let mut state = EditorState::new(initial_line, start_cursor);
    let mut seq: u64 = 0;
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let mut one = [0_u8; 1];
        if stdin.read_exact(&mut one).is_err() {
            break;
        }
        seq += 1;
        let decoded = decode_byte(one[0]);
        let action = action_from_key(state.mode(), decoded.normalized_key);
        let result = state.apply(action);
        writeln!(
            stdout,
            "TRACE event_seq={} mode_before={:?} focused_window_id=1 normalized_key={} resolved_action={} cursor_before={} cursor_after={} line={}",
            seq,
            result.mode_before,
            format_key(decoded.normalized_key),
            result.resolved_action,
            result.cursor_before,
            result.cursor_after,
            state.line()
        )?;
        stdout.flush()?;
        if result.should_quit {
            break;
        }
    }

    writeln!(
        stdout,
        "FINAL mode={:?} cursor={} line={}",
        state.mode(),
        state.cursor(),
        state.line()
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
