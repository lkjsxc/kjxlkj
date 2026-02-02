//! Layout management using a tree structure.

use kjxlkj_core_types::ids::WindowId;
use kjxlkj_core_types::snapshot::WindowDimensions;

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Split horizontally (windows stacked vertically).
    Horizontal,
    /// Split vertically (windows side by side).
    Vertical,
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// A leaf node containing a window.
    Window(WindowId),
    /// A split node containing children.
    Split {
        direction: SplitDirection,
        children: Vec<LayoutNode>,
        /// Proportions for each child (0.0 to 1.0).
        ratios: Vec<f32>,
    },
}

impl LayoutNode {
    /// Creates a window leaf node.
    pub fn window(id: WindowId) -> Self {
        Self::Window(id)
    }

    /// Creates a horizontal split.
    pub fn horizontal(children: Vec<LayoutNode>) -> Self {
        let n = children.len();
        Self::Split {
            direction: SplitDirection::Horizontal,
            children,
            ratios: vec![1.0 / n as f32; n],
        }
    }

    /// Creates a vertical split.
    pub fn vertical(children: Vec<LayoutNode>) -> Self {
        let n = children.len();
        Self::Split {
            direction: SplitDirection::Vertical,
            children,
            ratios: vec![1.0 / n as f32; n],
        }
    }

    /// Returns the window ID if this is a leaf.
    pub fn window_id(&self) -> Option<WindowId> {
        match self {
            Self::Window(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns all window IDs in this subtree.
    pub fn all_windows(&self) -> Vec<WindowId> {
        match self {
            Self::Window(id) => vec![*id],
            Self::Split { children, .. } => children.iter().flat_map(|c| c.all_windows()).collect(),
        }
    }
}

/// The root layout container.
#[derive(Debug)]
pub struct Layout {
    /// The root layout node.
    root: LayoutNode,
    /// Total dimensions.
    dimensions: WindowDimensions,
}

impl Layout {
    /// Creates a new layout with a single window.
    pub fn new(window_id: WindowId, dimensions: WindowDimensions) -> Self {
        Self {
            root: LayoutNode::Window(window_id),
            dimensions,
        }
    }

    /// Returns the root node.
    pub fn root(&self) -> &LayoutNode {
        &self.root
    }

    /// Sets the total dimensions.
    pub fn set_dimensions(&mut self, dimensions: WindowDimensions) {
        self.dimensions = dimensions;
    }

    /// Returns all window IDs.
    pub fn all_windows(&self) -> Vec<WindowId> {
        self.root.all_windows()
    }

    /// Calculates dimensions for each window.
    pub fn calculate_window_dimensions(&self) -> Vec<(WindowId, WindowDimensions)> {
        self.calculate_node_dimensions(&self.root, self.dimensions)
    }

    fn calculate_node_dimensions(
        &self,
        node: &LayoutNode,
        area: WindowDimensions,
    ) -> Vec<(WindowId, WindowDimensions)> {
        match node {
            LayoutNode::Window(id) => vec![(*id, area)],
            LayoutNode::Split {
                direction,
                children,
                ratios,
            } => {
                let mut result = Vec::new();
                let mut offset = 0u16;

                for (child, ratio) in children.iter().zip(ratios.iter()) {
                    let child_area = match direction {
                        SplitDirection::Horizontal => {
                            let h = (area.height as f32 * ratio) as u16;
                            let dims =
                                WindowDimensions::new(area.x, area.y + offset, area.width, h);
                            offset += h;
                            dims
                        }
                        SplitDirection::Vertical => {
                            let w = (area.width as f32 * ratio) as u16;
                            let dims =
                                WindowDimensions::new(area.x + offset, area.y, w, area.height);
                            offset += w;
                            dims
                        }
                    };
                    result.extend(self.calculate_node_dimensions(child, child_area));
                }
                result
            }
        }
    }
}
