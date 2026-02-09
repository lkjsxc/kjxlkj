use crate::BufferId;

/// Stable unique window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

/// Stable unique terminal identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(pub u64);

/// What a window displays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentSource {
    Buffer(BufferId),
    Terminal(TerminalId),
}

/// Recursive layout tree node.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf(WindowId),
    HorizontalSplit {
        children: Vec<LayoutNode>,
        weights: Vec<f64>,
    },
    VerticalSplit {
        children: Vec<LayoutNode>,
        weights: Vec<f64>,
    },
}

impl LayoutNode {
    /// Collect all window IDs in this subtree.
    pub fn window_ids(&self) -> Vec<WindowId> {
        match self {
            Self::Leaf(id) => vec![*id],
            Self::HorizontalSplit { children, .. } | Self::VerticalSplit { children, .. } => {
                children.iter().flat_map(|c| c.window_ids()).collect()
            }
        }
    }
}
