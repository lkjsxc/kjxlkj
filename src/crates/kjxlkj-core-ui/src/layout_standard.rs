//! Standard editor layout computation.

use crate::layout::{ComponentKind, LayoutNode, Rect};

/// Compute a standard editor layout for the given terminal size.
///
/// Layout (top to bottom): TabLine (1 row), BufferView (remaining), StatusLine (1 row),
/// CommandLine (1 row). LineNumbers and SignColumn are embedded in the buffer area.
pub fn standard_layout(width: u16, height: u16) -> Vec<LayoutNode> {
    let mut nodes = Vec::new();
    let mut id = 1u64;
    let mut y = 0u16;

    // Tab line at top
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::TabLine,
        rect: Rect::new(0, y, width, 1),
        visible: true,
    });
    id += 1;
    y += 1;

    let body_h = height.saturating_sub(3); // tab + status + cmdline

    // Sign column (1 col)
    let sign_w = 2u16.min(width);
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::SignColumn,
        rect: Rect::new(0, y, sign_w, body_h),
        visible: true,
    });
    id += 1;

    // Line numbers (4 cols)
    let num_x = sign_w;
    let num_w = 4u16.min(width.saturating_sub(num_x));
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::LineNumbers,
        rect: Rect::new(num_x, y, num_w, body_h),
        visible: true,
    });
    id += 1;

    // Buffer view (remaining)
    let buf_x = num_x + num_w;
    let buf_w = width.saturating_sub(buf_x);
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::BufferView,
        rect: Rect::new(buf_x, y, buf_w, body_h),
        visible: true,
    });
    id += 1;
    y += body_h;

    // Status line
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::StatusLine,
        rect: Rect::new(0, y, width, 1),
        visible: true,
    });
    id += 1;
    y += 1;

    // Command line
    nodes.push(LayoutNode {
        id,
        kind: ComponentKind::CommandLine,
        rect: Rect::new(0, y, width, 1),
        visible: true,
    });
    let _ = id;

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_layout_has_all_kinds() {
        let nodes = standard_layout(80, 24);
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::TabLine));
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::BufferView));
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::StatusLine));
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::CommandLine));
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::LineNumbers));
        assert!(nodes.iter().any(|n| n.kind == ComponentKind::SignColumn));
    }
}
