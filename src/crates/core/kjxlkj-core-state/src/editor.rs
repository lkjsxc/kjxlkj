//! Central editor state. See /docs/spec/architecture/runtime.md.

use std::collections::HashMap;

use kjxlkj_core_mode::{dispatch_key, resolve_mode_transition, PendingState};
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    Action, BufferId, CmdlineState, ContentKind,
    Key, KeyModifiers, Mode, VisualKind, WindowId,
};
use kjxlkj_core_ui::{FocusState, LayoutTree};

use crate::register::RegisterStore;
use crate::search::SearchState;
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
    pub pending: PendingState, // Multi-key pending state for normal mode.
    pub registers: RegisterStore, // Register store for yank/delete/put.
    /// Last text-changing action for dot-repeat.
    pub(crate) last_change: Option<Action>,
    /// Search state for / and ? patterns.
    pub search: SearchState,
    /// Text accumulated during current insert session.
    pub(crate) insert_text: String,
    /// Visual selection anchor (set when entering visual mode).
    pub visual_anchor: Option<kjxlkj_core_edit::Cursor>,
    /// Alternate buffer for Ctrl-^ / # register.
    pub alternate_buffer: Option<BufferId>,
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
        windows.insert(win_id, WindowState::new(win_id, content));
        Self {
            mode: Mode::Normal, buffers, layout, focus, windows,
            terminal_size: (cols, rows),
            cmdline: CmdlineState::default(),
            quit_requested: false, sequence: 0, id_counter: 2,
            pending: PendingState::default(),
            registers: RegisterStore::new(),
            last_change: None, search: SearchState::new(),
            insert_text: String::new(),
            visual_anchor: None, alternate_buffer: None,
        }
    }

    /// Process a key event through the mode dispatch pipeline.
    pub fn handle_key(&mut self, key: &Key, mods: &KeyModifiers) {
        // Command-line modes bypass normal dispatch.
        if let Mode::Command(kind) = self.mode {
            self.handle_command_input(key, mods, kind);
            return;
        }
        let reg = self.pending.register;
        let (action, new_mode) = dispatch_key(self.mode, key, mods, &mut self.pending);
        if let Some(r) = reg { self.registers.selected = Some(r); }
        let resolved = resolve_mode_transition(self.mode, new_mode);
        if let Mode::Command(kind) = resolved {
            if self.mode != resolved { self.activate_cmdline(kind); }
        }
        // Track insert session text for "." register.
        if self.mode == Mode::Insert && resolved == Mode::Normal {
            if !self.insert_text.is_empty() {
                let txt = self.insert_text.clone();
                self.registers.set_readonly('.', txt);
                self.insert_text.clear();
            }
        }
        if resolved == Mode::Insert && self.mode != Mode::Insert {
            self.insert_text.clear();
        }
        // Set visual anchor when entering visual mode.
        if matches!(resolved, Mode::Visual(_)) && !matches!(self.mode, Mode::Visual(_)) {
            let cur = self.windows.get(&self.focus.focused)
                .map(|w| w.cursor)
                .unwrap_or_default();
            self.visual_anchor = Some(cur);
        }
        // Remember if we were in visual mode before apply_action (which may change self.mode).
        let was_visual = matches!(self.mode, Mode::Visual(_));
        // Apply action BEFORE clearing visual anchor (operator needs it).
        if is_text_changing(&action) { self.last_change = Some(action.clone()); }
        self.apply_action(action);
        // Clear anchor when leaving visual mode (after action applied).
        if !matches!(resolved, Mode::Visual(_)) && was_visual {
            self.visual_anchor = None;
        }
        self.mode = resolved;
        self.update_filename_register();
        self.sequence += 1;
    }

    /// Update "%" register with current buffer filename.
    fn update_filename_register(&mut self) {
        let win = match self.windows.get(&self.focus.focused) { Some(w) => w, None => return };
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let name = buf.path.as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| buf.name.clone());
                self.registers.set_readonly('%', name);
            }
        }
    }
}

/// Whether an action changes buffer text (for dot-repeat).
fn is_text_changing(a: &Action) -> bool {
    matches!(a,
        Action::InsertChar(_) | Action::DeleteCharForward | Action::DeleteCharBackward
        | Action::DeleteLine | Action::OperatorLine(_) | Action::OperatorMotion(_, _, _)
        | Action::SubstituteChar | Action::SubstituteLine | Action::ChangeToEnd
        | Action::DeleteToEnd | Action::JoinLines | Action::JoinLinesNoSpace
        | Action::ReplaceChar(_) | Action::ToggleCase | Action::DeleteWordBackward
        | Action::DeleteToLineStart
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::EditorSnapshot;
    fn ed() -> EditorState { EditorState::new(80, 24) }
    fn m() -> KeyModifiers { KeyModifiers::default() }
    #[test]
    fn initial_state() {
        let s = ed();
        assert_eq!(s.mode, Mode::Normal);
        assert_eq!(s.buffers.len(), 1);
        assert!(!s.quit_requested);
    }
    #[test]
    fn insert_and_exit() {
        let mut s = ed();
        s.handle_key(&Key::Char('i'), &m());
        assert_eq!(s.mode, Mode::Insert);
        s.handle_key(&Key::Char('x'), &m());
        s.handle_key(&Key::Escape, &m());
        assert_eq!(s.mode, Mode::Normal);
    }
    #[test]
    fn shift_a_appends_at_eol() {
        let mut s = ed();
        s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "hello").unwrap();
        s.handle_key(&Key::Char('A'), &m());
        assert_eq!(s.mode, Mode::Insert);
        assert_eq!(s.windows.get(&s.focus.focused).unwrap().cursor.col, 5);
    }
    #[test]
    fn snapshot_works() {
        let s = ed();
        let snap = s.snapshot();
        assert_eq!(snap.terminal_size, (80, 24));
    }
    #[test]
    fn split_and_close() {
        let mut s = ed();
        s.apply_action(Action::SplitVertical);
        assert_eq!(s.windows.len(), 2);
        s.apply_action(Action::CloseWindow);
        assert_eq!(s.windows.len(), 1);
    }
    #[test]
    fn insert_text_recorded_to_dot_register() {
        let mut s = ed();
        s.handle_key(&Key::Char('i'), &m());
        s.handle_key(&Key::Char('a'), &m());
        s.handle_key(&Key::Char('b'), &m());
        s.handle_key(&Key::Escape, &m());
        assert_eq!(s.registers.get('.').unwrap().text, "ab");
    }
    #[test]
    fn filename_register_populated() {
        let mut s = ed();
        s.handle_key(&Key::Char('j'), &m());
        assert_eq!(s.registers.get('%').unwrap().text, "[No Name]");
    }
}
