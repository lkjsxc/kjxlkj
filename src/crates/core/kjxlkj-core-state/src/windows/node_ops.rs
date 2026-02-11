use std::collections::HashMap;

use super::{Axis, Direction, Rect};

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Leaf(u64),
    Split(Axis, Box<Node>, Box<Node>),
}

pub(crate) fn collect_leaves(node: &Node, ids: &mut Vec<u64>) {
    match node {
        Node::Leaf(id) => ids.push(*id),
        Node::Split(_, a, b) => {
            collect_leaves(a, ids);
            collect_leaves(b, ids);
        }
    }
}

pub(crate) fn replace_leaf(node: &Node, target: u64, replacement: &Node) -> Option<Node> {
    match node {
        Node::Leaf(id) => {
            if *id == target {
                Some(replacement.clone())
            } else {
                Some(Node::Leaf(*id))
            }
        }
        Node::Split(axis, a, b) => Some(Node::Split(
            *axis,
            Box::new(replace_leaf(a, target, replacement)?),
            Box::new(replace_leaf(b, target, replacement)?),
        )),
    }
}

pub(crate) fn remove_leaf(node: &Node, target: u64) -> Option<Node> {
    match node {
        Node::Leaf(id) => {
            if *id == target {
                None
            } else {
                Some(Node::Leaf(*id))
            }
        }
        Node::Split(axis, a, b) => match (remove_leaf(a, target), remove_leaf(b, target)) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(x), Some(y)) => Some(Node::Split(*axis, Box::new(x), Box::new(y))),
        },
    }
}

pub(crate) fn layout_node(node: &Node, area: Rect, out: &mut HashMap<u64, Rect>) {
    match node {
        Node::Leaf(id) => {
            out.insert(*id, area);
        }
        Node::Split(Axis::Horizontal, a, b) => {
            let top = area.rows / 2;
            layout_node(a, Rect { rows: top, ..area }, out);
            layout_node(
                b,
                Rect {
                    row: area.row + top,
                    rows: area.rows - top,
                    ..area
                },
                out,
            );
        }
        Node::Split(Axis::Vertical, a, b) => {
            let left = area.cols / 2;
            layout_node(a, Rect { cols: left, ..area }, out);
            layout_node(
                b,
                Rect {
                    col: area.col + left,
                    cols: area.cols - left,
                    ..area
                },
                out,
            );
        }
    }
}

pub(crate) fn rank_direction(from: Rect, to: Rect, dir: Direction) -> Option<(u16, u16)> {
    match dir {
        Direction::Left if to.col + to.cols <= from.col => Some((
            from.col - (to.col + to.cols),
            overlap_len(from.row, from.rows, to.row, to.rows),
        )),
        Direction::Right if from.col + from.cols <= to.col => Some((
            to.col - (from.col + from.cols),
            overlap_len(from.row, from.rows, to.row, to.rows),
        )),
        Direction::Up if to.row + to.rows <= from.row => Some((
            from.row - (to.row + to.rows),
            overlap_len(from.col, from.cols, to.col, to.cols),
        )),
        Direction::Down if from.row + from.rows <= to.row => Some((
            to.row - (from.row + from.rows),
            overlap_len(from.col, from.cols, to.col, to.cols),
        )),
        _ => None,
    }
}

pub(crate) fn overlap_area(a: Rect, b: Rect) -> u32 {
    u32::from(overlap_len(a.row, a.rows, b.row, b.rows))
        * u32::from(overlap_len(a.col, a.cols, b.col, b.cols))
}

fn overlap_len(a0: u16, al: u16, b0: u16, bl: u16) -> u16 {
    let a1 = a0 + al;
    let b1 = b0 + bl;
    a1.min(b1).saturating_sub(a0.max(b0))
}
