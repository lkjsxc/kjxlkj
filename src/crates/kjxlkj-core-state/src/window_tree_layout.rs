/// Window layout computation helpers.
use kjxlkj_core_types::{LayoutNode, WindowId};
use kjxlkj_core_ui::{WindowArea, WindowSnapshot};
use std::collections::HashMap;

use crate::window_tree::WindowState;

pub(crate) fn compute_areas(
    node: &LayoutNode,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    windows: &HashMap<WindowId, WindowState>,
    out: &mut HashMap<WindowId, WindowSnapshot>,
) {
    match node {
        LayoutNode::Leaf(wid) => {
            if let Some(ws) = windows.get(wid) {
                let area = WindowArea {
                    x,
                    y,
                    width,
                    height,
                };
                out.insert(*wid, ws.snapshot(area));
            }
        }
        LayoutNode::HorizontalSplit { children, weights } => {
            let total_weight: f64 = weights.iter().sum();
            let mut cy = y;
            for (i, child) in children.iter().enumerate() {
                let w = weights.get(i).copied().unwrap_or(1.0);
                let h = ((height as f64) * w / total_weight) as u16;
                let h = h.max(1);
                compute_areas(child, x, cy, width, h, windows, out);
                cy += h;
            }
        }
        LayoutNode::VerticalSplit { children, weights } => {
            let total_weight: f64 = weights.iter().sum();
            let mut cx = x;
            for (i, child) in children.iter().enumerate() {
                let w = weights.get(i).copied().unwrap_or(1.0);
                let cw = ((width as f64) * w / total_weight) as u16;
                let cw = cw.max(1);
                compute_areas(child, cx, y, cw, height, windows, out);
                cx += cw;
            }
        }
    }
}

pub(crate) fn replace_leaf(
    node: &LayoutNode,
    target: WindowId,
    replacement: LayoutNode,
) -> LayoutNode {
    match node {
        LayoutNode::Leaf(id) if *id == target => replacement,
        LayoutNode::Leaf(id) => LayoutNode::Leaf(*id),
        LayoutNode::HorizontalSplit { children, weights } => LayoutNode::HorizontalSplit {
            children: children
                .iter()
                .map(|c| replace_leaf(c, target, replacement.clone()))
                .collect(),
            weights: weights.clone(),
        },
        LayoutNode::VerticalSplit { children, weights } => LayoutNode::VerticalSplit {
            children: children
                .iter()
                .map(|c| replace_leaf(c, target, replacement.clone()))
                .collect(),
            weights: weights.clone(),
        },
    }
}

pub(crate) fn remove_leaf(node: &LayoutNode, target: WindowId) -> LayoutNode {
    match node {
        LayoutNode::Leaf(id) if *id == target => LayoutNode::Leaf(*id),
        LayoutNode::Leaf(id) => LayoutNode::Leaf(*id),
        LayoutNode::HorizontalSplit { children, weights } => {
            let filtered: Vec<(LayoutNode, f64)> = children
                .iter()
                .zip(weights.iter())
                .filter(|(c, _)| !matches!(c, LayoutNode::Leaf(id) if *id == target))
                .map(|(c, w)| (remove_leaf(c, target), *w))
                .collect();
            if filtered.len() == 1 {
                filtered.into_iter().next().unwrap().0
            } else {
                LayoutNode::HorizontalSplit {
                    children: filtered.iter().map(|(c, _)| c.clone()).collect(),
                    weights: filtered.iter().map(|(_, w)| *w).collect(),
                }
            }
        }
        LayoutNode::VerticalSplit { children, weights } => {
            let filtered: Vec<(LayoutNode, f64)> = children
                .iter()
                .zip(weights.iter())
                .filter(|(c, _)| !matches!(c, LayoutNode::Leaf(id) if *id == target))
                .map(|(c, w)| (remove_leaf(c, target), *w))
                .collect();
            if filtered.len() == 1 {
                filtered.into_iter().next().unwrap().0
            } else {
                LayoutNode::VerticalSplit {
                    children: filtered.iter().map(|(c, _)| c.clone()).collect(),
                    weights: filtered.iter().map(|(_, w)| *w).collect(),
                }
            }
        }
    }
}
