/// Terminal multiplexing — pane management, scroll regions, tmux integration.

/// Terminal pane identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaneId(pub u32);

/// Terminal pane state.
#[derive(Debug, Clone)]
pub struct TerminalPane {
    pub id: PaneId,
    pub title: String,
    pub lines: usize,
    pub cols: usize,
    pub scroll_top: usize,
    pub scroll_bottom: usize,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub alternate_screen: bool,
    pub running: bool,
}

impl TerminalPane {
    pub fn new(id: PaneId, lines: usize, cols: usize) -> Self {
        Self { id, title: String::new(), lines, cols,
            scroll_top: 0, scroll_bottom: lines.saturating_sub(1),
            cursor_line: 0, cursor_col: 0, alternate_screen: false, running: true }
    }
    pub fn resize(&mut self, lines: usize, cols: usize) {
        self.lines = lines; self.cols = cols;
        self.scroll_bottom = lines.saturating_sub(1);
    }
    pub fn set_scroll_region(&mut self, top: usize, bottom: usize) {
        self.scroll_top = top; self.scroll_bottom = bottom.min(self.lines.saturating_sub(1));
    }
}

/// Manager for multiple terminal panes.
#[derive(Debug, Default)]
pub struct PaneManager {
    panes: Vec<TerminalPane>,
    active: Option<PaneId>,
    next_id: u32,
}

impl PaneManager {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, lines: usize, cols: usize) -> PaneId {
        let id = PaneId(self.next_id);
        self.next_id += 1;
        self.panes.push(TerminalPane::new(id, lines, cols));
        if self.active.is_none() { self.active = Some(id); }
        id
    }

    pub fn close(&mut self, id: PaneId) {
        self.panes.retain(|p| p.id != id);
        if self.active == Some(id) {
            self.active = self.panes.first().map(|p| p.id);
        }
    }

    pub fn get(&self, id: PaneId) -> Option<&TerminalPane> { self.panes.iter().find(|p| p.id == id) }
    pub fn get_mut(&mut self, id: PaneId) -> Option<&mut TerminalPane> { self.panes.iter_mut().find(|p| p.id == id) }
    pub fn active_pane(&self) -> Option<&TerminalPane> { self.active.and_then(|id| self.get(id)) }
    pub fn set_active(&mut self, id: PaneId) { if self.get(id).is_some() { self.active = Some(id); } }
    pub fn count(&self) -> usize { self.panes.len() }
    pub fn list(&self) -> &[TerminalPane] { &self.panes }
}

/// Tmux-style key prefix state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmuxState { Normal, PrefixPending }

/// Tmux action after prefix key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmuxAction {
    SplitHorizontal, SplitVertical, NextPane, PrevPane,
    ClosePane, ZoomPane, Detach, CreateWindow,
}

pub fn map_tmux_key(ch: char) -> Option<TmuxAction> {
    match ch {
        '"' | '-' => Some(TmuxAction::SplitHorizontal),
        '%' | '|' => Some(TmuxAction::SplitVertical),
        'o' => Some(TmuxAction::NextPane),
        ';' => Some(TmuxAction::PrevPane),
        'x' => Some(TmuxAction::ClosePane),
        'z' => Some(TmuxAction::ZoomPane),
        'd' => Some(TmuxAction::Detach),
        'c' => Some(TmuxAction::CreateWindow),
        _ => None,
    }
}

/// Scrollback buffer for a terminal pane.
pub fn scrollback_capacity(pane_lines: usize) -> usize {
    (pane_lines * 100).min(50_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_get_pane() {
        let mut pm = PaneManager::new();
        let id = pm.create(24, 80);
        assert!(pm.get(id).is_some());
        assert_eq!(pm.count(), 1);
    }

    #[test]
    fn close_pane_switches_active() {
        let mut pm = PaneManager::new();
        let id1 = pm.create(24, 80);
        let id2 = pm.create(24, 80);
        pm.set_active(id1);
        pm.close(id1);
        assert_eq!(pm.active_pane().unwrap().id, id2);
    }

    #[test]
    fn pane_resize() {
        let mut pm = PaneManager::new();
        let id = pm.create(24, 80);
        pm.get_mut(id).unwrap().resize(40, 120);
        assert_eq!(pm.get(id).unwrap().lines, 40);
    }

    #[test]
    fn scroll_region() {
        let mut pane = TerminalPane::new(PaneId(0), 24, 80);
        pane.set_scroll_region(5, 20);
        assert_eq!(pane.scroll_top, 5);
        assert_eq!(pane.scroll_bottom, 20);
    }

    #[test]
    fn tmux_key_map() {
        assert_eq!(map_tmux_key('"'), Some(TmuxAction::SplitHorizontal));
        assert_eq!(map_tmux_key('%'), Some(TmuxAction::SplitVertical));
        assert_eq!(map_tmux_key('z'), Some(TmuxAction::ZoomPane));
        assert_eq!(map_tmux_key('q'), None);
    }

    #[test]
    fn scrollback_cap() {
        assert_eq!(scrollback_capacity(24), 2400);
        assert_eq!(scrollback_capacity(1000), 50_000);
    }

    #[test]
    fn active_pane_default() {
        let mut pm = PaneManager::new();
        assert!(pm.active_pane().is_none());
        pm.create(24, 80);
        assert!(pm.active_pane().is_some());
    }

    #[test]
    fn multiple_panes() {
        let mut pm = PaneManager::new();
        pm.create(24, 80);
        pm.create(24, 80);
        pm.create(24, 80);
        assert_eq!(pm.count(), 3);
    }
}
