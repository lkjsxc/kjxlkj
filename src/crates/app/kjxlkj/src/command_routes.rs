use kjxlkj_core_state::EditorAction;

pub fn action_from_command(command: &str) -> EditorAction {
    match command.trim().to_ascii_lowercase().as_str() {
        "explorer" | "explorerreveal" => EditorAction::WindowCommand('E'),
        "explorerclose" => EditorAction::WindowCommand('c'),
        "terminal" => EditorAction::WindowCommand('T'),
        _ => EditorAction::Ignore,
    }
}
