//! Window management: split, close, focus, resize, explorer.
//!
//! See /docs/spec/features/window/splits-windows.md.

use kjxlkj_core_types::{ContentKind, Direction, ExplorerStateId, Rect, ResizeEdge, WindowId};

use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    pub(crate) fn next_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    fn leaf_rects(&self) -> Vec<(WindowId, ContentKind, Rect)> {
        let (c, r) = self.terminal_size;
        self.layout.compute_rects(Rect::new(0, 0, c, r))
    }

    /// :split — top/bottom arrangement.
    pub(crate) fn split_horizontal(&mut self) {
        let f = self.focus.focused;
        let content = self.windows.get(&f).unwrap().content;
        let nw = WindowId(self.next_id());
        self.layout.split_vertical(f, nw, content);
        self.windows.insert(nw, WindowState::new(nw, content));
        self.focus.set_focus(nw);
    }

    /// :vsplit — left/right arrangement.
    pub(crate) fn split_vertical(&mut self) {
        let f = self.focus.focused;
        let content = self.windows.get(&f).unwrap().content;
        let nw = WindowId(self.next_id());
        self.layout.split_horizontal(f, nw, content);
        self.windows.insert(nw, WindowState::new(nw, content));
        self.focus.set_focus(nw);
    }

    pub(crate) fn close_window(&mut self) {
        let f = self.focus.focused;
        if self.layout.window_ids().len() <= 1 { return; }
        if self.layout.close_window(f) {
            self.windows.remove(&f);
            let fb = self.layout.window_ids().first().copied().unwrap_or(WindowId(0));
            self.focus.on_window_closed(f, fb);
        }
    }

    pub(crate) fn window_only(&mut self) {
        let f = self.focus.focused;
        for wid in self.layout.window_ids() {
            if wid != f { self.layout.close_window(wid); self.windows.remove(&wid); }
        }
    }

    pub(crate) fn focus_cycle(&mut self) {
        let ids = self.layout.window_ids();
        if ids.len() <= 1 { return; }
        let pos = ids.iter().position(|&id| id == self.focus.focused).unwrap_or(0);
        self.focus.set_focus(ids[(pos + 1) % ids.len()]);
    }

    pub(crate) fn focus_direction(&mut self, dir: Direction) {
        let rects = self.leaf_rects();
        let cr = match rects.iter().find(|(id, _, _)| *id == self.focus.focused) {
            Some((_, _, r)) => *r, None => return,
        };
        let mut best: Option<(WindowId, u32)> = None;
        for &(wid, _, ref r) in &rects {
            if wid == self.focus.focused { continue; }
            if !is_in_direction(&cr, r, dir) { continue; }
            let d = rect_distance(&cr, r);
            if best.map_or(true, |(_, bd)| d < bd) { best = Some((wid, d)); }
        }
        if let Some((t, _)) = best { self.focus.set_focus(t); }
    }

    pub(crate) fn focus_top_left(&mut self) {
        let rects = self.leaf_rects();
        if let Some(&(w, _, _)) = rects.iter().min_by_key(|(_, _, r)| {
            (r.y as u32) * 10000 + r.x as u32
        }) { self.focus.set_focus(w); }
    }

    pub(crate) fn focus_bottom_right(&mut self) {
        let rects = self.leaf_rects();
        if let Some(&(w, _, _)) = rects.iter().max_by_key(|(_, _, r)| {
            (r.y as u32 + r.height as u32) * 10000 + r.x as u32 + r.width as u32
        }) { self.focus.set_focus(w); }
    }

    pub(crate) fn window_equalize(&mut self) { self.layout.equalize(); }
    pub(crate) fn window_resize(&mut self, _e: ResizeEdge, _d: i16) {}
    pub(crate) fn window_max_height(&mut self) {}
    pub(crate) fn window_max_width(&mut self) {}

    /// :Explorer — opens or focuses explorer window.
    pub(crate) fn open_explorer(&mut self) {
        let ec = ContentKind::Explorer(ExplorerStateId(0));
        for (&wid, ws) in &self.windows {
            if matches!(ws.content, ContentKind::Explorer(_)) {
                self.focus.set_focus(wid); return;
            }
        }
        let target = self.layout.window_ids().first().copied().unwrap_or(self.focus.focused);
        let nw = WindowId(self.next_id());
        self.layout.split_horizontal(target, nw, ec);
        self.windows.insert(nw, WindowState::new(nw, ec));
        self.focus.set_focus(nw);
    }

    /// :ExplorerClose
    pub(crate) fn close_explorer(&mut self) {
        let ew = self.windows.iter()
            .find(|(_, ws)| matches!(ws.content, ContentKind::Explorer(_)))
            .map(|(&w, _)| w);
        if let Some(wid) = ew {
            if self.layout.window_ids().len() <= 1 { return; }
            if self.layout.close_window(wid) {
                self.windows.remove(&wid);
                if self.focus.focused == wid {
                    let fb = self.layout.window_ids().first().copied().unwrap_or(WindowId(0));
                    self.focus.on_window_closed(wid, fb);
                }
            }
        }
    }
}

fn is_in_direction(origin: &Rect, cand: &Rect, dir: Direction) -> bool {
    let (ox, oy) = center(origin);
    let (cx, cy) = center(cand);
    match dir {
        Direction::Left => cx < ox, Direction::Right => cx > ox,
        Direction::Up => cy < oy, Direction::Down => cy > oy,
    }
}

fn center(r: &Rect) -> (i32, i32) {
    (r.x as i32 + r.width as i32 / 2, r.y as i32 + r.height as i32 / 2)
}

fn rect_distance(a: &Rect, b: &Rect) -> u32 {
    let (ax, ay) = center(a);
    let (bx, by) = center(b);
    (ax - bx).unsigned_abs() + (ay - by).unsigned_abs()
}
