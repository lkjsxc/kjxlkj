//! Prefix key handlers for normal mode (g, z, [ ]).

use crate::command::{Command, FoldCommand, ScrollCommand};
use crate::key::{Key, KeyEvent};
use kjxlkj_core_types::motion::Motion;

/// Parses a g-prefixed command.
pub fn parse_g_command(event: KeyEvent, count: Option<usize>) -> (Command, bool) {
    if let Key::Char(c) = event.key {
        let cmd = match c {
            'g' => {
                if let Some(n) = count {
                    Command::Motion {
                        count: None,
                        motion: Motion::GoToLine(n.saturating_sub(1)),
                    }
                } else {
                    Command::Motion { count: None, motion: Motion::DocumentStart }
                }
            }
            'j' => Command::Motion { count: None, motion: Motion::Down },
            'k' => Command::Motion { count: None, motion: Motion::Up },
            '0' => Command::Motion { count: None, motion: Motion::FirstColumn },
            '$' => Command::Motion { count: None, motion: Motion::LineEnd },
            '_' => Command::Motion { count: None, motion: Motion::FirstNonBlank },
            'u' => return (Command::Incomplete, true), // Need operator mode
            'U' => return (Command::Incomplete, true),
            '~' => return (Command::Incomplete, true),
            'J' => Command::JoinLines { spaces: false },
            _ => Command::Invalid,
        };
        (cmd, false)
    } else {
        (Command::Invalid, false)
    }
}

/// Parses a z-prefixed command.
pub fn parse_z_command(event: KeyEvent) -> Command {
    if let Key::Char(c) = event.key {
        match c {
            't' => Command::Scroll(ScrollCommand::ToTop),
            'z' => Command::Scroll(ScrollCommand::CenterCursor),
            'b' => Command::Scroll(ScrollCommand::ToBottom),
            'o' => Command::Fold(FoldCommand::Open),
            'c' => Command::Fold(FoldCommand::Close),
            'a' => Command::Fold(FoldCommand::Toggle),
            'f' => Command::Fold(FoldCommand::Create),
            'd' => Command::Fold(FoldCommand::Delete),
            'R' => Command::Fold(FoldCommand::OpenAll),
            'M' => Command::Fold(FoldCommand::CloseAll),
            'H' => Command::Scroll(ScrollCommand::HalfWidth),
            'L' => Command::Scroll(ScrollCommand::HalfWidth),
            _ => Command::Invalid,
        }
    } else {
        Command::Invalid
    }
}

/// Parses a Ctrl-prefixed command.
pub fn parse_ctrl_command(c: char) -> Command {
    match c {
        'r' => Command::Redo,
        'd' => Command::Scroll(ScrollCommand::HalfPageDown),
        'u' => Command::Scroll(ScrollCommand::HalfPageUp),
        'f' => Command::Scroll(ScrollCommand::PageDown),
        'b' => Command::Scroll(ScrollCommand::PageUp),
        'e' => Command::Scroll(ScrollCommand::LineDown),
        'y' => Command::Scroll(ScrollCommand::LineUp),
        'a' => Command::Increment(1),
        'x' => Command::Decrement(1),
        'o' => Command::JumpOlder,
        'i' => Command::JumpNewer,
        'g' => Command::ShowInfo,
        ']' => Command::GoToDefinition,
        't' => Command::GoToTag,
        'w' => Command::WindowPrefix,
        _ => Command::Invalid,
    }
}
