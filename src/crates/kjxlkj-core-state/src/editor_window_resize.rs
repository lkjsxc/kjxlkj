//! Window resize, equalize, zoom, rotate, and move
//! operations for EditorState.

use kjxlkj_core_types::{Direction, WindowId};

use crate::EditorState;

impl EditorState {
    /// Resize the focused window.
    ///
    /// Direction determines the axis:
    /// - Up/Down: change height by `amount` rows.
    /// - Left/Right: change width by `amount` cols.
    ///
    /// In the simplified single-layout model, we adjust
    /// the viewport size of the focused window.
    pub(crate) fn do_resize_window(&mut self, direction: Direction, amount: i32) {
        let (cols, rows) = self.terminal_size;
        if let Some(w) = self.focused_window_mut() {
            match direction {
                Direction::Up | Direction::Down => {
                    let cur = w.viewport.height as i32;
                    let new = (cur + amount).max(1).min(rows as i32);
                    w.viewport.height = new as u16;
                }
                Direction::Left | Direction::Right => {
                    let cur = w.viewport.width as i32;
                    let new = (cur + amount).max(1).min(cols as i32);
                    w.viewport.width = new as u16;
                }
            }
        }
    }

    /// Equalize all window sizes (`Ctrl-w =`).
    pub(crate) fn do_equalize_windows(&mut self) {
        let count = self.windows.len() as u16;
        if count == 0 {
            return;
        }
        let (cols, rows) = self.terminal_size;
        let h = rows.saturating_sub(2) / count;
        for win in self.windows.values_mut() {
            win.viewport.set_size(cols, h.max(1));
        }
    }

    /// Zoom (maximize) the focused window (`Ctrl-w _`
    /// and `Ctrl-w |`).
    pub(crate) fn do_zoom_window(&mut self) {
        let (cols, rows) = self.terminal_size;
        if let Some(w) = self.focused_window_mut() {
            w.viewport.set_size(cols, rows.saturating_sub(2));
        }
    }

    /// Rotate windows forward or backward.
    pub(crate) fn do_rotate_windows(&mut self, forward: bool) {
        let mut ids: Vec<WindowId> = self.windows.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        ids.sort_by_key(|id| id.0);

        // Collect buffer IDs in window order.
        let contents: Vec<_> = ids.iter().map(|id| self.windows[id].buffer_id()).collect();

        let n = ids.len();
        for (i, id) in ids.iter().enumerate() {
            let src = if forward {
                (i + n - 1) % n
            } else {
                (i + 1) % n
            };
            if let Some(buf_id) = contents[src] {
                if let Some(w) = self.windows.get_mut(id) {
                    w.set_buffer(buf_id);
                }
            }
        }
    }

    /// Move the focused window to an edge.
    pub(crate) fn do_move_window(&mut self, _direction: Direction) {
        // In the simplified layout model, this is
        // equivalent to moving focus. Full layout
        // tree reordering would go here.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_window_height() {
        let mut ed = EditorState::new(80, 24);
        let h_before = ed.focused_window().unwrap().viewport.height;
        ed.do_resize_window(Direction::Up, 2);
        let h_after = ed.focused_window().unwrap().viewport.height;
        assert_eq!(h_after, h_before + 2);
    }

    #[test]
    fn equalize_windows() {
        let mut ed = EditorState::new(80, 24);
        ed.do_equalize_windows();
        let h = ed.focused_window().unwrap().viewport.height;
        assert!(h > 0);
    }

    #[test]
    fn zoom_window() {
        let mut ed = EditorState::new(80, 24);
        ed.do_resize_window(Direction::Up, -5);
        ed.do_zoom_window();
        let h = ed.focused_window().unwrap().viewport.height;
        assert_eq!(h, 22);
    }
}
