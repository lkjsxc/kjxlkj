//! Overlay manager and popup positioning utilities.

use crate::popup_overlay::{PopupAnchor, PopupKind, PopupState};

/// Manages a stack of popup overlays.
#[derive(Debug, Clone, Default)]
pub struct OverlayManager {
    stack: Vec<PopupState>,
}

impl OverlayManager {
    pub fn open(&mut self, popup: PopupState) {
        self.stack.push(popup);
    }

    pub fn close_kind(&mut self, kind: PopupKind) {
        self.stack.retain(|p| p.kind != kind);
    }

    pub fn close_all(&mut self) {
        self.stack.clear();
    }

    pub fn top(&self) -> Option<&PopupState> {
        self.stack.last()
    }
}

/// Compute the rectangle `(x, y, w, h)` for a popup given an anchor and
/// screen/popup dimensions.
pub fn compute_popup_rect(
    anchor: PopupAnchor,
    screen_w: u16,
    screen_h: u16,
    popup_w: u16,
    popup_h: u16,
) -> (u16, u16, u16, u16) {
    let w = popup_w.min(screen_w);
    let h = popup_h.min(screen_h);
    match anchor {
        PopupAnchor::TopLeft => (0, 0, w, h),
        PopupAnchor::Center => {
            let x = screen_w.saturating_sub(w) / 2;
            let y = screen_h.saturating_sub(h) / 2;
            (x, y, w, h)
        }
        PopupAnchor::CmdLine => {
            let y = screen_h.saturating_sub(h).saturating_sub(1);
            (0, y, w, h)
        }
        PopupAnchor::Cursor => {
            let x = 1_u16.min(screen_w.saturating_sub(w));
            let y = 1_u16.min(screen_h.saturating_sub(h));
            (x, y, w, h)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn popup() -> PopupState {
        PopupState {
            kind: PopupKind::Completion,
            items: vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()],
            selected: 0,
            visible: true,
            max_visible: 3,
            scroll_offset: 0,
        }
    }

    #[test]
    fn overlay_manager() {
        let mut mgr = OverlayManager::default();
        mgr.open(popup());
        let mut h = popup();
        h.kind = PopupKind::Hover;
        mgr.open(h);
        assert_eq!(mgr.top().unwrap().kind, PopupKind::Hover);
        mgr.close_kind(PopupKind::Hover);
        assert_eq!(mgr.top().unwrap().kind, PopupKind::Completion);
        mgr.close_all();
        assert!(mgr.top().is_none());
    }

    #[test]
    fn popup_rects() {
        assert_eq!(
            compute_popup_rect(PopupAnchor::Center, 80, 24, 20, 10),
            (30, 7, 20, 10)
        );
        let (_, _, w, h) = compute_popup_rect(PopupAnchor::TopLeft, 10, 5, 20, 10);
        assert_eq!((w, h), (10, 5));
    }
}
