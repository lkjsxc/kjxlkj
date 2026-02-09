//! Window layouts per /docs/spec/features/window/window-layouts.md.
//!
//! Predefined and custom window layout presets.

/// A named window layout.
#[derive(Debug, Clone)]
pub struct WindowLayout {
    /// Layout name.
    pub name: String,
    /// Layout tree structure.
    pub root: LayoutNode,
}

/// Node in layout tree.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// A leaf window pane.
    Leaf {
        /// Weight/proportion (1-100).
        weight: u16,
    },
    /// Horizontal split.
    HSplit {
        /// Children.
        children: Vec<LayoutNode>,
    },
    /// Vertical split.
    VSplit {
        /// Children.
        children: Vec<LayoutNode>,
    },
}

/// Named layout presets.
pub fn builtin_layouts() -> Vec<WindowLayout> {
    vec![
        WindowLayout {
            name: "single".into(),
            root: LayoutNode::Leaf { weight: 100 },
        },
        WindowLayout {
            name: "dual-v".into(),
            root: LayoutNode::VSplit {
                children: vec![
                    LayoutNode::Leaf { weight: 50 },
                    LayoutNode::Leaf { weight: 50 },
                ],
            },
        },
        WindowLayout {
            name: "dual-h".into(),
            root: LayoutNode::HSplit {
                children: vec![
                    LayoutNode::Leaf { weight: 50 },
                    LayoutNode::Leaf { weight: 50 },
                ],
            },
        },
        WindowLayout {
            name: "main-side".into(),
            root: LayoutNode::VSplit {
                children: vec![
                    LayoutNode::Leaf { weight: 70 },
                    LayoutNode::HSplit {
                        children: vec![
                            LayoutNode::Leaf { weight: 50 },
                            LayoutNode::Leaf { weight: 50 },
                        ],
                    },
                ],
            },
        },
        WindowLayout {
            name: "grid-4".into(),
            root: LayoutNode::HSplit {
                children: vec![
                    LayoutNode::VSplit {
                        children: vec![
                            LayoutNode::Leaf { weight: 50 },
                            LayoutNode::Leaf { weight: 50 },
                        ],
                    },
                    LayoutNode::VSplit {
                        children: vec![
                            LayoutNode::Leaf { weight: 50 },
                            LayoutNode::Leaf { weight: 50 },
                        ],
                    },
                ],
            },
        },
    ]
}

/// Count leaf nodes in a layout.
pub fn leaf_count(node: &LayoutNode) -> usize {
    match node {
        LayoutNode::Leaf { .. } => 1,
        LayoutNode::HSplit { children }
        | LayoutNode::VSplit { children } => {
            children.iter().map(leaf_count).sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_layout_count() {
        let layouts = builtin_layouts();
        assert_eq!(layouts.len(), 5);
    }

    #[test]
    fn grid4_has_4_leaves() {
        let layouts = builtin_layouts();
        let grid = layouts
            .iter()
            .find(|l| l.name == "grid-4")
            .unwrap();
        assert_eq!(leaf_count(&grid.root), 4);
    }

    #[test]
    fn single_has_1_leaf() {
        let layouts = builtin_layouts();
        let single = layouts
            .iter()
            .find(|l| l.name == "single")
            .unwrap();
        assert_eq!(leaf_count(&single.root), 1);
    }
}
