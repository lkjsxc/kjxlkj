/// Window split/layout tree — split, unsplit, navigate, resize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDir { Horizontal, Vertical }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDir { Left, Right, Up, Down }

/// A node in the window layout tree.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf { win_id: WinId, weight: f32 },
    Split { dir: SplitDir, children: Vec<LayoutNode>, weight: f32 },
}

impl LayoutNode {
    pub fn leaf(id: WinId) -> Self { LayoutNode::Leaf { win_id: id, weight: 1.0 } }
    pub fn weight(&self) -> f32 {
        match self { LayoutNode::Leaf { weight, .. } | LayoutNode::Split { weight, .. } => *weight }
    }
    pub fn all_windows(&self) -> Vec<WinId> { let mut o = Vec::new(); self.collect_wins(&mut o); o }

    fn collect_wins(&self, out: &mut Vec<WinId>) {
        match self {
            LayoutNode::Leaf { win_id, .. } => out.push(*win_id),
            LayoutNode::Split { children, .. } => { for c in children { c.collect_wins(out); } }
        }
    }
    pub fn window_count(&self) -> usize { self.all_windows().len() }
    pub fn first_window(&self) -> Option<WinId> {
        match self {
            LayoutNode::Leaf { win_id, .. } => Some(*win_id),
            LayoutNode::Split { children, .. } => children.first()?.first_window(),
        }
    }
    pub fn last_window(&self) -> Option<WinId> {
        match self {
            LayoutNode::Leaf { win_id, .. } => Some(*win_id),
            LayoutNode::Split { children, .. } => children.last()?.last_window(),
        }
    }
}

/// Computed rectangle for a window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WinRect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 }

pub fn compute_rects(node: &LayoutNode, x: f32, y: f32, w: f32, h: f32) -> Vec<(WinId, WinRect)> {
    let mut out = Vec::new();
    compute_rects_inner(node, x, y, w, h, &mut out);
    out
}

fn compute_rects_inner(node: &LayoutNode, x: f32, y: f32, w: f32, h: f32, out: &mut Vec<(WinId, WinRect)>) {
    match node {
        LayoutNode::Leaf { win_id, .. } => {
            out.push((*win_id, WinRect { x, y, w, h }));
        }
        LayoutNode::Split { dir, children, .. } => {
            let total_weight: f32 = children.iter().map(|c| c.weight()).sum();
            if total_weight <= 0.0 { return; }
            let mut offset = 0.0;
            for child in children {
                let frac = child.weight() / total_weight;
                match dir {
                    SplitDir::Vertical => {
                        let cw = w * frac;
                        compute_rects_inner(child, x + offset, y, cw, h, out);
                        offset += cw;
                    }
                    SplitDir::Horizontal => {
                        let ch = h * frac;
                        compute_rects_inner(child, x, y + offset, w, ch, out);
                        offset += ch;
                    }
                }
            }
        }
    }
}

/// Split a leaf window, adding a new window beside it.
pub fn split_window(node: &LayoutNode, target: WinId, new_id: WinId, dir: SplitDir) -> LayoutNode {
    match node {
        LayoutNode::Leaf { win_id, weight } if *win_id == target => {
            LayoutNode::Split {
                dir,
                children: vec![
                    LayoutNode::Leaf { win_id: *win_id, weight: 1.0 },
                    LayoutNode::Leaf { win_id: new_id, weight: 1.0 },
                ],
                weight: *weight,
            }
        }
        LayoutNode::Leaf { .. } => node.clone(),
        LayoutNode::Split { dir: d, children, weight } => {
            let new_children: Vec<_> = children.iter()
                .map(|c| split_window(c, target, new_id, dir)).collect();
            LayoutNode::Split { dir: *d, children: new_children, weight: *weight }
        }
    }
}

/// Remove a window from the layout, simplifying single-child splits.
pub fn remove_window(node: &LayoutNode, target: WinId) -> Option<LayoutNode> {
    match node {
        LayoutNode::Leaf { win_id, .. } if *win_id == target => None,
        LayoutNode::Leaf { .. } => Some(node.clone()),
        LayoutNode::Split { dir, children, weight } => {
            let nc: Vec<_> = children.iter().filter_map(|c| remove_window(c, target)).collect();
            match nc.len() {
                0 => None, 1 => Some(nc.into_iter().next().unwrap()),
                _ => Some(LayoutNode::Split { dir: *dir, children: nc, weight: *weight }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_window_rect() {
        let node = LayoutNode::leaf(WinId(1));
        let rects = compute_rects(&node, 0.0, 0.0, 80.0, 24.0);
        assert_eq!(rects.len(), 1); assert_eq!(rects[0].0, WinId(1));
        assert!((rects[0].1.w - 80.0).abs() < 0.01);
    }

    #[test]
    fn vertical_split_rects() {
        let node = LayoutNode::Split { dir: SplitDir::Vertical,
            children: vec![LayoutNode::leaf(WinId(1)), LayoutNode::leaf(WinId(2))], weight: 1.0 };
        let rects = compute_rects(&node, 0.0, 0.0, 80.0, 24.0);
        assert_eq!(rects.len(), 2); assert!((rects[0].1.w - 40.0).abs() < 0.01);
        assert!((rects[1].1.x - 40.0).abs() < 0.01);
    }

    #[test]
    fn horizontal_split_rects() {
        let node = LayoutNode::Split { dir: SplitDir::Horizontal,
            children: vec![LayoutNode::leaf(WinId(1)), LayoutNode::leaf(WinId(2))], weight: 1.0 };
        let rects = compute_rects(&node, 0.0, 0.0, 80.0, 24.0);
        assert_eq!(rects.len(), 2); assert!((rects[0].1.h - 12.0).abs() < 0.01);
    }

    #[test]
    fn split_creates_two() {
        let node = LayoutNode::leaf(WinId(1));
        let split = split_window(&node, WinId(1), WinId(2), SplitDir::Vertical);
        assert_eq!(split.window_count(), 2);
    }

    #[test]
    fn remove_simplifies() {
        let node = LayoutNode::Split { dir: SplitDir::Vertical,
            children: vec![LayoutNode::leaf(WinId(1)), LayoutNode::leaf(WinId(2))], weight: 1.0 };
        let result = remove_window(&node, WinId(1)).unwrap();
        assert_eq!(result.window_count(), 1); assert_eq!(result.first_window(), Some(WinId(2)));
    }

    #[test]
    fn all_windows_nested() {
        let node = LayoutNode::Split { dir: SplitDir::Vertical, children: vec![
            LayoutNode::leaf(WinId(1)),
            LayoutNode::Split { dir: SplitDir::Horizontal,
                children: vec![LayoutNode::leaf(WinId(2)), LayoutNode::leaf(WinId(3))], weight: 1.0 },
        ], weight: 1.0 };
        assert_eq!(node.all_windows(), vec![WinId(1), WinId(2), WinId(3)]);
    }

    #[test]
    fn first_last_window() {
        let node = LayoutNode::Split { dir: SplitDir::Vertical,
            children: vec![LayoutNode::leaf(WinId(5)), LayoutNode::leaf(WinId(9))], weight: 1.0 };
        assert_eq!(node.first_window(), Some(WinId(5)));
        assert_eq!(node.last_window(), Some(WinId(9)));
    }

    #[test]
    fn remove_last_returns_none() {
        let node = LayoutNode::leaf(WinId(1));
        assert!(remove_window(&node, WinId(1)).is_none());
    }
}
