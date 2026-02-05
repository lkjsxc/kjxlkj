//! Key sequence parser for modal editing.

use crate::state::{ModeState, PendingOperator, VisualModeType};
use kjxlkj_core_types::{
    InsertPosition, Intent, KeyCode, KeyEvent, Mode, Motion, Operator, VisualMode,
};

/// Parse result from key handler.
#[derive(Debug, Clone)]
pub enum ParseResult {
    /// Emit intent(s).
    Intent(Vec<Intent>),
    /// Need more input.
    Pending,
    /// Key not handled.
    Unhandled,
}

/// Parse a key event and produce intents.
pub fn parse_key(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    if let Some(_reg) = state.recording_macro {
        state.macro_buffer.push(key.clone());
    }

    match state.mode {
        Mode::Normal => parse_normal(state, key),
        Mode::Insert => parse_insert(state, key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => parse_visual(state, key),
        Mode::Command => parse_command(state, key),
        Mode::Replace => parse_replace(state, key),
    }
}

fn parse_normal(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    if key.modifiers.ctrl() {
        return parse_normal_ctrl(state, key);
    }

    match &key.code {
        KeyCode::Esc => {
            state.reset();
            ParseResult::Intent(vec![Intent::Nop])
        }
        KeyCode::Char(c) => parse_normal_char(state, *c),
        KeyCode::Enter => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Down)]),
        KeyCode::Backspace => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Left)]),
        KeyCode::Left => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Left)]),
        KeyCode::Right => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Right)]),
        KeyCode::Up => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Up)]),
        KeyCode::Down => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Down)]),
        KeyCode::Home => ParseResult::Intent(vec![Intent::MoveCursor(Motion::LineStart)]),
        KeyCode::End => ParseResult::Intent(vec![Intent::MoveCursor(Motion::LineEnd)]),
        KeyCode::PageUp => ParseResult::Intent(vec![Intent::MoveCursor(Motion::PageUp)]),
        KeyCode::PageDown => ParseResult::Intent(vec![Intent::MoveCursor(Motion::PageDown)]),
        _ => ParseResult::Unhandled,
    }
}

fn parse_normal_ctrl(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    match &key.code {
        KeyCode::Char('r') => ParseResult::Intent(vec![Intent::Redo]),
        KeyCode::Char('d') => ParseResult::Intent(vec![Intent::MoveCursor(Motion::HalfPageDown)]),
        KeyCode::Char('u') => ParseResult::Intent(vec![Intent::MoveCursor(Motion::HalfPageUp)]),
        KeyCode::Char('f') => ParseResult::Intent(vec![Intent::MoveCursor(Motion::PageDown)]),
        KeyCode::Char('b') => ParseResult::Intent(vec![Intent::MoveCursor(Motion::PageUp)]),
        KeyCode::Char('w') => parse_window_command(state),
        KeyCode::Char('o') => ParseResult::Intent(vec![Intent::Nop]),
        KeyCode::Char('i') => ParseResult::Intent(vec![Intent::Nop]),
        KeyCode::Char('v') => {
            state.enter_visual(VisualModeType::Block);
            ParseResult::Intent(vec![Intent::StartVisual(VisualMode::Block)])
        }
        _ => ParseResult::Unhandled,
    }
}

fn parse_window_command(_state: &mut ModeState) -> ParseResult {
    ParseResult::Pending
}

fn parse_normal_char(state: &mut ModeState, c: char) -> ParseResult {
    if c.is_ascii_digit() && !(c == '0' && state.count.is_none()) {
        state.append_count(c as u8 - b'0');
        return ParseResult::Pending;
    }

    if state.pending_operator.is_some() {
        return parse_motion_or_text_object(state, c);
    }

    match c {
        'h' => motion_intent(state, Motion::Left),
        'l' => motion_intent(state, Motion::Right),
        'j' => motion_intent(state, Motion::Down),
        'k' => motion_intent(state, Motion::Up),
        '0' => motion_intent(state, Motion::LineStart),
        '^' => motion_intent(state, Motion::FirstNonBlank),
        '$' => motion_intent(state, Motion::LineEnd),
        'w' => motion_intent(state, Motion::WordForward),
        'b' => motion_intent(state, Motion::WordBackward),
        'e' => motion_intent(state, Motion::WordEnd),
        'W' => motion_intent(state, Motion::BigWordForward),
        'B' => motion_intent(state, Motion::BigWordBackward),
        'E' => motion_intent(state, Motion::BigWordEnd),
        'g' => ParseResult::Pending,
        'G' => {
            let line = state.count.unwrap_or(0);
            state.count = None;
            if line == 0 {
                ParseResult::Intent(vec![Intent::MoveCursor(Motion::FileEnd)])
            } else {
                ParseResult::Intent(vec![Intent::MoveCursor(Motion::GoToLine(line))])
            }
        }
        '{' => motion_intent(state, Motion::ParagraphBackward),
        '}' => motion_intent(state, Motion::ParagraphForward),
        '%' => motion_intent(state, Motion::MatchingBracket),
        'i' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::Before)])
        }
        'a' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::After)])
        }
        'I' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::LineStart)])
        }
        'A' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::LineEnd)])
        }
        'o' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::NewLineBelow)])
        }
        'O' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::EnterInsert(InsertPosition::NewLineAbove)])
        }
        'v' => {
            state.enter_visual(VisualModeType::Char);
            ParseResult::Intent(vec![Intent::StartVisual(VisualMode::Char)])
        }
        'V' => {
            state.enter_visual(VisualModeType::Line);
            ParseResult::Intent(vec![Intent::StartVisual(VisualMode::Line)])
        }
        'd' => {
            state.pending_operator = Some(PendingOperator {
                operator: Operator::Delete,
                count: state.count.take(),
            });
            ParseResult::Pending
        }
        'y' => {
            state.pending_operator = Some(PendingOperator {
                operator: Operator::Yank,
                count: state.count.take(),
            });
            ParseResult::Pending
        }
        'c' => {
            state.pending_operator = Some(PendingOperator {
                operator: Operator::Change,
                count: state.count.take(),
            });
            ParseResult::Pending
        }
        '>' => {
            state.pending_operator = Some(PendingOperator {
                operator: Operator::Indent,
                count: state.count.take(),
            });
            ParseResult::Pending
        }
        '<' => {
            state.pending_operator = Some(PendingOperator {
                operator: Operator::Outdent,
                count: state.count.take(),
            });
            ParseResult::Pending
        }
        'x' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::DeleteChar])
        }
        'X' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::Backspace])
        }
        's' => {
            state.enter_insert();
            ParseResult::Intent(vec![Intent::DeleteChar, Intent::ChangeMode(Mode::Insert)])
        }
        'u' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::Undo])
        }
        'p' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::Nop])
        }
        'P' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::Nop])
        }
        '.' => {
            state.count = None;
            ParseResult::Intent(vec![Intent::Repeat])
        }
        '/' => {
            state.enter_command('/');
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Command)])
        }
        '?' => {
            state.enter_command('?');
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Command)])
        }
        ':' => {
            state.enter_command(':');
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Command)])
        }
        'n' => ParseResult::Intent(vec![Intent::NextMatch]),
        'N' => ParseResult::Intent(vec![Intent::PrevMatch]),
        'R' => {
            state.enter_replace();
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Replace)])
        }
        'z' => ParseResult::Pending,
        'm' => ParseResult::Pending,
        '\'' | '`' => ParseResult::Pending,
        'q' => {
            if state.recording_macro.is_some() {
                state.recording_macro = None;
                state.macro_buffer.clear();
                ParseResult::Intent(vec![Intent::StopRecordMacro])
            } else {
                ParseResult::Pending
            }
        }
        '@' => ParseResult::Pending,
        '"' => ParseResult::Pending,
        'f' | 'F' | 't' | 'T' => ParseResult::Pending,
        'r' => ParseResult::Pending,
        ' ' => ParseResult::Pending,
        _ => ParseResult::Unhandled,
    }
}

fn parse_motion_or_text_object(state: &mut ModeState, c: char) -> ParseResult {
    let pending = state.pending_operator.take().unwrap();
    let motion = match c {
        'h' => Some(Motion::Left),
        'l' => Some(Motion::Right),
        'j' => Some(Motion::Down),
        'k' => Some(Motion::Up),
        'w' => Some(Motion::WordForward),
        'b' => Some(Motion::WordBackward),
        'e' => Some(Motion::WordEnd),
        '$' => Some(Motion::LineEnd),
        '0' => Some(Motion::LineStart),
        '^' => Some(Motion::FirstNonBlank),
        'G' => Some(Motion::FileEnd),
        'd' if pending.operator == Operator::Delete => Some(Motion::Down),
        'y' if pending.operator == Operator::Yank => Some(Motion::Down),
        'c' if pending.operator == Operator::Change => Some(Motion::Down),
        '>' if pending.operator == Operator::Indent => Some(Motion::Down),
        '<' if pending.operator == Operator::Outdent => Some(Motion::Down),
        _ => None,
    };

    if let Some(motion) = motion {
        state.count = None;
        ParseResult::Intent(vec![Intent::Operator(pending.operator, motion)])
    } else {
        state.count = None;
        ParseResult::Unhandled
    }
}

fn motion_intent(state: &mut ModeState, motion: Motion) -> ParseResult {
    state.count = None;
    ParseResult::Intent(vec![Intent::MoveCursor(motion)])
}

fn parse_insert(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    match &key.code {
        KeyCode::Esc => {
            state.reset();
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
        }
        KeyCode::Char(c) => {
            if key.modifiers.ctrl() {
                match c {
                    'c' => {
                        state.reset();
                        ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
                    }
                    _ => ParseResult::Unhandled,
                }
            } else {
                ParseResult::Intent(vec![Intent::InsertText(c.to_string())])
            }
        }
        KeyCode::Enter => ParseResult::Intent(vec![Intent::InsertText("\n".to_string())]),
        KeyCode::Backspace => ParseResult::Intent(vec![Intent::Backspace]),
        KeyCode::Delete => ParseResult::Intent(vec![Intent::DeleteChar]),
        KeyCode::Tab => ParseResult::Intent(vec![Intent::InsertText("    ".to_string())]),
        KeyCode::Left => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Left)]),
        KeyCode::Right => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Right)]),
        KeyCode::Up => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Up)]),
        KeyCode::Down => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Down)]),
        KeyCode::Home => ParseResult::Intent(vec![Intent::MoveCursor(Motion::LineStart)]),
        KeyCode::End => ParseResult::Intent(vec![Intent::MoveCursor(Motion::LineEnd)]),
        _ => ParseResult::Unhandled,
    }
}

fn parse_visual(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    match &key.code {
        KeyCode::Esc => {
            state.reset();
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
        }
        KeyCode::Char(c) => match c {
            'h' => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Left)]),
            'l' => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Right)]),
            'j' => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Down)]),
            'k' => ParseResult::Intent(vec![Intent::MoveCursor(Motion::Up)]),
            'd' | 'x' => {
                state.reset();
                ParseResult::Intent(vec![Intent::OperatorSelection(Operator::Delete)])
            }
            'y' => {
                state.reset();
                ParseResult::Intent(vec![Intent::OperatorSelection(Operator::Yank)])
            }
            'c' => {
                state.enter_insert();
                ParseResult::Intent(vec![Intent::OperatorSelection(Operator::Change)])
            }
            'o' => ParseResult::Intent(vec![Intent::Nop]),
            'v' => {
                if state.mode == Mode::Visual {
                    state.reset();
                    ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
                } else {
                    state.mode = Mode::Visual;
                    ParseResult::Intent(vec![Intent::ToggleVisualMode(VisualMode::Char)])
                }
            }
            'V' => {
                if state.mode == Mode::VisualLine {
                    state.reset();
                    ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
                } else {
                    state.mode = Mode::VisualLine;
                    ParseResult::Intent(vec![Intent::ToggleVisualMode(VisualMode::Line)])
                }
            }
            '>' => ParseResult::Intent(vec![Intent::OperatorSelection(Operator::Indent)]),
            '<' => ParseResult::Intent(vec![Intent::OperatorSelection(Operator::Outdent)]),
            _ => ParseResult::Unhandled,
        },
        _ => ParseResult::Unhandled,
    }
}

fn parse_command(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    match &key.code {
        KeyCode::Esc => {
            state.reset();
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
        }
        KeyCode::Enter => {
            let cmd = state.cmdline.clone();
            state.reset();
            if state.search_forward || !state.search_pattern.is_empty() {
                if !cmd.is_empty() {
                    state.search_pattern = cmd.clone();
                }
                if state.search_forward {
                    ParseResult::Intent(vec![Intent::SearchForward(state.search_pattern.clone())])
                } else {
                    ParseResult::Intent(vec![Intent::SearchBackward(state.search_pattern.clone())])
                }
            } else {
                ParseResult::Intent(vec![Intent::ExCommand(cmd)])
            }
        }
        KeyCode::Char(c) => {
            if !key.modifiers.ctrl() {
                state.cmdline.push(*c);
            }
            ParseResult::Pending
        }
        KeyCode::Backspace => {
            state.cmdline.pop();
            if state.cmdline.is_empty() {
                state.reset();
                ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
            } else {
                ParseResult::Pending
            }
        }
        _ => ParseResult::Unhandled,
    }
}

fn parse_replace(state: &mut ModeState, key: &KeyEvent) -> ParseResult {
    match &key.code {
        KeyCode::Esc => {
            state.reset();
            ParseResult::Intent(vec![Intent::ChangeMode(Mode::Normal)])
        }
        KeyCode::Char(c) => {
            if !key.modifiers.ctrl() {
                ParseResult::Intent(vec![Intent::DeleteChar, Intent::InsertText(c.to_string())])
            } else {
                ParseResult::Unhandled
            }
        }
        _ => ParseResult::Unhandled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_insert_char() {
        let mut state = ModeState::new();
        state.enter_insert();
        let result = parse_key(&mut state, &KeyEvent::plain(KeyCode::Char('a')));
        match result {
            ParseResult::Intent(intents) => {
                assert_eq!(intents.len(), 1);
                assert!(matches!(intents[0], Intent::InsertText(_)));
            }
            _ => panic!("Expected intent"),
        }
    }

    #[test]
    fn test_parse_normal_motion() {
        let mut state = ModeState::new();
        let result = parse_key(&mut state, &KeyEvent::plain(KeyCode::Char('j')));
        match result {
            ParseResult::Intent(intents) => {
                assert_eq!(intents.len(), 1);
                assert!(matches!(intents[0], Intent::MoveCursor(Motion::Down)));
            }
            _ => panic!("Expected intent"),
        }
    }

    #[test]
    fn test_parse_escape() {
        let mut state = ModeState::new();
        state.enter_insert();
        let result = parse_key(&mut state, &KeyEvent::plain(KeyCode::Esc));
        assert_eq!(state.mode, Mode::Normal);
        match result {
            ParseResult::Intent(intents) => {
                assert_eq!(intents.len(), 1);
            }
            _ => panic!("Expected intent"),
        }
    }
}
