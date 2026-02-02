//! Command mode handling for Ex commands.

use crossterm::event::KeyCode;
use kjxlkj_core::Buffer;

/// Result of command execution.
pub enum CommandResult {
    /// Continue normally.
    Continue,
    /// Quit the application.
    Quit,
    /// Show a message.
    Message(String),
}

/// Handles command mode key input.
pub fn handle_command_key(code: KeyCode, command_line: &mut String) -> CommandModeAction {
    match code {
        KeyCode::Esc => {
            command_line.clear();
            CommandModeAction::Exit
        }
        KeyCode::Enter => {
            let result = CommandModeAction::Execute(command_line.clone());
            command_line.clear();
            result
        }
        KeyCode::Backspace => {
            if command_line.is_empty() {
                CommandModeAction::Exit
            } else {
                command_line.pop();
                CommandModeAction::Continue
            }
        }
        KeyCode::Char(c) => {
            command_line.push(c);
            CommandModeAction::Continue
        }
        _ => CommandModeAction::Continue,
    }
}

/// Action to take after command mode input.
#[derive(Debug)]
pub enum CommandModeAction {
    /// Stay in command mode.
    Continue,
    /// Exit to normal mode.
    Exit,
    /// Execute command string.
    Execute(String),
}

/// Executes an Ex command.
pub fn execute_command(cmd: &str, buffer: &mut Buffer) -> CommandResult {
    let cmd = cmd.trim();

    // Handle :help and :help topic
    if cmd == "help" || cmd == "h" {
        return CommandResult::Message(show_help(None));
    }
    if let Some(topic) = cmd.strip_prefix("help ").or_else(|| cmd.strip_prefix("h ")) {
        return CommandResult::Message(show_help(Some(topic.trim())));
    }

    match cmd {
        "w" | "write" => {
            if let Err(e) = buffer.save() {
                CommandResult::Message(format!("Error: {}", e))
            } else {
                CommandResult::Message("File written".to_string())
            }
        }
        "q" | "quit" => {
            if buffer.is_modified() {
                CommandResult::Message("No write since last change (use :q!)".to_string())
            } else {
                CommandResult::Quit
            }
        }
        "q!" => CommandResult::Quit,
        "wq" | "x" => {
            if let Err(e) = buffer.save() {
                CommandResult::Message(format!("Error: {}", e))
            } else {
                CommandResult::Quit
            }
        }
        _ => CommandResult::Message(format!("Unknown command: {}", cmd)),
    }
}

/// Shows built-in help.
fn show_help(topic: Option<&str>) -> String {
    match topic {
        None => HELP_MAIN.to_string(),
        Some("motions") | Some("motion") => HELP_MOTIONS.to_string(),
        Some("operators") | Some("operator") => HELP_OPERATORS.to_string(),
        Some("commands") | Some("command") | Some("ex") => HELP_COMMANDS.to_string(),
        Some(t) => format!(
            "No help for '{}'. Try :help motions, :help operators, :help commands",
            t
        ),
    }
}

const HELP_MAIN: &str = "\
kjxlkj - Modal Text Editor

Basic Usage:
  i      - Enter Insert mode
  Esc    - Return to Normal mode
  :w     - Save file
  :q     - Quit (use :q! to force)
  :wq    - Save and quit

Help Topics:
  :help motions   - Cursor movement
  :help operators - Text operations
  :help commands  - Ex commands
";

const HELP_MOTIONS: &str = "\
Motions:
  h/j/k/l   - Left/Down/Up/Right
  w/b/e     - Word forward/back/end
  0/^/$     - Line start/first-char/end
  gg/G      - File start/end
  f/t/F/T   - Find char forward/to/backward
";

const HELP_OPERATORS: &str = "\
Operators:
  d{motion} - Delete
  c{motion} - Change
  y{motion} - Yank
  >{motion} - Indent right
  <{motion} - Indent left
  gu/gU/g~  - Case operations
  gq        - Format text
";

const HELP_COMMANDS: &str = "\
Ex Commands:
  :w        - Write file
  :q/:q!    - Quit / Force quit
  :wq/:x    - Write and quit
  :e file   - Edit file
  :bn/:bp   - Next/previous buffer
  :bd       - Delete buffer
  :sp/:vs   - Split horizontal/vertical
  :s/a/b/g  - Substitute
  :g/pat/d  - Global command
";
