use kjxlkj_core_mode::Mode;
use kjxlkj_core_state::EditorAction;
use kjxlkj_input::Key;

pub fn action_from_key(mode: Mode, key: Key, focused_window_kind: &str) -> EditorAction {
    match mode {
        Mode::Normal => match key {
            Key::Enter if focused_window_kind == "Explorer" => EditorAction::WindowCommand('O'),
            Key::Char('v') if focused_window_kind == "Explorer" => EditorAction::WindowCommand('V'),
            Key::Char('s') if focused_window_kind == "Explorer" => EditorAction::WindowCommand('S'),
            Key::Char('q') if focused_window_kind == "Explorer" => EditorAction::WindowCommand('c'),
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

pub fn format_key(key: Key) -> String {
    match key {
        Key::Char(ch) => ch.to_string(),
        Key::Esc => "Esc".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Ctrl(ch) => format!("Ctrl+{ch}"),
        Key::Unknown(byte) => format!("Unknown({byte})"),
    }
}
