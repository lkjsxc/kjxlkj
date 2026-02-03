use kjxlkj_core_types::{Key, KeyCode, KeyMods, Mode};
use crate::Effect;
use super::{EditorState, PendingNormal};

impl EditorState {
    pub(super) fn handle_key(&mut self, key: Key) -> Vec<Effect> {
        match self.mode() {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
            Mode::Replace => self.handle_key_insert(key),
            Mode::Command => self.handle_key_command(key),
            Mode::Visual => self.handle_key_visual(key),
        }
    }

    fn handle_key_normal(&mut self, key: Key) -> Vec<Effect> {
        let plain = KeyMods::default();
        if let Some(p) = self.take_pending_normal() {
            match p {
                PendingNormal::Delete if key == Key { code: KeyCode::Char('d'), mods: plain } => {
                    self.delete_current_line();
                    return Vec::new();
                }
                PendingNormal::Yank if key == Key { code: KeyCode::Char('y'), mods: plain } => {
                    self.yank_current_line();
                    return Vec::new();
                }
                _ => {}
            }
        }

        match key {
            Key { code: KeyCode::Esc, .. } => Vec::new(),
            Key { code: KeyCode::Char('i'), mods } if mods == plain => { self.set_mode(Mode::Insert); Vec::new() }
            Key { code: KeyCode::Char('a'), mods } if mods == plain => {
                self.move_right_in_line();
                self.set_mode(Mode::Insert);
                Vec::new()
            }
            Key { code: KeyCode::Char('A'), mods } if !mods.ctrl && !mods.alt => {
                self.move_to_line_end();
                self.set_mode(Mode::Insert);
                Vec::new()
            }
            Key { code: KeyCode::Char('o'), mods } if mods == plain => {
                self.open_line_below();
                self.set_mode(Mode::Insert);
                Vec::new()
            }
            Key { code: KeyCode::Char('R'), mods } if !mods.ctrl && !mods.alt => {
                self.set_mode(Mode::Replace);
                Vec::new()
            }
            Key { code: KeyCode::Char('v'), mods } if mods == plain => {
                if let Some(c) = self.active_window_cursor() {
                    self.set_visual_anchor(c);
                }
                self.set_mode(Mode::Visual);
                Vec::new()
            }
            Key { code: KeyCode::Char(':'), mods } if mods == plain => { self.set_mode(Mode::Command); Vec::new() }
            Key { code: KeyCode::Char('u'), mods } if mods == plain => { self.undo(); Vec::new() }
            Key { code: KeyCode::Char('r'), mods } if mods.ctrl && !mods.alt => { self.redo(); Vec::new() }
            Key { code: KeyCode::Char('x'), mods } if mods == plain => { self.delete_char_under_cursor(); Vec::new() }
            Key { code: KeyCode::Char('p'), mods } if mods == plain => { self.paste_after(); Vec::new() }
            Key { code: KeyCode::Char('d'), mods } if mods == plain => { self.set_pending_normal(PendingNormal::Delete); Vec::new() }
            Key { code: KeyCode::Char('y'), mods } if mods == plain => { self.set_pending_normal(PendingNormal::Yank); Vec::new() }
            Key { code: KeyCode::Char('h'), mods } if mods == plain => { self.move_left(); Vec::new() }
            Key { code: KeyCode::Char('j'), mods } if mods == plain => { self.move_down(); Vec::new() }
            Key { code: KeyCode::Char('k'), mods } if mods == plain => { self.move_up(); Vec::new() }
            Key { code: KeyCode::Char('l'), mods } if mods == plain => { self.move_right(); Vec::new() }
            Key { code: KeyCode::Left, .. } => { self.move_left(); Vec::new() }
            Key { code: KeyCode::Right, .. } => { self.move_right(); Vec::new() }
            Key { code: KeyCode::Up, .. } => { self.move_up(); Vec::new() }
            Key { code: KeyCode::Down, .. } => { self.move_down(); Vec::new() }
            _ => Vec::new(),
        }
    }

    fn handle_key_insert(&mut self, key: Key) -> Vec<Effect> {
        match key {
            Key { code: KeyCode::Esc, .. } => { self.set_mode(Mode::Normal); Vec::new() }
            Key { code: KeyCode::Backspace, .. } => { self.backspace(); Vec::new() }
            Key { code: KeyCode::Enter, .. } => { self.insert_text("\n"); Vec::new() }
            Key { code: KeyCode::Tab, .. } => { self.insert_text("\t"); Vec::new() }
            Key { code: KeyCode::Char(c), mods } if !mods.ctrl && !mods.alt => { self.insert_text(&c.to_string()); Vec::new() }
            Key { code: KeyCode::Left, .. } => { self.move_left(); Vec::new() }
            Key { code: KeyCode::Right, .. } => { self.move_right(); Vec::new() }
            Key { code: KeyCode::Up, .. } => { self.move_up(); Vec::new() }
            Key { code: KeyCode::Down, .. } => { self.move_down(); Vec::new() }
            _ => Vec::new(),
        }
    }

    fn handle_key_command(&mut self, key: Key) -> Vec<Effect> {
        match key {
            Key { code: KeyCode::Esc, .. } => { self.set_mode(Mode::Normal); Vec::new() }
            Key { code: KeyCode::Backspace, .. } => { self.cmdline_backspace(); Vec::new() }
            Key { code: KeyCode::Enter, .. } => {
                let effects = self.cmdline_submit();
                self.set_mode(Mode::Normal);
                effects
            }
            Key { code: KeyCode::Char(c), mods } if !mods.ctrl && !mods.alt => { self.cmdline_push(c); Vec::new() }
            _ => Vec::new(),
        }
    }

    fn handle_key_visual(&mut self, key: Key) -> Vec<Effect> {
        let plain = KeyMods::default();
        match key {
            Key { code: KeyCode::Esc, .. } => { self.set_mode(Mode::Normal); Vec::new() }
            Key { code: KeyCode::Char('d'), mods } if mods == plain => {
                self.delete_visual_selection();
                self.set_mode(Mode::Normal);
                Vec::new()
            }
            Key { code: KeyCode::Char('y'), mods } if mods == plain => {
                self.yank_visual_selection();
                self.set_mode(Mode::Normal);
                Vec::new()
            }
            Key { code: KeyCode::Char('h'), mods } if mods == plain => { self.move_left(); Vec::new() }
            Key { code: KeyCode::Char('j'), mods } if mods == plain => { self.move_down(); Vec::new() }
            Key { code: KeyCode::Char('k'), mods } if mods == plain => { self.move_up(); Vec::new() }
            Key { code: KeyCode::Char('l'), mods } if mods == plain => { self.move_right(); Vec::new() }
            _ => Vec::new(),
        }
    }
}
