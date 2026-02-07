//! Core action processing loop.

use kjxlkj_core_state::snapshot_producer::produce_snapshot;
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Direction, EditorAction, KeyEvent, Mode, Position};
use kjxlkj_core_ui::snapshot::EditorSnapshot;

use crate::command_mode::handle_command_key;
use crate::insert_mode::handle_insert_key;
use crate::normal_mode::handle_normal_key;
use crate::replace_mode::handle_replace_key;
use crate::visual_mode::handle_visual_key;

/// The core processor wrapping editor state and dispatch logic.
pub struct CoreProcessor {
    state: EditorState,
}

impl CoreProcessor {
    /// Create a new processor with default initial state.
    pub fn new() -> Self {
        Self {
            state: EditorState::new(),
        }
    }

    /// Dispatch an `EditorAction` to the appropriate handler.
    pub fn process_action(&mut self, action: EditorAction) {
        dispatch_action(&mut self.state, action);
    }

    /// Interpret a key event in the current mode context and dispatch it.
    pub fn process_key(&mut self, key: KeyEvent) {
        if let Some(action) = self.interpret_key(key) {
            self.process_action(action);
        }
    }

    /// Produce an immutable snapshot for the renderer.
    pub fn snapshot(&self) -> EditorSnapshot {
        produce_snapshot(&self.state)
    }

    /// Whether the editor session should quit.
    pub fn is_quit(&self) -> bool {
        self.state.should_quit
    }

    /// Borrow the editor state.
    pub fn state(&self) -> &EditorState {
        &self.state
    }

    /// Mutably borrow the editor state.
    pub fn state_mut(&mut self) -> &mut EditorState {
        &mut self.state
    }

    /// Resize the terminal dimensions.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.state.terminal_size = (width, height);
        for win in &mut self.state.windows {
            win.width = width;
            win.height = height.saturating_sub(2); // status + cmdline
        }
    }

    /// Map key to action based on current mode.
    fn interpret_key(&mut self, key: KeyEvent) -> Option<EditorAction> {
        match self.state.mode.current() {
            Mode::Normal => handle_normal_key(&mut self.state, key),
            Mode::Insert => handle_insert_key(&mut self.state, key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                handle_visual_key(&mut self.state, key)
            }
            Mode::Command => handle_command_key(&mut self.state, key),
            Mode::Replace => handle_replace_key(&mut self.state, key),
            _ => None,
        }
    }
}

impl Default for CoreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply an EditorAction to editor state.
fn dispatch_action(state: &mut EditorState, action: EditorAction) {
    match action {
        EditorAction::CursorMove(dir, n) => move_cursor(state, dir, n),
        EditorAction::CursorTo(pos) => {
            state.active_window_mut().cursor = pos;
            clamp_and_follow(state);
        }
        EditorAction::InsertChar(ch) => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, ch);
            state.active_window_mut().cursor.col += 1;
        }
        EditorAction::InsertNewline => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, '\n');
            state.active_window_mut().cursor.line += 1;
            state.active_window_mut().cursor.col = 0;
        }
        EditorAction::DeleteChar => {
            let pos = state.active_window().cursor;
            let end = Position::new(pos.line, pos.col + 1);
            state.active_buffer_mut().delete_range(pos, end);
        }
        EditorAction::DeleteBack => {
            let pos = state.active_window().cursor;
            if pos.col > 0 {
                let start = Position::new(pos.line, pos.col - 1);
                state.active_buffer_mut().delete_range(start, pos);
                state.active_window_mut().cursor.col -= 1;
            }
        }
        EditorAction::ChangeMode(m) => state.mode.transition(m),
        EditorAction::Quit => state.should_quit = true,
        EditorAction::ForceQuit => state.should_quit = true,
        EditorAction::Undo | EditorAction::Redo => { /* delegated to undo system */ }
        EditorAction::Scroll(delta) => {
            let max = state.active_buffer().line_count().saturating_sub(1);
            kjxlkj_core_state::viewport::scroll(&mut state.viewport, delta, max);
        }
        EditorAction::Noop => {}
        _ => { /* remaining actions handled inline by mode handlers */ }
    }
    clamp_and_follow(state);
}

fn move_cursor(state: &mut EditorState, dir: Direction, n: usize) {
    let win = state.active_window_mut();
    match dir {
        Direction::Forward => win.cursor.col = win.cursor.col.saturating_add(n),
        Direction::Backward => win.cursor.col = win.cursor.col.saturating_sub(n),
    }
}

fn clamp_and_follow(state: &mut EditorState) {
    let max_line = state.active_buffer().line_count().saturating_sub(1);
    state.active_window_mut().cursor.line = state.active_window().cursor.line.min(max_line);
    let cur_line = state.active_window().cursor.line;
    let line_len = state.active_buffer().line_len(cur_line);
    let max_col = if line_len > 0 {
        line_len.saturating_sub(1)
    } else {
        0
    };
    state.active_window_mut().cursor.col = state.active_window().cursor.col.min(max_col);
    state.active_window_mut().ensure_cursor_visible();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_processor() {
        let p = CoreProcessor::new();
        assert!(!p.is_quit());
        assert_eq!(p.state().mode.current(), Mode::Normal);
    }

    #[test]
    fn resize() {
        let mut p = CoreProcessor::new();
        p.resize(120, 40);
        assert_eq!(p.state().terminal_size, (120, 40));
    }

    #[test]
    fn snapshot_produces() {
        let p = CoreProcessor::new();
        let snap = p.snapshot();
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn process_quit() {
        let mut p = CoreProcessor::new();
        p.process_action(EditorAction::Quit);
        assert!(p.is_quit());
    }

    #[test]
    fn insert_char_action() {
        let mut p = CoreProcessor::new();
        p.process_action(EditorAction::InsertChar('a'));
        assert_eq!(p.state().active_buffer().line(0), Some("a".to_string()));
    }

    #[test]
    fn default_impl() {
        let p = CoreProcessor::default();
        assert!(!p.is_quit());
    }
}
