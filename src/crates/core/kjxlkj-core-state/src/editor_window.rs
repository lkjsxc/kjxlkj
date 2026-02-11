//! Window management operations: split, close, focus.
//!
//! See /docs/spec/features/window/splits-windows.md.
//! See /docs/spec/features/window/wincmd.md.

use kjxlkj_core_types::{Direction, Rect, WindowId};

use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    pub(crate) fn next_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    /// Split current window horizontally (:split).
    /// Creates a top/bottom arrangement (horizontal divider).
    pub(crate) fn split_horizontal(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        // LayoutNode::Vertical stacks children top-to-bottom.
        self.layout.split_vertical(focused, new_wid, content);
        self.windows.insert(new_wid, WindowState::new(new_wid, content));
        self.focus.set_focus(new_wid);
    }

    /// Split current window vertically (:vsplit).
    /// Creates a left/right arrangement (vertical divider).
    pub(crate) fn split_vertical(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        // LayoutNode::Horizontal lays children side-by-side.
        self.layout.split_horizontal(focused, new_wid, content);
        self.windows.insert(new_wid, WindowState::new(new_wid, content));
        self.focus.set_focus(new_wid);
    }

    pub(crate) fn close_window(&mut self) {
        let focused = self.focus.focused;
        let ids = self.layout.window_ids();
        if ids.len() <= 1 {
            return;
        }
        if self.layout.close_window(focused) {
            self.windows.remove(&focused);
            let remaining = self.layout.window_ids();
            let fallback = remaining.first().copied().unwrap_or(WindowId(0));
            self.focus.on_window_closed(focused, fallback);
        }
    }

    /// Close all windows except the focused one (:only).
    pub(crate) fn window_only(&mut self) {
        let focused = self.focus.focused;
        let ids = self.layout.window_ids();
        for wid in ids {
            if wid != focused {
                self.layout.close_window(wid);
                self.windows.remove(&wid);
            }
        }
    }

    /// Cycle focus to the next window in layout order.
    pub(crate) fn focus_cycle(&mut self) {
        let ids = self.layout.window_ids();
        if ids.len() <= 1 {
            return;
        }
        let cur = self.focus.focused;
        let pos = ids.iter().position(|&id| id == cur).unwrap_or(0);
        let next = ids[(pos + 1) % ids.len()];
        self.focus.set_focus(next);
    }

    /// Focus the nearest window in the given direction
    /// using geometry-based resolution.
    pub(crate) fn focus_direction(&mut self, dir: Direction) {
        let (cols, rows) = self.terminal_size;
        let area = Rect::new(0, 0, cols, rows);
        let rects = self.layout.compute_rects(area);
        let cur = self.focus.focused;
        let cur_rect = match rects.iter().find(|(id, _, _)| *id == cur) {
            Some((_, _, r)) => *r,
            None => return,
        };
        let mut best: Option<(WindowId, u32)> = None;
        for &(wid, _, ref r) in &rects {
            if wid == cur { continue; }
            if !is_in_direction(&cur_rect, r, dir) { continue; }
            let dist = rect_distance(&cur_rect, r);
            if best.map_or(true, |(_, d)| dist < d) {
                best = Some((wid, dist));
            }
        }
        if let Some((target, _)) = best {
            self.focus.set_focus(target);
        }
    }
}

/// Check if `candidate` is in the specified direction from `origin`.
fn is_in_direction(origin: &Rect, candidate: &Rect, dir: Direction) -> bool {
    let (ox, oy) = center(origin);
    let (cx, cy) = center(candidate);
    match dir {
        Direction::Left => cx < ox,
        Direction::Right => cx > ox,
        Direction::Up => cy < oy,
        Direction::Down => cy > oy,
    }
}

fn center(r: &Rect) -> (i32, i32) {
    (r.x as i32 + r.width as i32 / 2, r.y as i32 + r.height as i32 / 2)
}

fn rect_distance(a: &Rect, b: &Rect) -> u32 {
    let (ax, ay) = center(a);
    let (bx, by) = center(b);
    ((ax - bx).unsigned_abs() + (ay - by).unsigned_abs())
}
