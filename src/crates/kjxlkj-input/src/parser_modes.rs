//! Parser mode handlers.

use crate::{CommandLine, Key, KeyCodeWrapper};
use kjxlkj_core_mode::{Intent, IntentKind, WindowDirection};
use kjxlkj_core_types::Mode;

/// Parses insert mode input.
pub fn parse_insert(key: &Key) -> Option<Intent> {
    if key.is_esc() {
        return Some(Intent::change_mode(Mode::Normal));
    }

    match &key.code {
        KeyCodeWrapper::Char(c) => Some(Intent::new(IntentKind::InsertText {
            text: c.to_string(),
        })),
        KeyCodeWrapper::Enter => Some(Intent::new(IntentKind::InsertNewline)),
        KeyCodeWrapper::Backspace => Some(Intent::new(IntentKind::Backspace)),
        KeyCodeWrapper::Delete => Some(Intent::new(IntentKind::DeleteChar)),
        _ => None,
    }
}

/// Parses window commands after Ctrl-W prefix.
pub fn parse_window_command(key: &Key) -> Option<Intent> {
    match &key.code {
        // Split commands
        KeyCodeWrapper::Char('s') => Some(Intent::new(IntentKind::SplitHorizontal)),
        KeyCodeWrapper::Char('v') => Some(Intent::new(IntentKind::SplitVertical)),

        // Close commands
        KeyCodeWrapper::Char('c') | KeyCodeWrapper::Char('q') => {
            Some(Intent::new(IntentKind::CloseWindow))
        }
        KeyCodeWrapper::Char('o') => Some(Intent::new(IntentKind::OnlyWindow)),

        // Navigation
        KeyCodeWrapper::Char('h') => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Left)))
        }
        KeyCodeWrapper::Char('j') => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Down)))
        }
        KeyCodeWrapper::Char('k') => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Up)))
        }
        KeyCodeWrapper::Char('l') => {
            Some(Intent::new(IntentKind::WindowDirection(
                WindowDirection::Right,
            )))
        }
        KeyCodeWrapper::Left => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Left)))
        }
        KeyCodeWrapper::Down => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Down)))
        }
        KeyCodeWrapper::Up => {
            Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Up)))
        }
        KeyCodeWrapper::Right => {
            Some(Intent::new(IntentKind::WindowDirection(
                WindowDirection::Right,
            )))
        }

        // Cycle windows
        KeyCodeWrapper::Char('w') => Some(Intent::new(IntentKind::NextWindow)),
        KeyCodeWrapper::Char('W') => Some(Intent::new(IntentKind::PrevWindow)),

        _ => None,
    }
}

/// Parses command mode input.
pub fn parse_command(key: &Key, cmdline: &mut CommandLine) -> Option<Intent> {
    if key.is_esc() {
        cmdline.close();
        return Some(Intent::change_mode(Mode::Normal));
    }

    let prompt = cmdline.prompt();

    match &key.code {
        KeyCodeWrapper::Enter => {
            cmdline.add_to_history();
            let input = cmdline.close();
            match prompt {
                '/' => Some(Intent::new(IntentKind::SearchForward { pattern: input })),
                '?' => Some(Intent::new(IntentKind::SearchBackward { pattern: input })),
                _ => Some(Intent::new(IntentKind::ExCommand { command: input })),
            }
        }
        KeyCodeWrapper::Char(c) => {
            cmdline.insert(*c);
            None
        }
        KeyCodeWrapper::Backspace => {
            if !cmdline.backspace() && cmdline.input().is_empty() {
                cmdline.close();
                return Some(Intent::change_mode(Mode::Normal));
            }
            None
        }
        KeyCodeWrapper::Delete => {
            cmdline.delete();
            None
        }
        KeyCodeWrapper::Left => {
            cmdline.move_left();
            None
        }
        KeyCodeWrapper::Right => {
            cmdline.move_right();
            None
        }
        KeyCodeWrapper::Home => {
            cmdline.move_start();
            None
        }
        KeyCodeWrapper::End => {
            cmdline.move_end();
            None
        }
        KeyCodeWrapper::Up => {
            cmdline.history_prev();
            None
        }
        KeyCodeWrapper::Down => {
            cmdline.history_next();
            None
        }
        _ => None,
    }
}
