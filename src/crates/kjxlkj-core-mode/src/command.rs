//! Command mode key handling.

use kjxlkj_core_types::{Intent, Key, KeyCode};

use crate::ModeState;

/// Handle a key in command mode.
pub fn handle_command_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    let mut intents = Vec::new();

    match key.code {
        KeyCode::Esc => {
            state.command_line.clear();
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        KeyCode::Enter => {
            let cmd = state.command_line.clone();
            state.command_line.clear();
            intents.push(Intent::ExecuteCommand(cmd));
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        KeyCode::Backspace => {
            if state.command_line.is_empty() {
                intents.push(Intent::ExitToNormal);
                state.exit_to_normal();
            } else {
                state.command_line.pop();
            }
        }

        KeyCode::Char(c) => {
            state.command_line.push(c);
        }

        _ => {}
    }

    intents
}

/// Parse and execute a command string.
pub fn parse_command(cmd: &str) -> Intent {
    let cmd = cmd.trim();

    // Handle empty command
    if cmd.is_empty() {
        return Intent::Noop;
    }

    // Split command and arguments
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let cmd_name = parts[0];
    let args = parts.get(1).map(|s| s.trim()).unwrap_or("");

    match cmd_name {
        // Quit commands
        "q" => Intent::Quit { force: false },
        "q!" => Intent::Quit { force: true },
        "qa" | "qall" => Intent::Quit { force: false },
        "qa!" | "qall!" => Intent::Quit { force: true },

        // Write commands
        "w" => {
            if args.is_empty() {
                Intent::WriteBuffer {
                    path: None,
                    force: false,
                }
            } else {
                Intent::WriteBuffer {
                    path: Some(args.into()),
                    force: false,
                }
            }
        }
        "w!" => Intent::WriteBuffer {
            path: if args.is_empty() {
                None
            } else {
                Some(args.into())
            },
            force: true,
        },
        "wa" | "wall" => Intent::WriteBuffer {
            path: None,
            force: false,
        },

        // Write and quit
        "wq" | "x" => {
            if args.is_empty() {
                Intent::WriteBuffer {
                    path: None,
                    force: false,
                }
            } else {
                Intent::WriteBuffer {
                    path: Some(args.into()),
                    force: false,
                }
            }
        }

        // Edit commands
        "e" => {
            if args.is_empty() {
                Intent::Noop
            } else {
                Intent::OpenFile(args.into())
            }
        }
        "e!" => {
            if args.is_empty() {
                Intent::Noop
            } else {
                Intent::OpenFile(args.into())
            }
        }

        // External commands
        cmd if cmd.starts_with('!') => {
            let _shell_cmd = format!("{}{}", &cmd[1..], if args.is_empty() { "" } else { " " });
            let full_cmd = if args.is_empty() {
                cmd[1..].to_string()
            } else {
                format!("{} {}", &cmd[1..], args)
            };
            Intent::RunExternalCommand(full_cmd)
        }

        _ => Intent::Noop,
    }
}
