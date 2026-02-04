//! Headless mode for scripted testing.

use anyhow::Result;
use kjxlkj_core::{BufferName, EditorState, Key, KeyCode};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

/// Headless script command.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HeadlessCommand {
    /// Open a buffer with content.
    #[serde(rename = "open")]
    Open { name: String, content: String },

    /// Send a key.
    #[serde(rename = "key")]
    Key { key: String },

    /// Send multiple keys.
    #[serde(rename = "keys")]
    Keys { keys: String },

    /// Get current buffer content.
    #[serde(rename = "get_content")]
    GetContent,

    /// Get cursor position.
    #[serde(rename = "get_cursor")]
    GetCursor,

    /// Get current mode.
    #[serde(rename = "get_mode")]
    GetMode,

    /// Quit.
    #[serde(rename = "quit")]
    Quit,
}

/// Headless response.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadlessResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl HeadlessResponse {
    fn ok(data: impl Serialize) -> Self {
        Self {
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
            error: None,
        }
    }

    fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }

    fn empty() -> Self {
        Self {
            success: true,
            data: None,
            error: None,
        }
    }
}

/// Run in headless mode.
pub fn run_headless(_args: &[String]) -> Result<()> {
    let mut editor = EditorState::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<HeadlessCommand>(&line) {
            Ok(cmd) => process_command(&mut editor, cmd),
            Err(e) => HeadlessResponse::error(format!("Parse error: {}", e)),
        };

        let output = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", output)?;
        stdout.flush()?;

        if editor.should_quit {
            break;
        }
    }

    Ok(())
}

/// Process a headless command.
fn process_command(editor: &mut EditorState, cmd: HeadlessCommand) -> HeadlessResponse {
    match cmd {
        HeadlessCommand::Open { name, content } => {
            editor.open_buffer(BufferName::new(&name), &content);
            HeadlessResponse::empty()
        }

        HeadlessCommand::Key { key } => {
            if let Some(k) = parse_key(&key) {
                let _intent = editor.mode_state.process_key(k);
                // Note: In a real implementation, we'd apply the intent
                HeadlessResponse::empty()
            } else {
                HeadlessResponse::error(format!("Unknown key: {}", key))
            }
        }

        HeadlessCommand::Keys { keys } => {
            for ch in keys.chars() {
                let k = Key::char(ch);
                editor.mode_state.process_key(k);
            }
            HeadlessResponse::empty()
        }

        HeadlessCommand::GetContent => {
            let content = editor.active_buffer().text.to_string();
            HeadlessResponse::ok(content)
        }

        HeadlessCommand::GetCursor => {
            let cursor = editor.active_buffer().cursor;
            HeadlessResponse::ok(serde_json::json!({
                "line": cursor.position.line,
                "col": cursor.position.col
            }))
        }

        HeadlessCommand::GetMode => {
            HeadlessResponse::ok(editor.mode().name())
        }

        HeadlessCommand::Quit => {
            editor.should_quit = true;
            HeadlessResponse::empty()
        }
    }
}

/// Parse a key string.
fn parse_key(s: &str) -> Option<Key> {
    match s {
        "<Esc>" | "<Escape>" => Some(Key::escape()),
        "<Enter>" | "<CR>" => Some(Key::enter()),
        "<Tab>" => Some(Key::new(KeyCode::Tab)),
        "<BS>" | "<Backspace>" => Some(Key::new(KeyCode::Backspace)),
        "<Del>" | "<Delete>" => Some(Key::new(KeyCode::Delete)),
        "<Up>" => Some(Key::new(KeyCode::Up)),
        "<Down>" => Some(Key::new(KeyCode::Down)),
        "<Left>" => Some(Key::new(KeyCode::Left)),
        "<Right>" => Some(Key::new(KeyCode::Right)),
        s if s.starts_with("<C-") && s.ends_with('>') => {
            let ch = s.chars().nth(3)?;
            Some(Key::ctrl(ch))
        }
        s if s.len() == 1 => {
            Some(Key::char(s.chars().next()?))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_keys() {
        assert!(parse_key("a").is_some());
        assert!(parse_key("<Esc>").is_some());
        assert!(parse_key("<C-c>").is_some());
    }

    #[test]
    fn headless_response_serialization() {
        let resp = HeadlessResponse::ok("test");
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("success"));
    }
}
