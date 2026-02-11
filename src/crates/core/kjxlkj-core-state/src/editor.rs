//! Central editor state owned by the core task.
//! See /docs/spec/architecture/runtime.md and
//! /docs/spec/editor/README.md.

use std::collections::HashMap;

use kjxlkj_core_mode::{dispatch_key, resolve_mode_transition, PendingState};
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    Action, BufferId, CmdlineState, ContentKind,
    Key, KeyModifiers, Mode, WindowId,
};
use kjxlkj_core_ui::{FocusState, LayoutTree};

use crate::register::RegisterStore;
use crate::window_state::WindowState;

/// The single mutable editor state.
pub struct EditorState {
    pub mode: Mode,
    pub buffers: HashMap<BufferId, Buffer>,
    pub layout: LayoutTree,
    pub focus: FocusState,
    pub windows: HashMap<WindowId, WindowState>,
    pub terminal_size: (u16, u16),
    pub cmdline: CmdlineState,
    pub quit_requested: bool,
    pub sequence: u64,
    pub(crate) id_counter: u64,
    /// Multi-key pending state for normal mode.
    pub pending: PendingState,
    /// Register store for yank/delete/put.
    pub registers: RegisterStore,
    /// Last text-changing action for dot-repeat.
    pub(crate) last_change: Option<Action>,
}

impl EditorState {
    /// Create initial editor state with one scratch buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(0);
        let win_id = WindowId(1);
        let buf = Buffer::new_scratch(buf_id);
        let content = ContentKind::Buffer(buf_id);

        let mut buffers = HashMap::new();
        buffers.insert(buf_id, buf);

        let layout = LayoutTree::single(win_id, content);
        let focus = FocusState::new(win_id);

        let mut windows = HashMap::new();
        windows.insert(
            win_id,
            WindowState::new(win_id, content),
        );

        Self {
            mode: Mode::Normal,
            buffers,
            layout,
            focus,
            windows,
            terminal_size: (cols, rows),
            cmdline: CmdlineState::default(),
            quit_requested: false,
            sequence: 0,
            id_counter: 2,
            pending: PendingState::default(),
            registers: RegisterStore::new(),
            last_change: None,
        }
    }

    /// Process a key event through the mode dispatch pipeline.
    pub fn handle_key(
        &mut self,
        key: &Key,
        mods: &KeyModifiers,
    ) {
        // Capture register selection before dispatch
        // may clear it.
        let reg = self.pending.register;
        let (action, new_mode) =
            dispatch_key(
                self.mode,
                key,
                mods,
                &mut self.pending,
            );
        // Transfer register from pending to register
        // store selection for the upcoming action.
        if let Some(r) = reg {
            self.registers.selected = Some(r);
        }
        self.mode =
            resolve_mode_transition(self.mode, new_mode);
        // Record text-changing actions for dot-repeat.
        if is_text_changing(&action) {
            self.last_change = Some(action.clone());
        }
        self.apply_action(action);
        self.sequence += 1;
    }
}

/// Whether an action changes buffer text (for dot-repeat).
fn is_text_changing(action: &Action) -> bool {
    matches!(
        action,
        Action::InsertChar(_)
            | Action::DeleteCharForward
            | Action::DeleteCharBackward
            | Action::DeleteLine
            | Action::OperatorLine(_)
            | Action::OperatorMotion(_, _, _)
            | Action::SubstituteChar
            | Action::SubstituteLine
            | Action::ChangeToEnd
            | Action::DeleteToEnd
            | Action::JoinLines
            | Action::JoinLinesNoSpace
            | Action::ReplaceChar(_)
            | Action::ToggleCase
            | Action::DeleteWordBackward
            | Action::DeleteToLineStart
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::EditorSnapshot;

    #[test]
    fn initial_state() {
        let s = EditorState::new(80, 24);
        assert_eq!(s.mode, Mode::Normal);
        assert_eq!(s.buffers.len(), 1);
        assert_eq!(s.windows.len(), 1);
        assert!(!s.quit_requested);
    }

    #[test]
    fn insert_and_exit() {
        let mut s = EditorState::new(80, 24);
        s.handle_key(
            &Key::Char('i'),
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Insert);
        s.handle_key(
            &Key::Char('x'),
            &KeyModifiers::default(),
        );
        s.handle_key(
            &Key::Escape,
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Normal);
    }

    #[test]
    fn shift_a_appends_at_eol() {
        let mut s = EditorState::new(80, 24);
        let buf_id = BufferId(0);
        s.buffers
            .get_mut(&buf_id)
            .unwrap()
            .insert(0, 0, "hello")
            .unwrap();
        s.handle_key(
            &Key::Char('A'),
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Insert);
        let win =
            s.windows.get(&s.focus.focused).unwrap();
        assert_eq!(win.cursor.col, 5);
    }

    #[test]
    fn snapshot_works() {
        let s = EditorState::new(80, 24);
        let snap = s.snapshot();
        assert_eq!(snap.terminal_size, (80, 24));
        assert_eq!(snap.window_contents.len(), 1);
    }

    #[test]
    fn split_and_close() {
        let mut s = EditorState::new(80, 24);
        s.apply_action(Action::SplitVertical);
        assert_eq!(s.windows.len(), 2);
        s.apply_action(Action::CloseWindow);
        assert_eq!(s.windows.len(), 1);
    }
}
