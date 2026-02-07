//! Command-line window: editable history buffer for ex-commands.

use kjxlkj_core_types::Direction;
use serde::{Deserialize, Serialize};

/// State of the command-line window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdlineWindowState {
    pub history: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub prompt_char: char,
    pub active: bool,
}

/// Viewport parameters for the command-line window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdlineViewport {
    pub top_line: usize,
    pub visible_lines: usize,
    pub width: usize,
}

/// Open the command-line window with existing history.
pub fn open(history: Vec<String>, prompt: char) -> CmdlineWindowState {
    let cursor_line = history.len(); // on the new empty line
    CmdlineWindowState {
        history,
        cursor_line,
        cursor_col: 0,
        prompt_char: prompt,
        active: true,
    }
}

/// Close the window, returning the current line if it has content.
pub fn close(state: &mut CmdlineWindowState) -> Option<String> {
    state.active = false;
    let line = current_line(state);
    if line.is_empty() {
        None
    } else {
        Some(line)
    }
}

/// Move the cursor within the command-line window.
pub fn move_cursor(state: &mut CmdlineWindowState, direction: Direction) {
    match direction {
        Direction::Backward => {
            if state.cursor_line > 0 {
                state.cursor_line -= 1;
                state.cursor_col = clamp_col(state);
            }
        }
        Direction::Forward => {
            if state.cursor_line < state.history.len() {
                state.cursor_line += 1;
                state.cursor_col = clamp_col(state);
            }
        }
    }
}

/// Insert or handle a character in the current command line.
pub fn edit_line(state: &mut CmdlineWindowState, ch: char) {
    // Ensure we have a line to edit
    while state.cursor_line >= state.history.len() {
        state.history.push(String::new());
    }
    match ch {
        '\x08' | '\x7f' => {
            // Backspace
            let line = &mut state.history[state.cursor_line];
            if state.cursor_col > 0 && !line.is_empty() {
                let col = state.cursor_col.min(line.len());
                line.remove(col - 1);
                state.cursor_col -= 1;
            }
        }
        '\r' | '\n' => {
            // Newline is not inserted; close handled separately
        }
        _ => {
            let line = &mut state.history[state.cursor_line];
            let col = state.cursor_col.min(line.len());
            line.insert(col, ch);
            state.cursor_col += 1;
        }
    }
}

/// Scroll the viewport so the cursor line is visible.
pub fn follow_cmdline_cursor(state: &CmdlineWindowState, viewport: &mut CmdlineViewport) {
    if state.cursor_line < viewport.top_line {
        viewport.top_line = state.cursor_line;
    } else if state.cursor_line >= viewport.top_line + viewport.visible_lines {
        viewport.top_line = state.cursor_line.saturating_sub(viewport.visible_lines - 1);
    }
}

/// Render the command-line window as a list of display strings.
pub fn render_cmdline_window(
    state: &CmdlineWindowState,
    viewport: &CmdlineViewport,
) -> Vec<String> {
    let mut output = Vec::new();
    let end = (viewport.top_line + viewport.visible_lines).min(state.history.len() + 1);
    for i in viewport.top_line..end {
        let content = if i < state.history.len() {
            &state.history[i]
        } else {
            ""
        };
        let prefix = state.prompt_char;
        let mut display = format!("{prefix}{content}");
        display.truncate(viewport.width);
        output.push(display);
    }
    output
}

fn current_line(state: &CmdlineWindowState) -> String {
    if state.cursor_line < state.history.len() {
        state.history[state.cursor_line].clone()
    } else {
        String::new()
    }
}

fn clamp_col(state: &CmdlineWindowState) -> usize {
    let len = if state.cursor_line < state.history.len() {
        state.history[state.cursor_line].len()
    } else {
        0
    };
    state.cursor_col.min(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_close() {
        let mut s = open(vec!["echo hi".into(), "set nu".into()], ':');
        assert!(s.active);
        assert_eq!(s.cursor_line, 2);
        let result = close(&mut s);
        assert!(result.is_none()); // empty new line
    }

    #[test]
    fn edit_and_close() {
        let mut s = open(vec![], ':');
        edit_line(&mut s, 'q');
        edit_line(&mut s, '!');
        let result = close(&mut s);
        assert_eq!(result, Some("q!".into()));
    }

    #[test]
    fn navigate_history() {
        let mut s = open(vec!["first".into(), "second".into()], ':');
        move_cursor(&mut s, Direction::Backward);
        assert_eq!(s.cursor_line, 1);
        move_cursor(&mut s, Direction::Backward);
        assert_eq!(s.cursor_line, 0);
    }

    #[test]
    fn render() {
        let s = open(vec!["echo 1".into(), "echo 2".into()], ':');
        let vp = CmdlineViewport {
            top_line: 0,
            visible_lines: 10,
            width: 40,
        };
        let lines = render_cmdline_window(&s, &vp);
        assert_eq!(lines.len(), 3); // 2 history + 1 new line
        assert!(lines[0].starts_with(':'));
    }
}
