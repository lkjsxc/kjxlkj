/// UI component behaviors — reusable building blocks for views and features.

/// Component identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(pub u32);

/// Component kind classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind { StatusLine, TabLine, CommandLine, LineNumbers, SignColumn, FoldColumn, VerticalSep, HorizontalSep, WinBar, MsgArea }

/// Component visibility state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility { Visible, Hidden, Collapsed }

/// A UI component with position and dimensions.
#[derive(Debug, Clone)]
pub struct Component {
    pub id: ComponentId,
    pub kind: ComponentKind,
    pub row: u16,
    pub col: u16,
    pub width: u16,
    pub height: u16,
    pub visibility: Visibility,
}

impl Component {
    pub fn new(id: u32, kind: ComponentKind, row: u16, col: u16, w: u16, h: u16) -> Self {
        Self { id: ComponentId(id), kind, row, col, width: w, height: h, visibility: Visibility::Visible }
    }

    pub fn is_visible(&self) -> bool { self.visibility == Visibility::Visible }
    pub fn contains(&self, r: u16, c: u16) -> bool { r >= self.row && r < self.row + self.height && c >= self.col && c < self.col + self.width }
    pub fn area(&self) -> u32 { self.width as u32 * self.height as u32 }
}

/// Layout a standard editor frame.
pub fn layout_frame(width: u16, height: u16, show_tabline: bool, show_statusline: bool) -> Vec<Component> {
    let mut parts = Vec::new();
    let mut id = 0u32;
    let mut top = 0u16;
    if show_tabline {
        parts.push(Component::new(id, ComponentKind::TabLine, 0, 0, width, 1));
        id += 1; top = 1;
    }
    let bottom_reserve = if show_statusline { 2 } else { 1 }; // statusline + cmdline, or just cmdline
    let editor_h = height.saturating_sub(top + bottom_reserve);
    parts.push(Component::new(id, ComponentKind::LineNumbers, top, 0, 4, editor_h));
    id += 1;
    if show_statusline {
        parts.push(Component::new(id, ComponentKind::StatusLine, top + editor_h, 0, width, 1));
        id += 1;
    }
    parts.push(Component::new(id, ComponentKind::CommandLine, height - 1, 0, width, 1));
    parts
}

/// Find component at position.
pub fn component_at(components: &[Component], row: u16, col: u16) -> Option<&Component> {
    components.iter().filter(|c| c.is_visible()).find(|c| c.contains(row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_basics() {
        let c = Component::new(1, ComponentKind::StatusLine, 23, 0, 80, 1);
        assert!(c.is_visible()); assert_eq!(c.area(), 80);
    }

    #[test]
    fn contains_check() {
        let c = Component::new(1, ComponentKind::TabLine, 0, 0, 80, 1);
        assert!(c.contains(0, 40)); assert!(!c.contains(1, 0));
    }

    #[test]
    fn layout_with_all() {
        let parts = layout_frame(80, 24, true, true);
        assert!(parts.len() >= 3);
        assert_eq!(parts[0].kind, ComponentKind::TabLine);
    }

    #[test]
    fn layout_minimal() {
        let parts = layout_frame(80, 24, false, false);
        assert!(parts.iter().any(|c| c.kind == ComponentKind::CommandLine));
    }

    #[test]
    fn find_at_position() {
        let parts = layout_frame(80, 24, true, true);
        let found = component_at(&parts, 0, 10);
        assert!(found.is_some());
        assert_eq!(found.unwrap().kind, ComponentKind::TabLine);
    }

    #[test]
    fn hidden_not_found() {
        let mut c = Component::new(0, ComponentKind::MsgArea, 0, 0, 80, 1);
        c.visibility = Visibility::Hidden;
        assert!(component_at(&[c], 0, 0).is_none());
    }

    #[test]
    fn all_component_kinds() {
        let kinds = [ComponentKind::StatusLine, ComponentKind::TabLine, ComponentKind::CommandLine,
            ComponentKind::LineNumbers, ComponentKind::SignColumn, ComponentKind::FoldColumn,
            ComponentKind::VerticalSep, ComponentKind::HorizontalSep, ComponentKind::WinBar, ComponentKind::MsgArea];
        assert_eq!(kinds.len(), 10);
    }
}
