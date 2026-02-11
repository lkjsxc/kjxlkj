use std::collections::HashMap;

use super::node_ops::{collect_leaves, layout_node, overlap_area, rank_direction};
use super::{Rect, WindowTree};

impl WindowTree {
    pub fn focus_top_left(&mut self, area: Rect) {
        let id = self
            .layout(area)
            .iter()
            .min_by_key(|(_, rect)| (rect.row, rect.col))
            .map(|(id, _)| *id)
            .unwrap_or(self.focused);
        self.focus_window(id);
    }

    pub fn focus_bottom_right(&mut self, area: Rect) {
        let id = self
            .layout(area)
            .iter()
            .max_by_key(|(_, rect)| (rect.row + rect.rows, rect.col + rect.cols))
            .map(|(id, _)| *id)
            .unwrap_or(self.focused);
        self.focus_window(id);
    }

    pub fn focus_direction(&mut self, direction: super::Direction, area: Rect) {
        let layout = self.layout(area);
        let Some(current) = layout.get(&self.focused) else {
            return;
        };
        let mut candidates: Vec<(u16, u16, u64, u64)> = layout
            .iter()
            .filter_map(|(id, rect)| {
                if *id == self.focused {
                    return None;
                }
                let (dist, overlap) = rank_direction(*current, *rect, direction)?;
                let seq = self.focus_seq.get(id).copied().unwrap_or(0);
                Some((dist, u16::MAX - overlap, u64::MAX - seq, *id))
            })
            .collect();
        candidates.sort_unstable();
        if let Some((_, _, _, id)) = candidates.first().copied() {
            self.focus_window(id);
        }
    }

    pub fn geometry_invariants_hold(&self, area: Rect) -> bool {
        let layout = self.layout(area);
        let mut covered = 0_u32;
        for a in layout.values() {
            if a.rows == 0 || a.cols == 0 {
                return false;
            }
            covered += u32::from(a.rows) * u32::from(a.cols);
            for b in layout.values() {
                if std::ptr::eq(a, b) {
                    continue;
                }
                if overlap_area(*a, *b) > 0 {
                    return false;
                }
            }
        }
        covered == u32::from(area.rows) * u32::from(area.cols)
    }

    pub(super) fn cycle(&mut self, forward: bool) {
        let ids = self.leaf_ids();
        if ids.is_empty() {
            return;
        }
        let idx = ids.iter().position(|id| *id == self.focused).unwrap_or(0);
        let next = if forward {
            (idx + 1) % ids.len()
        } else {
            (idx + ids.len() - 1) % ids.len()
        };
        self.focus_window(ids[next]);
    }

    pub(super) fn focus_window(&mut self, id: u64) {
        if self.focused != id {
            self.previous = Some(self.focused);
            self.focused = id;
        }
        self.seq += 1;
        self.focus_seq.insert(id, self.seq);
    }

    pub(super) fn layout(&self, area: Rect) -> HashMap<u64, Rect> {
        let mut output = HashMap::new();
        layout_node(&self.root, area, &mut output);
        output
    }

    pub(super) fn leaf_ids(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        collect_leaves(&self.root, &mut ids);
        ids
    }
}
