//! Directional window focus using layout geometry.
//!
//! Computes virtual rectangles from the layout tree, then finds
//! the nearest neighbor in the requested direction.

use kjxlkj_core_types::WindowId;
use kjxlkj_core_ui::LayoutNode;

/// Direction for focus navigation (Ctrl-w h/j/k/l).
#[derive(Debug, Clone, Copy)]
pub enum FocusDir {
    Left,
    Right,
    Up,
    Down,
}

/// Virtual rectangle in normalised [0,1] space.
#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

/// Compute virtual rectangles for every leaf in a layout tree.
fn layout_rects(node: &LayoutNode) -> Vec<(WindowId, Rect)> {
    let root = Rect {
        x: 0.0,
        y: 0.0,
        w: 1.0,
        h: 1.0,
    };
    rects_inner(node, root)
}

fn rects_inner(node: &LayoutNode, r: Rect) -> Vec<(WindowId, Rect)> {
    match node {
        LayoutNode::Leaf(id) => vec![(*id, r)],
        LayoutNode::Horizontal(children) => {
            // Horizontal split → windows stacked vertically.
            let tw: f32 = children.iter().map(|c| c.weight).sum();
            let mut y = r.y;
            let mut out = Vec::new();
            for c in children {
                let h = r.h * c.weight / tw;
                let cr = Rect {
                    x: r.x,
                    y,
                    w: r.w,
                    h,
                };
                out.extend(rects_inner(&c.node, cr));
                y += h;
            }
            out
        }
        LayoutNode::Vertical(children) => {
            // Vertical split → windows side-by-side.
            let tw: f32 = children.iter().map(|c| c.weight).sum();
            let mut x = r.x;
            let mut out = Vec::new();
            for c in children {
                let w = r.w * c.weight / tw;
                let cr = Rect {
                    x,
                    y: r.y,
                    w,
                    h: r.h,
                };
                out.extend(rects_inner(&c.node, cr));
                x += w;
            }
            out
        }
    }
}

/// Find the best window to focus from `current` in direction `dir`.
pub fn find_focus(layout: &LayoutNode, current: WindowId, dir: FocusDir) -> Option<WindowId> {
    let rects = layout_rects(layout);
    let cur = rects.iter().find(|(id, _)| *id == current)?.1;
    let eps = 0.001;

    let mut best: Option<(WindowId, f32)> = None;
    for &(id, r) in &rects {
        if id == current {
            continue;
        }
        let valid = match dir {
            FocusDir::Left => r.x + r.w <= cur.x + eps && overlaps_v(&cur, &r),
            FocusDir::Right => r.x >= cur.x + cur.w - eps && overlaps_v(&cur, &r),
            FocusDir::Up => r.y + r.h <= cur.y + eps && overlaps_h(&cur, &r),
            FocusDir::Down => r.y >= cur.y + cur.h - eps && overlaps_h(&cur, &r),
        };
        if !valid {
            continue;
        }
        let dist = match dir {
            FocusDir::Left => cur.x - (r.x + r.w),
            FocusDir::Right => r.x - (cur.x + cur.w),
            FocusDir::Up => cur.y - (r.y + r.h),
            FocusDir::Down => r.y - (cur.y + cur.h),
        };
        if best.is_none() || dist < best.unwrap().1 {
            best = Some((id, dist));
        }
    }
    best.map(|(id, _)| id)
}

/// Vertical overlap check (for left/right navigation).
fn overlaps_v(a: &Rect, b: &Rect) -> bool {
    a.y < b.y + b.h && b.y < a.y + a.h
}

/// Horizontal overlap check (for up/down navigation).
fn overlaps_h(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w
}
