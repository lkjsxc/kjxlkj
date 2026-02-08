//! Undo tree data structure with branching support.
//!
//! Uses an arena-allocated tree where each node represents an edit
//! and branches are preserved for full undo-tree traversal.

use serde::{Deserialize, Serialize};

/// A single reversible edit operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoEntry {
    /// Starting char index of the affected range.
    pub start: usize,
    /// The removed text (for redo: text to remove again).
    pub old_text: String,
    /// The inserted text (for undo: text to remove).
    pub new_text: String,
    /// Cursor position before the edit.
    pub cursor_before: (usize, usize),
    /// Cursor position after the edit.
    pub cursor_after: (usize, usize),
}

/// A group of edits that form a single undo step.
///
/// All edits in an insert session (from `i` to `Esc`) are grouped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoGroup {
    /// The individual edits in this group.
    pub entries: Vec<UndoEntry>,
    /// Timestamp when this group was created (ms since epoch).
    pub timestamp: u64,
}

/// A node in the undo tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UndoNode {
    /// The undo group at this node.
    group: UndoGroup,
    /// Index of the parent node (None for root).
    parent: Option<usize>,
    /// Indices of child nodes (branches).
    children: Vec<usize>,
    /// Which child branch was most recently visited.
    active_child: Option<usize>,
}

/// Tree-structured undo history with branching.
///
/// Supports linear undo/redo as well as tree traversal
/// for accessing all historical states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoTree {
    /// Arena of all undo nodes.
    nodes: Vec<UndoNode>,
    /// Index of the current node (-1 equivalent: None = before root).
    current: Option<usize>,
    /// Maximum number of undo levels (0 = unlimited).
    max_levels: usize,
    /// Whether a group is currently being recorded.
    recording: bool,
    /// Entries accumulated during the current group.
    pending_entries: Vec<UndoEntry>,
}

impl UndoTree {
    /// Create a new empty undo tree.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current: None,
            max_levels: 0,
            recording: false,
            pending_entries: Vec::new(),
        }
    }

    /// Create with a max undo level limit.
    pub fn with_max_levels(max: usize) -> Self {
        Self {
            max_levels: max,
            ..Self::new()
        }
    }

    /// Begin recording a new undo group.
    pub fn begin_group(&mut self) {
        self.recording = true;
        self.pending_entries.clear();
    }

    /// Record an edit entry within the current group.
    pub fn record(&mut self, entry: UndoEntry) {
        if !self.recording {
            // Auto-begin a single-entry group
            self.begin_group();
            self.pending_entries.push(entry);
            self.end_group();
        } else {
            self.pending_entries.push(entry);
        }
    }

    /// End the current undo group and commit it to the tree.
    pub fn end_group(&mut self) {
        if !self.recording || self.pending_entries.is_empty() {
            self.recording = false;
            return;
        }
        self.recording = false;

        let group = UndoGroup {
            entries: std::mem::take(&mut self.pending_entries),
            timestamp: current_timestamp(),
        };

        let new_idx = self.nodes.len();
        let node = UndoNode {
            group,
            parent: self.current,
            children: Vec::new(),
            active_child: None,
        };
        self.nodes.push(node);

        if let Some(current_idx) = self.current {
            self.nodes[current_idx].children.push(new_idx);
            self.nodes[current_idx].active_child = Some(new_idx);
        }

        self.current = Some(new_idx);
        self.prune_if_needed();
    }

    /// Undo: move back one step, returning the group to reverse.
    pub fn undo(&mut self) -> Option<&UndoGroup> {
        let current = self.current?;
        let group = &self.nodes[current].group;
        self.current = self.nodes[current].parent;
        Some(group)
    }

    /// Redo: move forward one step, returning the group to replay.
    pub fn redo(&mut self) -> Option<&UndoGroup> {
        let current_idx = self.current;
        let node = match current_idx {
            Some(idx) => &self.nodes[idx],
            None => {
                // At root; check if there is a first node
                if self.nodes.is_empty() {
                    return None;
                }
                self.current = Some(0);
                return Some(&self.nodes[0].group);
            }
        };
        let next_idx = node.active_child?;
        self.current = Some(next_idx);
        Some(&self.nodes[next_idx].group)
    }

    /// Number of undo levels available.
    pub fn undo_count(&self) -> usize {
        let mut count = 0;
        let mut idx = self.current;
        while let Some(i) = idx {
            count += 1;
            idx = self.nodes[i].parent;
        }
        count
    }

    /// Number of redo levels available on the active branch.
    pub fn redo_count(&self) -> usize {
        let mut count = 0;
        let mut idx = self.current;
        loop {
            let node = match idx {
                Some(i) => &self.nodes[i],
                None => {
                    if !self.nodes.is_empty() {
                        count += 1;
                    }
                    break;
                }
            };
            match node.active_child {
                Some(child) => {
                    count += 1;
                    idx = Some(child);
                }
                None => break,
            }
        }
        count
    }

    /// Total number of nodes in the tree.
    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Prune oldest nodes if max_levels is exceeded.
    fn prune_if_needed(&mut self) {
        if self.max_levels == 0 || self.nodes.len() <= self.max_levels {
            return;
        }
        // Simple strategy: do not prune for now;
        // full tree pruning with re-indexing is complex.
        // This is a placeholder for the pruning algorithm.
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
