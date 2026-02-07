//! UI component model: layout frames and component types.

use serde::{Deserialize, Serialize};

/// Kind of UI component rendered on screen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentKind {
    StatusLine,
    TabLine,
    CommandLine,
    LineNumbers,
    SignColumn,
    BufferView,
    Gutter,
    VerticalSep,
    MessageArea,
    FloatWindow,
}

/// A positioned, sized UI component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub kind: ComponentKind,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub visible: bool,
}

/// Lay out the editor frame into components.
pub fn layout_frame(
    width: u16,
    height: u16,
    show_tabline: bool,
    show_numbers: bool,
) -> Vec<Component> {
    let mut components = Vec::new();
    let mut top = 0u16;

    // Tab line (top row)
    if show_tabline {
        components.push(Component {
            kind: ComponentKind::TabLine,
            x: 0, y: top, w: width, h: 1, visible: true,
        });
        top += 1;
    }

    let bottom_reserve = 2; // status line + command line
    let buffer_height = height.saturating_sub(top + bottom_reserve);

    // Gutter / line numbers
    let gutter_w = if show_numbers { 6 } else { 0 };
    if show_numbers {
        components.push(Component {
            kind: ComponentKind::LineNumbers,
            x: 0, y: top, w: 4, h: buffer_height, visible: true,
        });
        components.push(Component {
            kind: ComponentKind::SignColumn,
            x: 4, y: top, w: 2, h: buffer_height, visible: true,
        });
    }

    // Main buffer view
    components.push(Component {
        kind: ComponentKind::BufferView,
        x: gutter_w, y: top, w: width.saturating_sub(gutter_w), h: buffer_height, visible: true,
    });

    // Status line
    let status_y = top + buffer_height;
    components.push(Component {
        kind: ComponentKind::StatusLine,
        x: 0, y: status_y, w: width, h: 1, visible: true,
    });

    // Command line / message area (last row)
    components.push(Component {
        kind: ComponentKind::CommandLine,
        x: 0, y: status_y + 1, w: width, h: 1, visible: true,
    });

    components
}

/// Find which component is at the given screen coordinate.
pub fn component_at(components: &[Component], x: u16, y: u16) -> Option<&Component> {
    // Search in reverse so floating windows take priority.
    components.iter().rev().find(|c| {
        c.visible && x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + c.h
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_basic() {
        let comps = layout_frame(80, 24, true, true);
        assert!(comps.iter().any(|c| c.kind == ComponentKind::TabLine));
        assert!(comps.iter().any(|c| c.kind == ComponentKind::BufferView));
        assert!(comps.iter().any(|c| c.kind == ComponentKind::StatusLine));
        assert!(comps.iter().any(|c| c.kind == ComponentKind::CommandLine));
    }

    #[test]
    fn component_at_finds_buffer() {
        let comps = layout_frame(80, 24, false, false);
        let c = component_at(&comps, 10, 5);
        assert!(c.is_some());
        assert_eq!(c.unwrap().kind, ComponentKind::BufferView);
    }

    #[test]
    fn layout_no_tabline() {
        let comps = layout_frame(80, 24, false, false);
        assert!(!comps.iter().any(|c| c.kind == ComponentKind::TabLine));
    }
}
