//! Command-line mode dispatch: editing, history, execution.

use crate::EditorState;
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode};

pub fn handle_cmdline_key(state: &mut EditorState, key: &KeyEvent) -> Intent {
    let cl = &mut state.cmdline;
    match &key.code {
        KeyCode::Escape => {
            cl.text.clear(); cl.cursor = 0;
            cl.history_idx = None; cl.saved_text = None;
            state.message = None;
            Intent::EnterMode(Mode::Normal)
        }
        KeyCode::Enter => {
            let text = cl.text.clone();
            let prefix = cl.prefix;
            if !text.is_empty() { cl.history.push(text.clone()); }
            cl.text.clear(); cl.cursor = 0;
            cl.history_idx = None; cl.saved_text = None;
            match prefix {
                ':' => Intent::ExCommand(format!(":{}", text)),
                '/' => Intent::SearchForward(text),
                '?' => Intent::SearchBackward(text),
                _ => Intent::EnterMode(Mode::Normal),
            }
        }
        KeyCode::Backspace => {
            if cl.cursor > 0 { cl.cursor -= 1; cl.text.remove(cl.cursor); }
            if cl.text.is_empty() { return Intent::EnterMode(Mode::Normal); }
            Intent::Noop
        }
        KeyCode::Delete => {
            if cl.cursor < cl.text.len() { cl.text.remove(cl.cursor); }
            Intent::Noop
        }
        KeyCode::Left => { if cl.cursor > 0 { cl.cursor -= 1; } Intent::Noop }
        KeyCode::Right => { if cl.cursor < cl.text.len() { cl.cursor += 1; } Intent::Noop }
        KeyCode::Home => { cl.cursor = 0; Intent::Noop }
        KeyCode::End => { cl.cursor = cl.text.len(); Intent::Noop }
        KeyCode::Up => { cmdline_history_prev(state); Intent::Noop }
        KeyCode::Down => { cmdline_history_next(state); Intent::Noop }
        KeyCode::Char(c) if key.ctrl => {
            match c {
                'u' => {
                    state.cmdline.text = state.cmdline.text[state.cmdline.cursor..].to_string();
                    state.cmdline.cursor = 0;
                }
                'w' => {
                    let before = &state.cmdline.text[..state.cmdline.cursor];
                    let new_end = before.trim_end()
                        .rfind(|c: char| c.is_whitespace()).map(|i| i + 1).unwrap_or(0);
                    state.cmdline.text = format!("{}{}",
                        &state.cmdline.text[..new_end], &state.cmdline.text[state.cmdline.cursor..]);
                    state.cmdline.cursor = new_end;
                }
                'c' => {
                    state.cmdline.text.clear(); state.cmdline.cursor = 0;
                    return Intent::EnterMode(Mode::Normal);
                }
                _ => {}
            }
            Intent::Noop
        }
        KeyCode::Tab => { cmdline_complete(state); Intent::Noop }
        KeyCode::Char(c) => {
            state.cmdline.text.insert(state.cmdline.cursor, *c);
            state.cmdline.cursor += 1;
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}

fn cmdline_history_prev(state: &mut EditorState) {
    if state.cmdline.history.is_empty() { return; }
    let idx = match state.cmdline.history_idx {
        Some(0) => return,
        Some(i) => i - 1,
        None => {
            state.cmdline.saved_text = Some(state.cmdline.text.clone());
            state.cmdline.history.len() - 1
        }
    };
    state.cmdline.history_idx = Some(idx);
    state.cmdline.text = state.cmdline.history[idx].clone();
    state.cmdline.cursor = state.cmdline.text.len();
}

fn cmdline_history_next(state: &mut EditorState) {
    let idx = match state.cmdline.history_idx { Some(i) => i, None => return };
    if idx + 1 >= state.cmdline.history.len() {
        state.cmdline.history_idx = None;
        if let Some(saved) = state.cmdline.saved_text.take() {
            state.cmdline.text = saved;
        } else {
            state.cmdline.text.clear();
        }
    } else {
        state.cmdline.history_idx = Some(idx + 1);
        state.cmdline.text = state.cmdline.history[idx + 1].clone();
    }
    state.cmdline.cursor = state.cmdline.text.len();
}

fn cmdline_complete(state: &mut EditorState) {
    let text = &state.cmdline.text;
    if state.cmdline.prefix != ':' || text.is_empty() { return; }
    let cmds = [
        "quit", "quit!", "write", "wq", "edit", "split", "vsplit", "close",
        "only", "new", "bnext", "bprev", "bdelete", "set", "nohlsearch",
        "marks", "registers", "jumps", "changes", "sort", "file", "pwd",
        "cd", "messages", "copy", "move", "read", "delete", "yank",
    ];
    let matches: Vec<&&str> = cmds.iter().filter(|c| c.starts_with(text.as_str())).collect();
    if matches.len() == 1 {
        state.cmdline.text = matches[0].to_string();
        state.cmdline.cursor = state.cmdline.text.len();
    } else if !matches.is_empty() {
        let first = matches[0];
        let mut plen = first.len();
        for m in &matches[1..] {
            plen = plen.min(first.chars().zip(m.chars()).take_while(|(a, b)| a == b).count());
        }
        if plen > text.len() {
            state.cmdline.text = first[..plen].to_string();
            state.cmdline.cursor = state.cmdline.text.len();
        }
        let display: Vec<&str> = matches.iter().map(|m| **m).collect();
        state.message = Some(display.join("  "));
    }
}