/// Popup and overlay state management — menus, hover, signature help.

/// Popup kind classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupKind { CompletionMenu, Hover, SignatureHelp, CommandPalette, ContextMenu, Wildmenu }

/// Popup position anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupAnchor { Cursor, TopLeft, Center, CmdLine }

/// Popup display state.
#[derive(Debug, Clone)]
pub struct PopupState {
    pub kind: PopupKind,
    pub anchor: PopupAnchor,
    pub items: Vec<String>,
    pub selected: Option<usize>,
    pub max_visible: usize,
    pub scroll_offset: usize,
    pub visible: bool,
}

impl PopupState {
    pub fn new(kind: PopupKind, anchor: PopupAnchor) -> Self {
        Self { kind, anchor, items: Vec::new(), selected: None, max_visible: 10, scroll_offset: 0, visible: false }
    }

    pub fn show(&mut self, items: Vec<String>) {
        self.items = items; self.selected = if self.items.is_empty() { None } else { Some(0) };
        self.scroll_offset = 0; self.visible = true;
    }

    pub fn hide(&mut self) { self.visible = false; self.items.clear(); self.selected = None; }

    pub fn select_next(&mut self) {
        if let Some(sel) = self.selected {
            if sel + 1 < self.items.len() {
                self.selected = Some(sel + 1);
                if sel + 1 >= self.scroll_offset + self.max_visible {
                    self.scroll_offset += 1;
                }
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(sel) = self.selected {
            if sel > 0 {
                self.selected = Some(sel - 1);
                if sel - 1 < self.scroll_offset { self.scroll_offset = sel - 1; }
            }
        }
    }

    pub fn current_item(&self) -> Option<&str> {
        self.selected.and_then(|i| self.items.get(i).map(|s| s.as_str()))
    }

    pub fn visible_items(&self) -> &[String] {
        let end = (self.scroll_offset + self.max_visible).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    pub fn needs_scroll(&self) -> bool { self.items.len() > self.max_visible }
}

/// Overlay layer for combining multiple popups.
#[derive(Debug, Default)]
pub struct OverlayManager { popups: Vec<PopupState> }

impl OverlayManager {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, popup: PopupState) { self.popups.push(popup); }

    pub fn close_kind(&mut self, kind: PopupKind) { self.popups.retain(|p| p.kind != kind); }

    pub fn close_all(&mut self) { self.popups.clear(); }

    pub fn top(&self) -> Option<&PopupState> { self.popups.last() }

    pub fn has_popup(&self, kind: PopupKind) -> bool { self.popups.iter().any(|p| p.kind == kind) }

    pub fn count(&self) -> usize { self.popups.len() }
}

/// Compute popup position given anchor and dimensions.
pub fn compute_popup_rect(anchor: PopupAnchor, cursor_row: u16, cursor_col: u16,
    popup_h: u16, popup_w: u16, screen_h: u16, screen_w: u16) -> (u16, u16) {
    match anchor {
        PopupAnchor::Cursor => {
            let row = if cursor_row + 1 + popup_h <= screen_h { cursor_row + 1 }
                else { cursor_row.saturating_sub(popup_h) };
            let col = cursor_col.min(screen_w.saturating_sub(popup_w));
            (row, col)
        }
        PopupAnchor::TopLeft => (0, 0),
        PopupAnchor::Center => ((screen_h.saturating_sub(popup_h)) / 2, (screen_w.saturating_sub(popup_w)) / 2),
        PopupAnchor::CmdLine => (screen_h.saturating_sub(popup_h).saturating_sub(1), 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popup_show_hide() {
        let mut p = PopupState::new(PopupKind::CompletionMenu, PopupAnchor::Cursor);
        p.show(vec!["a".into(), "b".into(), "c".into()]);
        assert!(p.visible);
        assert_eq!(p.selected, Some(0));
        p.hide();
        assert!(!p.visible);
    }

    #[test]
    fn select_next_prev() {
        let mut p = PopupState::new(PopupKind::CompletionMenu, PopupAnchor::Cursor);
        p.show(vec!["a".into(), "b".into(), "c".into()]);
        p.select_next();
        assert_eq!(p.current_item(), Some("b"));
        p.select_prev();
        assert_eq!(p.current_item(), Some("a"));
    }

    #[test]
    fn scroll_on_overflow() {
        let mut p = PopupState::new(PopupKind::CompletionMenu, PopupAnchor::Cursor);
        p.max_visible = 2;
        p.show((0..5).map(|i| format!("item{}", i)).collect());
        p.select_next(); p.select_next();
        assert!(p.scroll_offset > 0);
    }

    #[test]
    fn overlay_manager() {
        let mut om = OverlayManager::new();
        om.open(PopupState::new(PopupKind::Hover, PopupAnchor::Cursor));
        om.open(PopupState::new(PopupKind::CompletionMenu, PopupAnchor::Cursor));
        assert_eq!(om.count(), 2);
        om.close_kind(PopupKind::Hover);
        assert_eq!(om.count(), 1);
    }

    #[test]
    fn popup_position_cursor() {
        let (r, c) = compute_popup_rect(PopupAnchor::Cursor, 5, 10, 5, 20, 24, 80);
        assert_eq!(r, 6);
        assert_eq!(c, 10);
    }

    #[test]
    fn popup_position_center() {
        let (r, c) = compute_popup_rect(PopupAnchor::Center, 0, 0, 10, 20, 24, 80);
        assert_eq!(r, 7);
        assert_eq!(c, 30);
    }

    #[test]
    fn popup_above_if_no_room() {
        let (r, _) = compute_popup_rect(PopupAnchor::Cursor, 22, 10, 5, 20, 24, 80);
        assert!(r < 22);
    }

    #[test]
    fn needs_scroll() {
        let mut p = PopupState::new(PopupKind::Wildmenu, PopupAnchor::CmdLine);
        p.max_visible = 5;
        p.show((0..3).map(|i| format!("{}", i)).collect());
        assert!(!p.needs_scroll());
        p.show((0..10).map(|i| format!("{}", i)).collect());
        assert!(p.needs_scroll());
    }
}
