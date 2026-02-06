//! Command-line mode dispatch: editing, history, execution.

use crate::EditorState;
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode};

/// Handle a key event in command-line mode.
/// Returns Some(intent) if the command line produced an action.
pub fn handle_cmdline_key(
    state: &mut EditorState,
    key: &KeyEvent,
) -> Intent {
    match &key.code {
        KeyCode::Escape => {
            state.cmdline.text.clear();
            state.cmdline.cursor = 0;
            state.cmdline.history_idx = None;
            state.cmdline.saved_text = None;
            state.message = None;
            Intent::EnterMode(Mode::Normal)
        }
        KeyCode::Enter => {
            let text = state.cmdline.text.clone();
            let prefix = state.cmdline.prefix;
            // Save to history
            if !text.is_empty() {
                state.cmdline.history.push(text.clone());
            }
            state.cmdline.text.clear();
            state.cmdline.cursor = 0;
            state.cmdline.history_idx = None;
            state.cmdline.saved_text = None;
            // Process the command
            match prefix {
                ':' => {
                    let cmd = format!(":{}", text);
                    Intent::ExCommand(cmd)
                }
                '/' => Intent::SearchForward(text),
                '?' => Intent::SearchBackward(text),
                _ => Intent::EnterMode(Mode::Normal),
            }
        }
        KeyCode::Backspace => {
            if state.cmdline.cursor > 0 {
                state.cmdline.cursor -= 1;
                state.cmdline.text.remove(state.cmdline.cursor);
            }
            if state.cmdline.text.is_empty() {
                return Intent::EnterMode(Mode::Normal);
            }
            Intent::Noop
        }
        KeyCode::Delete => {
            if state.cmdline.cursor < state.cmdline.text.len() {
                state.cmdline.text.remove(state.cmdline.cursor);
            }
            Intent::Noop
        }
        KeyCode::Left => {
            if state.cmdline.cursor > 0 {
                state.cmdline.cursor -= 1;
            }
            Intent::Noop
        }
        KeyCode::Right => {
            if state.cmdline.cursor < state.cmdline.text.len() {
                state.cmdline.cursor += 1;
            }
            Intent::Noop
        }
        KeyCode::Home => {
            state.cmdline.cursor = 0;
            Intent::Noop
        }
        KeyCode::End => {
            state.cmdline.cursor = state.cmdline.text.len();
            Intent::Noop
        }
        KeyCode::Up => {
            cmdline_history_prev(state);
            Intent::Noop
        }
        KeyCode::Down => {
            cmdline_history_next(state);
            Intent::Noop
        }
        KeyCode::Char(c) if key.ctrl => {
            match c {
                'u' => {
                    // Delete to start of line
                    state.cmdline.text =
                        state.cmdline.text[state.cmdline.cursor..]
                            .to_string();
                    state.cmdline.cursor = 0;
                }
                'w' => {
                    // Delete word before cursor
                    let before =
                        &state.cmdline.text[..state.cmdline.cursor];
                    let trimmed = before.trim_end();
                    let new_end = trimmed
                        .rfind(|c: char| c.is_whitespace())
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    state.cmdline.text = format!(
                        "{}{}",
                        &state.cmdline.text[..new_end],
                        &state.cmdline.text[state.cmdline.cursor..]
                    );
                    state.cmdline.cursor = new_end;
                }
                'c' => {
                    // Cancel like Escape
                    state.cmdline.text.clear();
                    state.cmdline.cursor = 0;
                    return Intent::EnterMode(Mode::Normal);
                }
                _ => {}
            }
            Intent::Noop
        }
        KeyCode::Tab => {
            // Basic tab completion
            cmdline_complete(state);
            Intent::Noop
        }
        KeyCode::Char(c) => {
            state
                .cmdline
                .text
                .insert(state.cmdline.cursor, *c);
            state.cmdline.cursor += 1;
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}

/// Navigate to previous history entry.
fn cmdline_history_prev(state: &mut EditorState) {
    if state.cmdline.history.is_empty() {
        return;
    }
    let idx = match state.cmdline.history_idx {
        Some(0) => return,
        Some(i) => i - 1,
        None => {
            state.cmdline.saved_text =
                Some(state.cmdline.text.clone());
            state.cmdline.history.len() - 1
        }
    };
    state.cmdline.history_idx = Some(idx);
    state.cmdline.text = state.cmdline.history[idx].clone();
    state.cmdline.cursor = state.cmdline.text.len();
}

/// Navigate to next history entry.
fn cmdline_history_next(state: &mut EditorState) {
    let idx = match state.cmdline.history_idx {
        Some(i) => i,
        None => return,
    };
    if idx + 1 >= state.cmdline.history.len() {
        // Restore saved text
        state.cmdline.history_idx = None;
        if let Some(saved) = state.cmdline.saved_text.take() {
            state.cmdline.text = saved;
        } else {
            state.cmdline.text.clear();
        }
    } else {
        state.cmdline.history_idx = Some(idx + 1);
        state.cmdline.text =
            state.cmdline.history[idx + 1].clone();
    }
    state.cmdline.cursor = state.cmdline.text.len();
}

/// Basic command-line tab completion.
fn cmdline_complete(state: &mut EditorState) {
    let text = &state.cmdline.text;
    if state.cmdline.prefix != ':' || text.is_empty() {
        return;
    }
    let cmds = [
        "quit", "quit!", "write", "wq", "edit", "split",
        "vsplit", "close", "only", "new", "bnext", "bprev",
        "bdelete", "set", "nohlsearch", "marks", "registers",
        "jumps", "changes", "sort", "file", "pwd", "cd",
        "messages",
    ];
    let matches: Vec<&&str> = cmds
        .iter()
        .filter(|c| c.starts_with(text.as_str()))
        .collect();
    if matches.len() == 1 {
        state.cmdline.text = matches[0].to_string();
        state.cmdline.cursor = state.cmdline.text.len();
    } else if !matches.is_empty() {
        // Find longest common prefix
        let first = matches[0];
        let mut prefix_len = first.len();
        for m in &matches[1..] {
            let common = first
                .chars()
                .zip(m.chars())
                .take_while(|(a, b)| a == b)
                .count();
            prefix_len = prefix_len.min(common);
        }
        if prefix_len > text.len() {
            state.cmdline.text =
                first[..prefix_len].to_string();
            state.cmdline.cursor = state.cmdline.text.len();
        }
        let display: Vec<&str> =
            matches.iter().map(|m| **m).collect();
        state.message =
            Some(display.join("  "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;
    use kjxlkj_core_types::Size;

    fn setup() -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text("hello");
        s.create_window(bid);
        s.mode.transition(Mode::Command);
        s.cmdline.prefix = ':';
        s
    }

    #[test]
    fn type_char() {
        let mut s = setup();
        let r = handle_cmdline_key(
            &mut s,
            &KeyEvent::char('q'),
        );
        assert_eq!(r, Intent::Noop);
        assert_eq!(s.cmdline.text, "q");
        assert_eq!(s.cmdline.cursor, 1);
    }

    #[test]
    fn enter_executes() {
        let mut s = setup();
        handle_cmdline_key(&mut s, &KeyEvent::char('q'));
        let r = handle_cmdline_key(
            &mut s,
            &KeyEvent::special(KeyCode::Enter),
        );
        assert_eq!(r, Intent::ExCommand(":q".into()));
    }

    #[test]
    fn escape_cancels() {
        let mut s = setup();
        handle_cmdline_key(&mut s, &KeyEvent::char('q'));
        let r = handle_cmdline_key(
            &mut s,
            &KeyEvent::special(KeyCode::Escape),
        );
        assert_eq!(r, Intent::EnterMode(Mode::Normal));
        assert!(s.cmdline.text.is_empty());
    }

    #[test]
    fn backspace_deletes() {
        let mut s = setup();
        handle_cmdline_key(&mut s, &KeyEvent::char('a'));
        handle_cmdline_key(&mut s, &KeyEvent::char('b'));
        handle_cmdline_key(
            &mut s,
            &KeyEvent::special(KeyCode::Backspace),
        );
        assert_eq!(s.cmdline.text, "a");
    }

    #[test]
    fn history_navigation() {
        let mut s = setup();
        s.cmdline.history =
            vec!["first".into(), "second".into()];
        cmdline_history_prev(&mut s);
        assert_eq!(s.cmdline.text, "second");
        cmdline_history_prev(&mut s);
        assert_eq!(s.cmdline.text, "first");
        cmdline_history_next(&mut s);
        assert_eq!(s.cmdline.text, "second");
    }

    #[test]
    fn tab_completion() {
        let mut s = setup();
        s.cmdline.text = "wr".into();
        s.cmdline.cursor = 2;
        cmdline_complete(&mut s);
        assert_eq!(s.cmdline.text, "write");
    }

    #[test]
    fn search_forward() {
        let mut s = setup();
        s.cmdline.prefix = '/';
        handle_cmdline_key(&mut s, &KeyEvent::char('h'));
        handle_cmdline_key(&mut s, &KeyEvent::char('i'));
        let r = handle_cmdline_key(
            &mut s,
            &KeyEvent::special(KeyCode::Enter),
        );
        assert_eq!(r, Intent::SearchForward("hi".into()));
    }

    #[test]
    fn search_backward() {
        let mut s = setup();
        s.cmdline.prefix = '?';
        handle_cmdline_key(&mut s, &KeyEvent::char('x'));
        let r = handle_cmdline_key(
            &mut s,
            &KeyEvent::special(KeyCode::Enter),
        );
        assert_eq!(r, Intent::SearchBackward("x".into()));
    }
}
